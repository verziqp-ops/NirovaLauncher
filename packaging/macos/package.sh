#!/usr/bin/env bash
# Packages the macOS launcher: NIROVA.app (the binary, Info.plist, the icon), signed, then the
# two release assets, nirova-launcher-macos-ARCH.zip (what the launcher updates itself from;
# older launchers find the binary inside it) and nirova-launcher-macos-ARCH.dmg (the installer:
# the app and an Applications link over a background, to drag one onto the other). ARCH is the
# binary's: arm64 (Apple Silicon, the default) or x64 (Intel).
#
#   packaging/macos/package.sh BINARY VERSION OUTDIR [arm64|x64]
#
# Needs macOS (ditto, codesign, hdiutil) and dmgbuild (pip install dmgbuild==1.6.5; $DMGBUILD
# names it if it isn't on PATH). Signing: ad hoc by default, which needs no Apple account. With
# MACOS_SIGN_IDENTITY set to a "Developer ID Application: …" identity (in $MACOS_SIGN_KEYCHAIN
# when set), the app and the .dmg are signed with the hardened runtime and a timestamp, and with
# MACOS_NOTARY_PROFILE too, both are notarized and stapled (keychain.sh unlock sets the two;
# see packaging/macos/README.md).
set -euo pipefail
here=$(cd "$(dirname "$0")" && pwd)
binary=$1
version=$2
out=$(mkdir -p "$3" && cd "$3" && pwd)
arch=${4:-arm64}
case "$arch" in
  arm64) cpu=arm64 ;;
  x64) cpu=x86_64 ;;
  *) echo "unknown architecture $arch (arm64 or x64)" >&2; exit 1 ;;
esac
# The asset names say which Macs the binary runs on: it must be built for that processor.
[[ "$(lipo -archs "$binary")" == "$cpu" ]] || { echo "$binary is $(lipo -archs "$binary"), not $cpu" >&2; exit 1; }
zip=nirova-launcher-macos-$arch.zip
dmg=$out/nirova-launcher-macos-$arch.dmg
app=$out/NIROVA.app

rm -rf "$app"
mkdir -p "$app/Contents/MacOS" "$app/Contents/Resources"
cp "$binary" "$app/Contents/MacOS/nirova-launcher"
cp "$(dirname "$binary")/nirova-runtime.pack" "$app/Contents/Resources/nirova-runtime.pack"
cp "$here/../../LICENSE" "$here/../../THIRD_PARTY_NOTICES.md" "$app/Contents/Resources/"
chmod 755 "$app/Contents/MacOS/nirova-launcher"
# Build the macOS icon from NIROVA's own 512px app mark; no upstream Horizon icon is used.
iconset="$out/nirova.iconset"
mkdir -p "$iconset"
source_icon="$here/../icons/hicolor/512x512/apps/nirova.png"
for size in 16 32 64 128 256 512; do
  sips -z "$size" "$size" "$source_icon" --out "$iconset/icon_${size}x${size}.png" >/dev/null
done
for size in 16 32 128 256; do
  double=$((size * 2))
  sips -z "$double" "$double" "$source_icon" --out "$iconset/icon_${size}x${size}@2x.png" >/dev/null
done
sips -z 1024 1024 "$source_icon" --out "$iconset/icon_512x512@2x.png" >/dev/null
iconutil -c icns "$iconset" -o "$app/Contents/Resources/nirova.icns"
rm -rf "$iconset"
sed "s/@VERSION@/$version/g" "$here/Info.plist" > "$app/Contents/Info.plist"
printf 'APPL????' > "$app/Contents/PkgInfo"
plutil -lint "$app/Contents/Info.plist"

identity=${MACOS_SIGN_IDENTITY:--}
keychain=()
[[ -n "${MACOS_SIGN_KEYCHAIN:-}" ]] && keychain=(--keychain "$MACOS_SIGN_KEYCHAIN")
if [[ "$identity" == "-" ]]; then
  codesign --force --sign - "$app"
else
  codesign --force --options runtime --timestamp ${keychain[@]+"${keychain[@]}"} --sign "$identity" "$app"
fi
codesign --verify --strict --verbose=2 "$app"
# Still starts under the hardened runtime.
"$app/Contents/MacOS/nirova-launcher" --version
notarize=$([[ "$identity" != "-" && -n "${MACOS_NOTARY_PROFILE:-}" ]] && echo yes || true)
# The app first, so the zip and the .dmg both carry it stapled.
[[ -z "$notarize" ]] || "$here/notarize.sh" "$app"

rm -f "$out/$zip" "$dmg"
# No resource forks or extended attributes: they would land as ._ files inside the bundle.
(cd "$out" && ditto -c -k --norsrc --noextattr --noqtn --keepParent NIROVA.app "$zip")

cat > "$out/dmg.json" <<JSON
{
  "title": "NIROVA",
  "icon-size": 128,
  "background": "$here/background.png",
  "window": { "position": { "x": 200, "y": 160 }, "size": { "width": 660, "height": 420 } },
  "format": "UDZO",
  "contents": [
    { "x": 180, "y": 200, "type": "file", "path": "$app" },
    { "x": 480, "y": 200, "type": "link", "path": "/Applications" }
  ]
}
JSON
"${DMGBUILD:-dmgbuild}" -s "$out/dmg.json" NIROVA "$dmg"
rm "$out/dmg.json"
if [[ "$identity" != "-" ]]; then
  codesign --force --timestamp ${keychain[@]+"${keychain[@]}"} --sign "$identity" "$dmg"
  codesign --verify --strict --verbose=2 "$dmg"
fi
hdiutil verify "$dmg" >/dev/null
if [[ -n "$notarize" ]]; then
  "$here/notarize.sh" "$dmg"
  # What Gatekeeper says of a download: both must be "source=Notarized Developer ID".
  spctl --assess --verbose=2 "$app"
  spctl --assess --type open --context context:primary-signature --verbose=2 "$dmg"
fi
echo "Packaged $out/$zip and $dmg"
