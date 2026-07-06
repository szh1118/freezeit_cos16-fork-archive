#!/usr/bin/env bash
set -euo pipefail

root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
utils="$root/freezeitVS/include/utils.hpp"
freezeit="$root/freezeitVS/include/freezeit.hpp"
system_tools="$root/freezeitVS/include/systemTools.hpp"

fail=0

require_text() {
  local file=$1
  local text=$2
  local message=$3
  if ! grep -Fq "$text" "$file"; then
    echo "FAIL: $message" >&2
    echo "missing text: $text" >&2
    echo "file: $file" >&2
    fail=1
  fi
}

reject_text() {
  local file=$1
  local text=$2
  local message=$3
  if grep -Fq "$text" "$file"; then
    echo "FAIL: $message" >&2
    echo "unexpected text: $text" >&2
    echo "file: $file" >&2
    fail=1
  fi
}

require_text "$utils" 'const char* get_binary_path()' "Magisk detection must be centralized"
require_text "$utils" '"/product/bin/magisk"' "Magisk detection must cover product-mounted binaries"
require_text "$utils" '"/data/adb/magisk/magisk"' "Magisk detection must cover modern data/adb binaries"
require_text "$utils" '"/debug_ramdisk/magisk"' "Magisk detection must cover ramdisk binaries"
require_text "$utils" 'int get_version_code(const char* magiskPath)' "Magisk version lookup must use the detected binary"
reject_text "$utils" 'Utils::popenRead("/system/bin/magisk -V"' "Magisk version lookup must not be hard-coded to /system/bin"

require_text "$freezeit" 'const char* magiskPath = MAGISK::get_binary_path();' "Freezeit must probe Magisk before setting moduleEnv"
require_text "$freezeit" 'MAGISK::get_version_code(magiskPath)' "Freezeit must query the detected Magisk binary"

require_text "$system_tools" 'freezeit.moduleEnv.starts_with("Magisk")' "Magisk checks must tolerate appended version text"
require_text "$system_tools" 'freezeit.moduleEnv.starts_with("KernelSU")' "KernelSU checks must tolerate appended version text"
reject_text "$system_tools" 'freezeit.moduleEnv == "Magisk"' "Magisk checks must not use exact moduleEnv equality"
reject_text "$system_tools" 'freezeit.moduleEnv == "KernelSU"' "KernelSU checks must not use exact moduleEnv equality"

exit "$fail"
