#!/usr/bin/env sh
set -eu

SERIAL_ARG="${1:-}"
ADB="${ADB:-adb}"
PACKAGE_LIST="${PACKAGE_LIST:-}"
if [ -z "$PACKAGE_LIST" ]; then
    echo "usage: PACKAGE_LIST='pkg.one pkg.two pkg.three' $0 [serial]" >&2
    exit 2
fi

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

echo "freezeit freeze/unfreeze validation"
echo "timestamp=$(date '+%Y-%m-%d %H:%M:%S')"
echo "packages=$PACKAGE_LIST"

if [ "$(run_device "su -c 'ss -ltn 2>/dev/null | grep -q 60613 && echo reachable || echo unreachable'")" != "reachable" ]; then
    echo "daemon_socket=unreachable"
    exit 1
fi
echo "daemon_socket=reachable"

for package_name in $PACKAGE_LIST; do
    uid="$(run_device "cmd package list packages -U '$package_name' 2>/dev/null | sed -n 's/.* uid://p' | head -n 1")"
    echo "package=$package_name uid=${uid:-unknown}"
    run_device "pidof '$package_name' 2>/dev/null || true"
    run_device "am force-stop '$package_name' 2>/dev/null || true"
    run_device "monkey -p '$package_name' -c android.intent.category.LAUNCHER 1 >/dev/null 2>&1 || true"
    sleep 2
    echo "foreground_pid=$(run_device "pidof '$package_name' 2>/dev/null || true")"
    run_device "input keyevent KEYCODE_HOME"
    sleep 2
    echo "background_pid=$(run_device "pidof '$package_name' 2>/dev/null || true")"
done

echo "manual_check=confirm operation log contains freeze/unfreeze/postpone/skip decisions for each package"
