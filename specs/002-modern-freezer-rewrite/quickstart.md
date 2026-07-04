# Quickstart Validation: Modern Freezer Rewrite

This guide defines the validation path for implementation. It is not a manual for end users.

## Prerequisites

- Host has Android SDK platform-tools, Android NDK, Rust stable, `cargo-ndk`, Gradle/JDK compatible with `freezeitApp`.
- Target device is connected over ADB as `3B1F4LE5MS142WJY`.
- Target device has Magisk root and LSPosed enabled.
- LSPosed scope includes `system` and the manager package.

## Build Validation

1. Build Rust daemon for arm64 Android:

   ```bash
   cargo ndk -t arm64-v8a build --release
   ```

2. Run Rust tests:

   ```bash
   cargo test
   ```

3. Build manager APK:

   ```bash
   ./gradlew :app:assembleRelease
   ```

4. Package Magisk release zip using the existing release pipeline after tasks define the final script.

Expected outcome: daemon binary, manager APK, and Magisk zip are produced without warnings that affect release behavior.

## Device Baseline Check

Run read-only checks:

```bash
adb -s 3B1F4LE5MS142WJY shell getprop ro.build.version.release
adb -s 3B1F4LE5MS142WJY shell getprop ro.build.version.sdk
adb -s 3B1F4LE5MS142WJY shell getprop ro.build.fingerprint
adb -s 3B1F4LE5MS142WJY shell uname -a
adb -s 3B1F4LE5MS142WJY shell su -c id
adb -s 3B1F4LE5MS142WJY shell su -c 'find /sys/fs/cgroup/apps -name cgroup.freeze 2>/dev/null | head'
adb -s 3B1F4LE5MS142WJY shell dumpsys activity processes | head -120
```

Expected outcome: baseline matches the compatibility report, root context is Magisk, cgroup freezer paths exist, and ActivityManager freezer fields are visible.

## Install and Boot Validation

1. Install the Magisk zip through the chosen release/install path.
2. Reboot the target device.
3. Unlock once.
4. Open manager.

Expected outcome within 30 seconds after unlock:

- Manager can reach daemon.
- Hook readiness is active.
- Root readiness is active.
- Freezer capability is active.
- No manual shell command is required.

## Freeze/Unfreeze Validation

1. Select at least three third-party apps, including one multi-process app.
2. Configure freeze policy with normal delay.
3. Launch each app, return home, and wait for the configured delay.
4. Reopen each app.

Expected outcome:

- Freeze operations occur only after eligibility checks pass.
- Foreground launch triggers unfreeze before user-visible breakage.
- At least 95% of transitions meet the spec timing criteria.
- Logs contain package, UID, PID list, action, backend, result, and reason.

## Degraded-State Validation

Run these one at a time:

- Disable LSPosed scope for the manager/module, reboot, and open manager.
- Temporarily block hook bridge response during a test build.
- Simulate missing or corrupted config file.

Expected outcome:

- Manager reports degraded or inactive state clearly.
- Daemon does not run unsafe freeze/terminate actions.
- Logs explain the blocker and next action.

## Recovery Validation

1. Freeze a selected test app.
2. Restart daemon without reboot.
3. Query process state.
4. Reopen the app.

Expected outcome:

- Daemon reads current process/cgroup state before new actions.
- No stale frozen state remains after foreground launch.
- Recovery operation is logged.

## Release Validation

Before declaring implementation complete:

- Run build checks and tests.
- Install release zip on the target device.
- Reboot and complete install validation.
- Complete freeze/unfreeze validation.
- Inspect zip contents for daemon, APK, module metadata, scripts, baseline notes, and changelog.
- Run `/brooks-review`.
- Run `/speckit-converge`.
- Complete a 24-hour self-use soak with no boot loop, daemon crash, or manager crash attributable to the module.
