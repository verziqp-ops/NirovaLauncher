# NIVRORA region-threading architecture plan

Status: design proposal only. The game simulation remains single-threaded until each phase has implementation, differential tests, and measured results.

## Invariants

- A region has one simulation owner during a tick; no worker mutates another region's state directly.
- Within a region, entity AI, block entities, scheduled ticks, redstone, fluids and block updates retain a deterministic sequential order.
- Cross-region interactions are explicit messages, not shared mutable references.
- Save/load boundaries and crash recovery must never observe a half-applied region tick.
- Vanilla parity is a release gate, not an optional benchmark.

## Staged migration

### 0. Establish the baseline
Capture deterministic fixtures and event traces for redstone clocks, observers, pistons, comparators, hoppers, fluids, mob AI, collisions, projectiles, portals, chunk borders and save/reload. Compare against the current engine and the relevant vanilla reference.

### 1. Model ownership
Introduce region identifiers, ownership metadata, read-only snapshots and APIs that make cross-region writes impossible. Keep all simulation on the current main tick loop.

### 2. Extract region tick transactions
Make one region tick an explicit transaction with stable phases: inbound events, scheduled block work, block entities, entities, outbound events and commit. Do not reorder existing vanilla work within a region.

### 3. Message passing at boundaries
Use per-destination queues with deterministic ordering keys (simulation tick, source region, event sequence). Define when inbound events become visible. Never synchronously wait on a neighbour worker while holding region state.

### 4. Parallel region scheduler
Only after steps 0–3 pass, schedule independent regions on a bounded worker pool. A region must have exactly one writer at a time. Add back-pressure and deterministic fallback when a dependency cannot be resolved in the current tick.

### 5. Dimension scheduling
Do not simply give Overworld, Nether and End independent clocks. First map every cross-dimension dependency: portals, player transfer, global time/weather, scheduled tasks, sleep, server tick and persistence. Parallelize dimension work only where dependencies and vanilla timing are explicitly preserved.

## Required correctness tests

- Same seed and input trace produce identical block/entity state hashes in single-threaded and region-scheduled modes.
- Repeat tests with different worker counts and randomized worker scheduling.
- Boundary tests cover redstone, piston structures, fluids, entities, projectiles and chunk load/unload.
- Save/reload during heavy cross-region traffic yields the same state as uninterrupted execution.
- No deadlocks, data races, lost messages, duplicate events or unbounded queues.
- Benchmark tick time, p50/p95/p99 latency, CPU utilization and memory against the single-threaded baseline.

## Rollout

Keep the existing single-threaded scheduler as the reference and safe fallback. Enable region scheduling behind a developer feature flag first. Do not make it the default until parity suites pass on CI and long-running stress tests.
