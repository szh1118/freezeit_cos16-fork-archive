#!/usr/bin/env sh
set -eu

SERIAL_ARG="${1:-}"
ADB="${ADB:-adb}"
MODDIR="${MODDIR:-/data/adb/modules/freezeit}"
HOST="${FREEZEIT_HOST:-127.0.0.1}"
PORT="${FREEZEIT_PORT:-60613}"

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

echo "freezeit install boot validation"
echo "timestamp=$(date '+%Y-%m-%d %H:%M:%S')"
echo "boot_completed=$(run_device 'getprop sys.boot_completed')"
echo "module_dir=$MODDIR"

if [ "$(run_device "su -c 'test -d \"$MODDIR\" && echo present || echo missing'")" != "present" ]; then
    echo "module_dir=missing"
    exit 1
fi

if [ "$(run_device "su -c 'test -e \"$MODDIR/disable\" -o -e \"$MODDIR/remove\" && echo blocked || echo enabled'")" = "blocked" ]; then
    echo "module_state=disabled_or_remove_pending"
    exit 1
fi

if [ "$(run_device "su -c 'test -x \"$MODDIR/freezeit\" && echo executable || echo missing'")" = "executable" ]; then
    echo "daemon_binary=executable"
else
    echo "daemon_binary=missing_or_not_executable"
    exit 1
fi

if [ "$(run_device "command -v nc >/dev/null 2>&1 && echo present || echo missing")" = "present" ]; then
    if [ "$(run_device "nc -z '$HOST' '$PORT' >/dev/null 2>&1 && echo reachable || echo unreachable")" = "reachable" ]; then
        echo "daemon_socket=reachable"
    else
        echo "daemon_socket=unreachable"
        exit 1
    fi
else
    echo "daemon_socket=not_checked_nc_missing"
fi

if [ "$(run_device "su -c 'test -f \"$MODDIR/boot.log\" && echo present || echo missing'")" = "present" ]; then
    echo "boot_log=present"
    run_device "su -c 'tail -n 20 \"$MODDIR/boot.log\"'"
else
    echo "boot_log=missing"
fi
