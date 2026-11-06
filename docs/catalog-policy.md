# NIROVA Catalog Policy

The NIROVA catalog is a curated, repository-backed index of projects compatible with NIROVA.

## Current state

The catalog index is intentionally empty until a project has been reviewed and a release artifact has been published. An empty catalog is not evidence that the catalog service is fully operational.

## Submission requirements

A submission must include:

- A project ID using lowercase letters, digits and underscores.
- Project name, summary, description, authors, source repository and SPDX license.
- Project kind and supported NIROVA/game version.
- A release artifact with a stable download URL, exact byte size and lowercase SHA-256.
- A project manifest that matches the index entry.
- A minisign public key and signature for maintained executable/native projects. Unsigned files are explicitly marked as unsigned and are not treated as verified.

## Review and publishing

1. Submit a project through the repository's catalog issue form.
2. Maintainers review source ownership, license, provenance, manifest, capabilities and artifact.
3. A reviewed index entry is added to `catalog/index.json` through a pull request.
4. The artifact is published as a GitHub Release asset in this repository or another approved HTTPS release location.
5. The index pins the exact SHA-256 and size. The launcher verifies those values and the project manifest before installation.
6. Maintainers may add a revocation entry to disable a compromised or unsafe version.

## Security

- A catalog listing is not a guarantee that code is harmless.
- Native code runs with the user's operating-system permissions. The launcher must show a clear warning before the first native install.
- Never silently execute downloaded code.
- Never accept an artifact whose size, checksum, signature or manifest does not match the index.
- Report security issues privately to the repository maintainers rather than publishing exploit details.

## Compatibility

The current index schema is version 1. The launcher only offers project kinds it knows how to validate and install. Unsupported kinds remain unavailable rather than being installed speculatively.
