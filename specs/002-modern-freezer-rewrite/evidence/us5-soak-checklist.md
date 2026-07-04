# US5 24-Hour Self-Use Soak Checklist

Status: not started

Target: `3B1F4LE5MS142WJY` / OnePlus `CPH2653` / Android 16

## Start Conditions

- [ ] Release candidate zip installed from `freezeitRelease/`
- [ ] Device rebooted after install
- [ ] Manager opens after first unlock
- [ ] Daemon reachable on `127.0.0.1:60613`
- [ ] Hook readiness recorded as active or clearly degraded with control blocked
- [ ] Compatibility baseline captured

## During Soak

- [ ] Normal daily unlock/lock cycle observed
- [ ] At least three selected third-party apps exercised
- [ ] One multi-process app exercised
- [ ] Manager log inspected for operation identity, action, result, and reason
- [ ] Battery/network/wake-lock degraded paths checked if unavailable

## End Conditions

- [ ] No boot loop
- [ ] No daemon crash attributable to Freezeit
- [ ] No manager crash attributable to Freezeit
- [ ] No stale frozen app state after foreground launch
- [ ] Final `adb logcat -b crash -d` reviewed
- [ ] Final daemon PID/listener state recorded
- [ ] Observations copied into `us5-24h-soak.md`
