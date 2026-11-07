# Native Rust bootstrap

The complete launcher package contains `nirova-runtime.pack`, one native game archive for its platform. `launcher/runtime-manifest.json` pins the reviewed game source, original CI run, archive size and SHA-256. No Minecraft jars, assets or prepared resources are bundled.

CI downloads the source-pinned artifact and rejects changed bytes before packaging. When production Windows signing is configured, CI signs and verifies the two native game executables, repacks the archive and binds the exact final archive digest into the launcher at compile time. The launcher verifies that digest while copying into the selected installation folder, before unpacking anything. This is the same installed-launcher trust boundary; it is not a replacement for Authenticode or signed online update manifests.

Windows and Linux ship the pack next to the executable; macOS ships it in `Contents/Resources`. Signed self-updates retain a content-addressed pack so a failed binary replacement cannot invalidate the old launcher's pack. Existing online game downloads still require their existing signed checksums.

To refresh the bootstrap, build/review the native game in this repository, verify its platform artifacts, update all four manifest entries and the CI artifact run ID together, then rebuild the launcher. CI artifacts have limited retention: keep the pinned source artifacts available or refresh them before expiration. The installed package itself works offline and has no artifact-server dependency.

`Rust game installation folder` changes only native version/resource storage. Player saves and instance options remain in their existing game home. The installer never overwrites an unrelated incomplete version folder. Resource preparation still uses the player's own files or verified-owner downloads; choosing an empty game destination is not a proof of Minecraft ownership.
