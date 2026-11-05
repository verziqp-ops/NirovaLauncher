# Nirova release flow

Nirova owns its game and launcher release channels. Game packages are built from this repository's checkout; the game workflow does not download or unpack prebuilt archives from `horizon-rust/horizon`.

## Game channels

| Channel | Trigger | Release tag | Meaning |
| --- | --- | --- | --- |
| Stable | Push a version tag such as `v1.2.3` | `v1.2.3` | Stable game build |
| Preview | Run `.github/workflows/game.yml` with `workflow_dispatch` | `nightly` | Preview build; the launcher displays it as Preview |

The GitHub tag `nightly` is retained for compatibility with existing signed-update handling. It is Nirova's own channel: its build is compiled from this repository and its checksum list is signed for the separate `nightly` channel. Stable releases are signed for their own version tag. A signature from one channel cannot be replayed as the other.

## What the game workflow builds

`.github/workflows/game.yml` runs on relevant pull requests, on `v*` tags and when Preview is dispatched.

- Linux x86_64, Windows x64, macOS arm64 and macOS x64 game packages.
- Linux x86_64, macOS arm64 and macOS x64 dedicated-server packages.
- `horizon-client`, `horizon-prepare` and `horizon-server` are compiled from this checkout.
- Each game package includes the matching resource-preparer source, Java parity/exporter sources and `research-lock.json`. The launcher uses these to prepare resources from the player's own official Minecraft installation.
- No Mojang game jar or asset bundle is republished by the workflow. The user prepares resources locally from their own game installation.

Pull-request runs build and package artifacts but never publish a release. A release job signs every package with `tools/ci/sign_release.sh`, verifies the signature with `horizon-release-sign`, then publishes the assets and `SHA256SUMS` files.

## Launcher channel and compatibility

The launcher reads game releases from `verziqp-ops/NirovaLauncher`. New packages use `nirova-game-*` names; the updater also recognises the previous `horizon-*` asset names so older releases remain installable for rollback. The server installer reads the Nirova repository and uses the `nirova-server-*` packages.

Launcher self-updates are separate: `.github/workflows/launcher.yml` builds Windows, Linux and macOS packages for tags `launcher@X.Y.Z`. Its artifacts are named `nirova-launcher-*`.

## Validation and release status

A successful workflow run is required before a build can be described as a release. A pull-request build is not a published installer. Do not announce a version as playable until the platform packages have completed and a clean installation has been tested.

## Region simulation

The deterministic region executor in `horizon-threads::region` is infrastructure only. The live world tick remains sequential until region ownership, RNG ordering, cross-region effects and vanilla differential tests are implemented. See [regional multithreading](region-threading-ru.md).
