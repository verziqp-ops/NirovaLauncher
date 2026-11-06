#!/usr/bin/env bash
# Run locally as the repository owner. Never print or commit the private key.
set -euo pipefail
umask 077
repo=verziqp-ops/NirovaLauncher
root=$(cd "$(dirname "$0")/../.." && pwd)
command -v gh >/dev/null || { echo 'Install GitHub CLI first.' >&2; exit 1; }
command -v cargo >/dev/null || { echo 'Install Rust first.' >&2; exit 1; }
gh auth status >/dev/null
existing=$(gh secret list --repo "$repo" --json name --jq '.[].name')
if [[ "$existing" == *RELEASE_SIGNING_KEY* ]]; then
  echo 'A release signing secret already exists. Use the documented key-rotation process instead.' >&2
  exit 1
fi
work=$(mktemp -d)
trap 'rm -rf "$work"' EXIT
cd "$root"
# The private key travels only through this pipe. pipefail prevents installing a public key if
# generation or encrypted secret upload fails. No private key is written to disk or stdout.
cargo run --locked --quiet -p horizon-release-sign -- keygen --public-out "$work/public.txt" |
  gh secret set RELEASE_SIGNING_KEY --repo "$repo"
{
  echo '# NIROVA release verification key. Private key: GitHub Actions RELEASE_SIGNING_KEY.'
  cat "$work/public.txt"
} > crates/horizon-release-sign/release-keys.txt
printf '%s\n' 'Secret saved. Commit and push crates/horizon-release-sign/release-keys.txt before rebuilding releases.'
