#!/bin/sh
# Installs Nirova Launcher on Linux (x86-64) or macOS (Apple Silicon), for this user only.
#   curl -fsSL https://github.com/verziqp-ops/NirovaLauncher/releases/latest/download/install.sh | sh
# Linux: ~/.local/share/nirova-launcher, `nirova-launcher` in ~/.local/bin, and
# the launcher adds itself to the applications menu when it first starts.
# macOS: ~/Applications/Nirova Launcher/, `nirova-launcher` in ~/.local/bin.
set -eu

REPO="verziqp-ops/NirovaLauncher"
BASE="https://github.com/$REPO/releases/latest/download"

case "$(uname -s)" in
  Linux)
    ASSET="horizon-launcher-linux-x86_64.tar.gz"
    DIR="${XDG_DATA_HOME:-$HOME/.local/share}/nirova-launcher"
    ;;
  Darwin)
    ASSET="horizon-launcher-macos-arm64.zip"
    DIR="$HOME/Applications/Nirova Launcher"
    ;;
  *)
    echo "Unsupported system: $(uname -s). On Windows run install.ps1." >&2
    exit 1
    ;;
esac

echo "Nirova Launcher"
echo "  Downloading $ASSET"
TMP="$(mktemp -d)"
trap 'rm -rf "$TMP"' EXIT
curl -fL --progress-bar "$BASE/$ASSET" -o "$TMP/$ASSET"

mkdir -p "$DIR"
case "$ASSET" in
  *.tar.gz) tar -xzf "$TMP/$ASSET" -C "$TMP" ;;
  *.zip) (cd "$TMP" && unzip -q "$ASSET") ;;
esac
mv -f "$TMP/horizon-launcher" "$DIR/nirova-launcher"
chmod +x "$DIR/nirova-launcher"
# macOS asks before opening downloaded programs that aren't notarized.
if [ "$(uname -s)" = Darwin ]; then
  xattr -dr com.apple.quarantine "$DIR" 2>/dev/null || true
fi

mkdir -p "$HOME/.local/bin"
ln -sf "$DIR/nirova-launcher" "$HOME/.local/bin/nirova-launcher"
echo "  Installed in $DIR"
echo "  Start it with: nirova-launcher"
case ":$PATH:" in
  *":$HOME/.local/bin:"*) ;;
  *) echo "  (~/.local/bin is not on your PATH: add it, or run $DIR/nirova-launcher)" ;;
esac
echo "  Next: open the launcher, press Prepare (it uses your Minecraft 26.3), then Play."
