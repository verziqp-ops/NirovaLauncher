# Code signing policy

Status: **not enrolled / no production code-signing certificate configured**. Current NIROVA previews must not be described as signed. This policy prepares the project for a trusted signing provider; it is not a claim of approval, certification, endorsement or completed enrollment.

NIROVA signs only its own launcher, Windows installer, native Rust client and resource preparer built by GitHub-hosted workflows in https://github.com/verziqp-ops/NirovaLauncher. It does not sign user-selected programs, Minecraft jars/assets, Java mods or arbitrary external binaries. The launcher and game archives contain native binaries from this repository; preparation uses the player's own Minecraft files separately.

The repository owner (`verziqp-ops`) is responsible for choosing the provider, proving publisher identity, configuring signing roles, maintaining MFA and approving production requests as required by the provider. Contributions must be reviewed under the project's and provider's applicable policies before they are signed. No assertion is made here that enrollment, MFA verification or provider approval is already complete.

The project originated from an upstream fork. SignPath Foundation's additional rules for forks and prior public releases must be reviewed by the provider; eligibility is not assumed. If the project is approved for Foundation certificates, the public policy will identify the actual service and Foundation publisher, including its required acknowledgement. A production certificate with a different publisher requires that publisher's verified identity and permission.

The private signing key stays with the CA/signing service/HSM. No private key is committed or sent through chat. Public releases require valid, timestamped Windows Authenticode signatures from the configured publisher. Current integration covers launcher EXE, Setup EXE, native game EXE and preparer EXE; it does not currently sign Inno Setup's embedded uninstaller.

See [Windows signing setup and diagnostics](windows-security.md) for exact configuration and the distinction between Authenticode and NIROVA's Ed25519 update signatures.
