#!/usr/bin/env sh
set -eu

ROOT="$(CDPATH= cd -- "$(dirname -- "$0")/.." && pwd)"
MAGISK_DIR="$ROOT/freezeitVS/magisk"
OUT_DIR="$ROOT/freezeitRelease"
VERSION="$(sed -n 's/^version=//p' "$MAGISK_DIR/module.prop" | head -n 1)"
OUT_ZIP="$OUT_DIR/freezeit_${VERSION:-dev}.zip"

DAEMON="${DAEMON:-$ROOT/freezeitDaemon/target/aarch64-linux-android/release/freezeit}"
APK="${APK:-}"
if [ -z "$APK" ]; then
  APK="$(find "$ROOT/freezeitApp/app/build/outputs/apk/release" -maxdepth 1 -type f -name '*.apk' | sort | tail -n 1)"
fi

if [ ! -f "$DAEMON" ]; then
  echo "daemon artifact missing: $DAEMON" >&2
  exit 1
fi

if [ ! -f "$APK" ]; then
  echo "manager APK missing: $APK" >&2
  exit 1
fi

TMP="$(mktemp -d)"
trap 'rm -rf "$TMP"' EXIT

cp -R "$MAGISK_DIR"/. "$TMP"/
rm -f "$TMP"/freezeit "$TMP"/freezeitARM64 "$TMP"/freezeitX64 "$TMP"/freezeitRustARM64 "$TMP"/freezeitRustX64 "$TMP"/freezeit*.apk
cp "$DAEMON" "$TMP/freezeitRustARM64"
cp "$APK" "$TMP/freezeit-${VERSION:-dev}.apk"
chmod 755 "$TMP/freezeitRustARM64" "$TMP/service.sh" "$TMP/customize.sh"

mkdir -p "$OUT_DIR"
if command -v zip >/dev/null 2>&1; then
  (cd "$TMP" && zip -qr "$OUT_ZIP" .)
elif command -v bsdtar >/dev/null 2>&1; then
  (cd "$TMP" && find . -mindepth 1 -print | sed 's#^\./##' | bsdtar -a -cf "$OUT_ZIP" -T -)
else
  echo "zip creation requires zip or bsdtar" >&2
  exit 1
fi

"$ROOT/scripts/validate-release-zip.sh" "$OUT_ZIP"
echo "packaged release: $OUT_ZIP"
