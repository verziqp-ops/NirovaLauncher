#!/bin/sh
# Installs NIROVA Launcher on Linux (x86-64) or macOS (Apple Silicon), for this user only.
#   curl -fsSL https://github.com/verziqp-ops/NIROVALauncher/releases/latest/download/install.sh | sh
# Linux: ~/.local/share/nirova-launcher, `nirova-launcher` in ~/.local/bin, and
# the launcher adds itself to the applications menu when it first starts.
# macOS: ~/Applications/NIROVA Launcher/, `nirova-launcher` in ~/.local/bin.
set -eu

REPO="verziqp-ops/NIROVALauncher"
API="https://api.github.com/repos/$REPO/releases?per_page=100"

case "$(uname -s)" in
  Linux)
    ASSET="nirova-launcher-linux-x86_64.tar.gz"
    DIR="${XDG_DATA_HOME:-$HOME/.local/share}/nirova-launcher"
    ;;
  Darwin)
    ASSET="nirova-launcher-macos-arm64.zip"
    DIR="$HOME/Applications/NIROVA Launcher"
    ;;
  *)
    echo "Unsupported system: $(uname -s). On Windows run install.ps1." >&2
    exit 1
    ;;
esac

# Game releases and launcher releases share this repository. Select the newest published
# launcher@ tag explicitly instead of GitHub's repository-wide /releases/latest alias.
RELEASES_JSON="$(curl -fsSL "$API")"
TAG="$(printf '%s\n' "$RELEASES_JSON" | sed -nE 's/^[[:space:]]*"tag_name":[[:space:]]*"(launcher@[^"]+)".*/\1/p' | head -n 1)"
if [ -z "$TAG" ]; then
  echo "No published NIROVA Launcher release was found. Download it from GitHub Releases." >&2
  exit 1
fi
BASE="https://github.com/$REPO/releases/download/$TAG"

echo "NIROVA Launcher"
echo "  Downloading $ASSET ($TAG)"
TMP="$(mktemp -d)"
trap 'rm -rf "$TMP"' EXIT
curl -fL --progress-bar "$BASE/$ASSET" -o "$TMP/$ASSET"

mkdir -p "$DIR"
case "$ASSET" in
  *.tar.gz)
    tar -xzf "$TMP/$ASSET" -C "$TMP"
    install -m 755 "$TMP/nirova-launcher" "$DIR/nirova-launcher"
    BIN="$DIR/nirova-launcher"
    ;;
  *.zip)
    (cd "$TMP" && unzip -q "$ASSET")
    # macOS release ZIPs contain the complete NIROVA.app bundle, not a loose binary.
    test -x "$TMP/NIROVA.app/Contents/MacOS/nirova-launcher"
    rm -rf "$DIR/NIROVA.app"
    mv "$TMP/NIROVA.app" "$DIR/NIROVA.app"
    BIN="$DIR/NIROVA.app/Contents/MacOS/nirova-launcher"
    ;;
esac

mkdir -p "$HOME/.local/bin"
ln -sf "$BIN" "$HOME/.local/bin/nirova-launcher"
echo "  Installed in $DIR"
echo "  Start it with: nirova-launcher"
case ":$PATH:" in
  *":$HOME/.local/bin:"*) ;;
  *) echo "  (~/.local/bin is not on your PATH: add it, or run $DIR/nirova-launcher)" ;;
esac
echo "  Next: open the launcher, press Prepare (it uses your Minecraft 26.3), then Play."
