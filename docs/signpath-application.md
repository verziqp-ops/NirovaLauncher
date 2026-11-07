# NIROVA signing application — owner review draft

This is a draft, not a submitted or approved application. The owner must choose and verify the signing identity and accept the provider's terms personally.

- Project: NIROVA Launcher and native Rust game client/resource preparer.
- Repository: https://github.com/verziqp-ops/NirovaLauncher
- Source license: MIT, with upstream/third-party notices retained.
- Maintainer contact / legal identity: **owner to supply directly to the provider**.
- Public release/download URL: **owner to supply an actual public release in the form to be signed**. A CI artifact alone must not be represented as an approved production release.
- Origin: maintained fork of an upstream Rust Minecraft implementation. Request provider review of the fork-specific requirements; do not assert automatic eligibility.
- Intended files: `nirova-launcher.exe`, `NIROVA-Setup.exe`, `horizon-client.exe`, `horizon-prepare.exe`, built from this repository on GitHub-hosted runners.
- Exclusions: Minecraft jars/assets, user-supplied executables, third-party Java mods. Proprietary game resources are prepared separately from the player's own files, not shipped for signing.
- Build integration: `.github/workflows/launcher.yml`, `.github/workflows/game.yml`; constrained artifact configurations in `packaging/signpath/`.
- Signing policy and present status: `docs/code-signing-policy.md`.
- MFA, approved branches, reviewers/approvers, privacy disclosures and release availability: **owner/provider to verify**, not claimed complete by this draft.

If using Foundation certificates, acknowledge the actual Foundation publisher after approval. If requiring the owner's legal identity/company as publisher, obtain a matching CA-issued certificate and configure that actual subject. Never invent a certificate identity or present a self-signed certificate as public trust.

Apply/terms: https://signpath.org/apply and https://signpath.org/terms .
