# Nirova Launcher

A community launcher for the Rust-based Minecraft 26.3 client. This repository is in active development; public release builds are not available yet.

## Current target

- One supported game version: **26.3**.
- Windows, Linux and macOS build pipelines.
- Local game-file discovery and manual selection.
- Guest/offline profile support.
- Microsoft account sign-in where configured.
- Ely.by integration is planned; it is not yet available in this build.
- Local skin and cape PNG selection is available for offline profiles; remote skin-provider integration is still being developed.

## Before the first public release

The launcher needs a compatible local copy of the game's 26.3 files to prepare resources. It does not bundle Minecraft assets or promise compatibility with other game versions. Guest mode does not grant access to servers that require Microsoft account ownership checks.

## Downloads

Downloads will appear under [Releases](https://github.com/verziqp-ops/NirovaLauncher/releases) after the first successful, tested release. Do not install unofficial binaries from third-party mirrors.

## Build from source

See the workflow files in the .github/workflows directory for supported CI targets and required build dependencies.

## Security and privacy

- Never share your Microsoft or Ely.by password with anyone.
- Account tokens must be stored using the operating system's secure credential store.
- Report security issues privately to the repository owner rather than publishing exploit details.

## Licensing

This project includes third-party components. Their original copyright notices and license terms are retained in LICENSE and THIRD_PARTY_NOTICES.md.