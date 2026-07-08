#!/usr/bin/env sh
set -eu

repo_root=$(CDPATH= cd -- "$(dirname -- "$0")/.." && pwd)
tmp_dir=$(mktemp -d)
trap 'rm -rf "$tmp_dir"' EXIT

cat >"$tmp_dir/large_writer.cpp" <<'CPP'
#include <string.h>
#include <unistd.h>

int main() {
    char block[4096];
    memset(block, 'x', sizeof(block));

    for (int i = 0; i < 512; ++i) {
        if (write(STDOUT_FILENO, block, sizeof(block)) < 0)
            return 1;
    }

    return 0;
}
CPP

cat >"$tmp_dir/vpopen_drain_test.cpp" <<'CPP'
#include <assert.h>
#include <errno.h>
#include <string.h>
#include <sys/wait.h>

#include "vpopen.hpp"

int main(int argc, char** argv) {
    assert(argc == 2);

    char buf[16] = {};
    const char* cmd[] = { argv[1], nullptr };
    VPOPEN::vpopen(argv[1], cmd, buf, sizeof(buf));

    assert(strlen(buf) == sizeof(buf) - 1);
    for (size_t i = 0; i < sizeof(buf) - 1; ++i)
        assert(buf[i] == 'x');

    return 0;
}
CPP

compiler=${CXX:-c++}
"$compiler" -std=c++17 "$tmp_dir/large_writer.cpp" -o "$tmp_dir/large_writer"
"$compiler" -std=c++17 -I "$repo_root/freezeitVS/include" \
    "$tmp_dir/vpopen_drain_test.cpp" -o "$tmp_dir/vpopen_drain_test"

if "$tmp_dir/vpopen_drain_test" "$tmp_dir/large_writer" 2>"$tmp_dir/stderr.txt"; then
    if [ -s "$tmp_dir/stderr.txt" ]; then
        cat "$tmp_dir/stderr.txt" >&2
        exit 1
    fi
else
    cat "$tmp_dir/stderr.txt" >&2
    exit 1
fi
