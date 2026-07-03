#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
NDK_PATH="${NDK_PATH:-/home/admin/Android/Sdk/ndk/28.2.13676358}"
TOOLCHAIN="${NDK_PATH}/toolchains/llvm/prebuilt/linux-x86_64"
CLANG="${TOOLCHAIN}/bin/aarch64-linux-android35-clang++"
OUT="${ROOT_DIR}/magisk/freezeitARM64"

if [[ ! -x "${CLANG}" ]]; then
  echo "Missing ARM64 compiler: ${CLANG}" >&2
  exit 1
fi

cd "${ROOT_DIR}"

"${CLANG}" \
  -std=c++20 \
  -static \
  -s \
  -Ofast \
  -Wall \
  -Wextra \
  -Wshadow \
  -fno-exceptions \
  -fno-rtti \
  -DNDEBUG \
  -fPIE \
  -Iinclude \
  src/main.cpp \
  -o "${OUT}"

chmod 0755 "${OUT}"
echo "Built ${OUT}"
