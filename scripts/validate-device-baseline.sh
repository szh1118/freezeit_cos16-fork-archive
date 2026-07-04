#!/usr/bin/env sh
set -eu

SERIAL_ARG="${1:-}"
ADB="${ADB:-adb}"

if [ -n "$SERIAL_ARG" ] && command -v "$ADB" >/dev/null 2>&1; then
    ADB_CMD="$ADB -s $SERIAL_ARG shell"
elif command -v getprop >/dev/null 2>&1; then
    ADB_CMD=""
else
    ADB_CMD="$ADB shell"
fi

run_device() {
    if [ -n "$ADB_CMD" ]; then
        # shellcheck disable=SC2086
        $ADB_CMD "$1"
    else
        sh -c "$1"
    fi
}

echo "freezeit baseline validation"
echo "timestamp=$(date '+%Y-%m-%d %H:%M:%S')"
echo "device=$(run_device 'getprop ro.product.model')"
echo "manufacturer=$(run_device 'getprop ro.product.manufacturer')"
echo "android_release=$(run_device 'getprop ro.build.version.release')"
echo "sdk=$(run_device 'getprop ro.build.version.sdk')"
echo "fingerprint=$(run_device 'getprop ro.build.fingerprint')"
echo "incremental=$(run_device 'getprop ro.build.version.incremental')"
echo "kernel=$(run_device 'uname -r')"
echo "arch=$(run_device 'uname -m')"
echo "magisk_context=$(run_device 'su -c id -Z 2>/dev/null || id -Z 2>/dev/null || true')"

if [ "$(run_device 'su -c "test -d /data/adb/modules/freezeit && echo present || echo missing"')" = "present" ]; then
    echo "module_dir=present"
else
    echo "module_dir=missing"
fi

if [ "$(run_device 'test -e /dev/binder -o -e /dev/binderfs/binder && echo present || echo missing')" = "present" ]; then
    echo "binder_device=present"
else
    echo "binder_device=missing"
fi

if [ "$(run_device 'test -e /sys/fs/cgroup/cgroup.controllers && echo present || echo missing')" = "present" ]; then
    echo "cgroup_v2=present"
else
    echo "cgroup_v2=missing"
fi
