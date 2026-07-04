#!/usr/bin/env sh
set -eu

zip_path="${1:-}"
if [ -z "$zip_path" ]; then
    echo "usage: $0 <magisk-zip>" >&2
    exit 2
fi

if [ ! -f "$zip_path" ]; then
    echo "missing zip: $zip_path" >&2
    exit 1
fi

tmp_list="$(mktemp)"
unzip -l "$zip_path" >"$tmp_list"

require_entry() {
    entry="$1"
    if ! grep -q "[[:space:]]$entry$" "$tmp_list"; then
        echo "missing archive entry: $entry" >&2
        rm -f "$tmp_list"
        exit 1
    fi
}

require_entry "module.prop"
require_entry "customize.sh"
require_entry "service.sh"
require_entry "freezeit.apk"

if ! grep -Eq "[[:space:]]freezeit((Rust)?(ARM64|X64))?$" "$tmp_list"; then
    echo "missing daemon binary entry" >&2
    rm -f "$tmp_list"
    exit 1
fi

rm -f "$tmp_list"
echo "magisk zip integrity: pass"
