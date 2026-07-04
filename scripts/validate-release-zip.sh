#!/usr/bin/env sh
set -eu

ZIP_PATH="${1:-}"
if [ -z "$ZIP_PATH" ]; then
  echo "usage: $0 <release.zip>" >&2
  exit 2
fi

if [ ! -f "$ZIP_PATH" ]; then
  echo "release zip not found: $ZIP_PATH" >&2
  exit 1
fi

LIST="$(unzip -Z1 "$ZIP_PATH" | sed 's#^\./##')"

require_entry() {
  entry="$1"
  echo "$LIST" | grep -Fx "$entry" >/dev/null || {
    echo "missing zip entry: $entry" >&2
    exit 1
  }
}

require_entry module.prop
require_entry service.sh
require_entry customize.sh
require_entry rom_baseline.prop
require_entry changelog.txt

echo "$LIST" | grep -E '^freezeitRustARM64$|^freezeitARM64$|^freezeit$' >/dev/null || {
  echo "missing zip entry: daemon payload" >&2
  exit 1
}

echo "$LIST" | grep -E '^freezeit.*\.apk$' >/dev/null || {
  echo "missing zip entry: freezeit*.apk" >&2
  exit 1
}

echo "release zip integrity: pass"
