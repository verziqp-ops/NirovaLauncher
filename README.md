# Nirova Launcher

Nirova is a community launcher for the Rust-based Horizon/Minecraft 26.3 client. The project is in active development; **there is no tested public installer release yet**.

## Project status

See **[Project status and roadmap (Ukrainian)](docs/STATUS_UK.md)** for what is already in the repository, what needs testing, and the remaining performance, design and release work.

## Current target
- One declared game target: **26.3**. Other versions are not currently advertised as supported.
- Launcher build workflows are configured for Windows, Linux, macOS Apple Silicon and Intel macOS.
- Local compatible game-file discovery and manual selection.
- Local offline profiles and Microsoft sign-in in configured flows.
- Local PNG skin and cape selection for offline profiles.
- Ely.by integration is planned, not available in this build.

## Downloads

**No public binary release is available yet.** Tested installers and platform archives will appear on the [GitHub Releases page](https://github.com/verziqp-ops/NirovaLauncher/releases).

The launcher does not bundle Minecraft assets or jars. A compatible local copy of the required 26.3 game files is needed. Do not download unofficial builds from third-party mirrors.

## Modding

A Ukrainian beginner guide is available in [docs/modding-uk.md](docs/modding-uk.md). The visual in-launcher mod creator is planned, not implemented. Java Forge/Fabric/NeoForge mods are not automatically compatible with the native Rust engine.

## Build from source

See [launcher CI](.github/workflows/launcher.yml) for launcher build targets and [game CI](.github/workflows/game.yml) for the separate game-client packaging workflow.

## Security and privacy
- Never share Microsoft or Ely.by passwords.
- Account tokens should be stored using the operating system's secure credential store.
- Offline profiles do not grant Minecraft Java Edition ownership or server entitlements.

## Licensing

This project includes third-party components. Their original copyright notices and license terms are retained in LICENSE and THIRD_PARTY_NOTICES.md.
