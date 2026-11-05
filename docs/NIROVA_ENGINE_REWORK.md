# Nirova / Horizon engine modernization plan

This document records the staged performance and compatibility work. It is a plan, not a claim that the engine changes below are already implemented.

## Non-negotiable compatibility rule

Performance work must preserve the existing simulation contract: fixed game-tick cadence, deterministic ordering of state mutations, redstone scheduled-tick order and delay semantics, entity/AI decisions, collision and physics outcomes, and save/load behavior. Do not parallelize mutable world access speculatively. Establish golden tests and replay traces before replacing a subsystem.

## Existing architecture found during the initial audit

- Single-player already runs its integrated server on a dedicated server thread by default (`crates/horizon-client/src/game/integrated.rs`). The in-frame server path is documented as deprecated.
- Chunk/mesh streaming already has worker infrastructure (`horizon_client::streaming::{Streaming, WorkerContext}`).
- The launcher already has a custom palette in `launcher/src/theme.rs`; this branch refreshes it to blue-charcoal / milky-blue with a pink accent.
- The game is Rust-based and expects local compatible game files; this repository does not bundle Minecraft assets.
- The repository license is MIT for this repository's code. It does not grant rights to Minecraft game code, assets, trademarks, or distribution.

## Work packages

### 0. Baseline, profiling and compatibility harness
1. Record the exact base commit and build/test commands for Windows, Linux and macOS.
2. Capture frame-time, tick-time, CPU worker utilization, allocations, peak resident memory, chunk generation latency and save latency.
3. Add deterministic replay fixtures for redstone circuits, piston timing, observers, repeaters/comparators, hopper/item movement, mob AI, collisions and dimension travel.
4. Compare state hashes and event traces at fixed tick boundaries. Any intentional divergence must be documented and approved.

### 1. Keep simulation and presentation decoupled
1. Preserve the integrated server thread as the owner of mutable authoritative world state.
2. Publish immutable, versioned render snapshots through a bounded latest-value channel; never make the renderer lock the world for a full frame.
3. Keep rendering on the window/render thread and let it draw the newest available snapshot even when the simulation tick is late.
4. Interpolate render-only transforms between snapshots; do not feed interpolated values back into physics or gameplay.
5. Define behavior for stale snapshots, pause, resize, shutdown, split-screen, multiplayer and world transitions.
6. Measure frame pacing and ensure bounded queues cannot accumulate unbounded stale snapshots.

### 2. Parallelize safe simulation work
1. Profile `game/creatures.rs` and identify read-only or independent calculations (path-cost fields, visibility candidates, broad-phase queries, steering proposals).
2. Run those calculations on immutable tick snapshots using a bounded worker pool.
3. Return proposals tagged with entity ID and source tick; apply them on the authoritative server thread in stable entity/order sequence.
4. Keep interactions with shared blocks, inventories, targets, collisions and other entities serialized unless a proven deterministic conflict-resolution scheme exists.
5. Parallelize physics broad-phase / candidate generation first; retain authoritative collision resolution and world mutation in a deterministic phase.
6. Do not change AI update cadence, random-number consumption order, target selection tie-breaks or navigation semantics without golden-test approval.

### 3. Redstone and scheduled world events
1. Inventory all scheduled block/fluid/entity tick queues and their ordering rules.
2. Preserve tick deadlines, priority, insertion ordering and cross-chunk behavior.
3. Parallelize only independent read/compute stages; commit state changes through deterministic ordered batches.
4. Add regression fixtures for clocks, quasi-connectivity/edge cases where applicable, pistons, observers, repeaters, comparators, hoppers, TNT and chunk-boundary circuits.
5. Do not ship a scheduler rewrite until replay traces match the baseline.

### 4. Memory and stutter reduction
1. Profile allocation hotspots before changing allocators or memory policy.
2. Reuse scratch buffers and arenas for per-tick/per-frame temporary data where lifetimes are clear.
3. Reduce cloning and serialization copies; use immutable shared data for assets and chunk snapshots.
4. Bound worker queues, caches, mesh upload queues and retained snapshots.
5. Evict inactive chunk/render data according to explicit budgets and ownership rules.
6. Track p50/p95/p99 frame time, tick time, allocation rate and RSS; avoid claims of lower RAM or fewer stutters without measurements.
7. Rust has no Java-style stop-the-world tracing GC by default; focus on allocator contention, allocation churn, locks, page faults and unbounded retention.

### 5. Chunk, file and save pipeline
1. Keep disk/network I/O asynchronous and off the render thread.
2. Bound concurrency and apply backpressure; prioritize chunks near active players.
3. Use safe snapshot/commit semantics for saves and backups so background work cannot stall world mutation.
4. Verify cancellation, shutdown, error recovery and atomic save behavior.

### 6. Dimensions
Give Overworld, Nether and End independent scheduling budgets and queues where appropriate, while retaining the required shared global tick semantics and correct cross-dimension travel. A slow dimension must not monopolize the worker pool or block rendering.

### 7. Rust mod ecosystem (later milestone)
Java Forge/Fabric/NeoForge mods are not binary-compatible with a Rust engine. Do not promise automatic support. After the engine API stabilizes, provide a versioned Rust SDK, examples, packaging/validation tools and a documented sandbox boundary. Evaluate WASM as an optional language-neutral target. Keep content-only packs distinct from executable native mods.

### 8. Launcher and visual design
- Dark: blue-charcoal surfaces, soft blue neutrals, pink accent.
- Light: milky white and pale blue surfaces, pink accent.
- Keep contrast accessible; verify focus rings, disabled states, errors and buttons in both appearances.
- Regenerate shared design tokens/CSS from the theme source where required; do not hand-edit generated tokens.
- Add screenshot/UI tests for both appearances and verify all launcher pages use the same palette.

## Acceptance gates

A work package is complete only when:
- the relevant tests pass on supported targets;
- replay traces preserve baseline gameplay semantics;
- no unbounded queue or new main-thread blocking is introduced;
- before/after performance data is attached (frame-time percentiles, tick-time percentiles, memory and workload);
- failure, cancellation and shutdown paths are tested.

## Immediate next implementation target

Build the immutable render-snapshot boundary and frame pacing instrumentation around the existing integrated server thread. This is safer and more foundational than moving arbitrary mutable AI/physics code into worker threads.
