# Nirova Launcher

**An independent Rust launcher and experimental Minecraft Java Edition client project.**

[![Latest CI preview](https://img.shields.io/badge/preview-GitHub_Actions-blue?logo=github)](https://github.com/verziqp-ops/NirovaLauncher/actions/runs/42054977083)
[![Releases](https://img.shields.io/github/v/release/verziqp-ops/NirovaLauncher?include_prereleases)](https://github.com/verziqp-ops/NirovaLauncher/releases)

## Download

**Preview builds are available now** from the [latest GitHub Actions run](https://github.com/verziqp-ops/NirovaLauncher/actions/runs/42054977083). Open the run, scroll to **Artifacts**, and download the platform package. GitHub may ask you to sign in. Extract the downloaded ZIP before running its contents.

| Platform | Artifact | Status |
|---|---|---|
| Windows x64 | `Nirova-Setup.exe` | Preview installer |
| Windows x64 | `nirova-launcher-windows-x64.zip` | Preview portable archive |
| Linux x86_64 | `nirova-launcher-linux-x86_64.tar.gz` | Preview archive |
| macOS | — | No successful package published yet |

These are temporary CI artifacts, not permanent releases. GitHub currently expires them after 90 days. They may contain bugs and are **not verified as a complete playable Minecraft experience**. Do not use them with important worlds or accounts.

Permanent downloads will appear on the [GitHub Releases page](https://github.com/verziqp-ops/NirovaLauncher/releases) after the release checks are complete. If that page has no release, there is no stable release yet.

## Project status

- **Target:** Minecraft Java Edition 26.3.
- **Launcher:** Rust desktop application; Windows and Linux preview packages have been built by CI.
- **Accounts:** Microsoft sign-in where configured; guest/offline profiles do not bypass server ownership requirements.
- **Mod loaders:** Fabric and Quilt support is under development.
- **Skins and capes:** local PNG selection is available for offline profiles; remote provider integration is in progress.
- **Ely.by:** planned, not available in this preview.
- **Compatibility:** a version selector and version-porting tools exist; this does not mean every Minecraft version is fully implemented or tested.

## Requirements and game files

Nirova does not bundle Minecraft assets. Provide game files from a copy you are entitled to use. A compatible local Minecraft Java Edition 26.3 installation and suitable Java runtime may be required. See [packaging/PLAY.txt](packaging/PLAY.txt).

## Build from source

See [the launcher CI workflow](.github/workflows/launcher.yml) for supported targets and build steps. Rust/Cargo and platform-specific native dependencies are required.

## Security

Never share account passwords or access tokens. Remove secrets and personal data from logs before reporting issues. Report security vulnerabilities privately to the repository owner.

## Licensing and trademarks

See [LICENSE](LICENSE) and [THIRD_PARTY_NOTICES.md](THIRD_PARTY_NOTICES.md). Minecraft is a trademark of Microsoft/Mojang. Nirova is an independent community project and is not affiliated with or endorsed by Microsoft or Mojang. Minecraft assets are not included.
