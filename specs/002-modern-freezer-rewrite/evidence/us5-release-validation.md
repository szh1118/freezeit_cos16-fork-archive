# US5 Release Validation Evidence

Date: 2026-07-04

Scope: T083 release install, reboot, control, restore, and archive integrity.

Status: pass for T083. Release archive, install, reboot, daemon readiness,
manager version, active hook readiness, app control, app restore, and archive
integrity are evidenced on the target device.

## Archive Integrity

```text
rtk sh scripts/validate-release-zip.sh freezeitRelease/freezeit_3.2.0SelfUse.zip
result: pass
observed: release zip integrity: pass
```

Release candidate artifacts:

- `freezeitRelease/freezeit_3.2.0SelfUse.zip` (1.8M)
- `freezeitRelease/freezeit_3.2.0SelfUse_source.tar.gz` (8.6M)

## Install Attempt 1

Initial package attempt was rejected by Magisk:

```text
! This zip is not a Magisk module!
```

Root cause: the `bsdtar` packaging path stored entries with a `./` prefix, so
Magisk did not detect `META-INF/com/google/android/update-binary` at the exact
path it expects. `scripts/package-release.sh` now writes entries without that
prefix.

## Install And Reboot

```text
rtk adb -s 3B1F4LE5MS142WJY push freezeitRelease/freezeit_3.2.0SelfUse.zip /sdcard/Download/freezeit_3.2.0SelfUse.zip
rtk adb -s 3B1F4LE5MS142WJY shell 'su -c "magisk --install-module /sdcard/Download/freezeit_3.2.0SelfUse.zip"'
result: pass
observed:
- 正在安装 3.2.0SelfUse
- 冻它APP 安装成功
- 安装完毕, 重启生效
- Done
```

```text
rtk adb -s 3B1F4LE5MS142WJY reboot
rtk adb -s 3B1F4LE5MS142WJY wait-for-device
boot_completed polling
result: pass
observed: boot_completed=1
```

## Post-Reboot State

```text
rtk sh scripts/validate-install-boot.sh 3B1F4LE5MS142WJY
result: pass
observed:
- boot_completed=1
- daemon_binary=executable
- daemon_socket=reachable
- boot_log=present
```

```text
module.prop:
version=3.2.0SelfUse
versionCode=302000

daemon:
/data/adb/modules/freezeit/freezeit
size: 508688
pid: 3477
listener: 127.0.0.1:60613

manager:
versionCode=302000
versionName=3.2.0SelfUse
```

Manager UI hierarchy after launch:

```text
statusText="Daemon degraded / Hook unknown\n{\"status\":\"degraded\"}"
module_env="Magisk"
module_ver="0.1.0-rust (1)"
manager_ver="3.2.0SelfUse (302000)"
```

Crash buffer after reboot contains only init/crash_dump-helper entries and no
Freezeit Java or native process crash attribution in the captured tail.

## Open Blocker

T083 also requires release control and restore validation. The current manager
state is degraded with hook unknown, so this evidence does not prove safe app
control, app restore, or active release readiness. Do not check T083 until
control/restore validation is complete or explicitly revised.

## Active Release Readiness Refresh

Timestamp: `2026-07-04 01:40:55 Asia/Hong_Kong`

Final release candidate:

- `freezeitRelease/freezeit_3.2.0SelfUse.zip`
- `freezeitRelease/freezeit_3.2.0SelfUse_source.tar.gz`

After rebuilding the Rust daemon and manager APK, `scripts/package-release.sh`
reported:

```text
release zip integrity: pass
packaged release: `freezeitRelease/freezeit_3.2.0SelfUse.zip`
```

Target install/reboot evidence:

```text
magisk --install-module /sdcard/Download/freezeit_3.2.0SelfUse.zip
result: pass
observed:
- 正在安装 3.2.0SelfUse
- 冻它APP 安装成功
- 安装完毕, 重启生效
- Done
```

```text
rtk sh scripts/validate-install-boot.sh 3B1F4LE5MS142WJY
result: pass
observed:
- boot_completed=1
- daemon_binary=executable
- daemon_socket=reachable
- boot_log=present
```

Control-readiness socket evidence:

```text
manager SetAppCfg over 127.0.0.1:60613
result: success
```

```text
manager GetXpLog over 127.0.0.1:60613
{"status":"active","system_server_ready":true,"config_ready":true,"screen_ready":true,"wakelock_ready":true,"network_ready":true}
```

```text
manager GetHealthReport over 127.0.0.1:60613
{"status":"active","daemonReady":true,"hookHealth":"active"}
```

Manager UI hierarchy evidence:

```text
statusText="Daemon active / Hook active\n{\"status\":\"active\",\"daemonReady\":true,\"hookHealth\":\"active\"}"
manager_ver="3.2.0SelfUse (302000)"
```

Release readiness is active. Concrete app control and restore validation were
completed during the T052 cgroup control run and final cleanup.

## Final Control And Restore Refresh

Final release candidate after T052 fixes:

```text
freezeitRelease/freezeit_3.2.0SelfUse.zip
freezeitRelease/freezeit_3.2.0SelfUse_source.tar.gz
```

Archive integrity:

```text
rtk sh scripts/validate-release-zip.sh freezeitRelease/freezeit_3.2.0SelfUse.zip
result: pass
observed: release zip integrity: pass
```

Final package install and reboot:

```text
adb push freezeitRelease/freezeit_3.2.0SelfUse.zip /sdcard/Download/freezeit_3.2.0SelfUse.zip
adb shell "su -c 'magisk --install-module /sdcard/Download/freezeit_3.2.0SelfUse.zip'"
adb reboot
result: pass
observed:
- 正在安装 3.2.0SelfUse
- 冻它APP 安装成功
- 安装完毕, 重启生效
- Done
```

Final post-reboot daemon state:

```text
LISTEN 127.0.0.1:60613 users:(("freezeit",pid=3501,fd=3))
SetAppCfgEmpty=success
GetAppCfgLen=0
health={"status":"active","daemonReady":true,"hookHealth":"active"}
self_check={"controlAllowed":true}
stopped process check: no output
```

Control validation is recorded in
`specs/002-modern-freezer-rewrite/evidence/us2-device-freeze.md`:

```text
freeze_passes=4/4
unfreeze_passes=4/4
backend=cgroup.freeze
```

Restore validation:

```text
SetAppCfgEmpty=success
GetAppCfgLen=0
known cgroup.freeze values checked before final reboot: 0 0 0 0 0
```

This satisfies SC-008 for the release candidate: packaged module installs,
reboots, activates, controls selected test apps, restores them, and passes
archive integrity checks.

## Scoped Brooks Review

Mode: PR Review

Scope: T083 release package/install evidence and package-script correction.

Finding summary: no unresolved code finding remains for the Magisk module zip
structure, daemon hook-health bridge, config forwarding, dynamic manager health
refresh, or manager status rendering in the scope validated here.

Verification refreshed after T052:

```text
rtk sh -lc '. "$HOME/.cargo/env" &&
  cargo fmt --manifest-path freezeitDaemon/Cargo.toml &&
  cargo test --manifest-path freezeitDaemon/Cargo.toml --test freeze_unfreeze_state live_control_pass_treats_foreground_query_failure_as_fail_closed &&
  sh freezeitDaemon/scripts/test-host.sh &&
  sh freezeitDaemon/scripts/build-android.sh &&
  sh scripts/package-release.sh'
result: pass, 55 Rust contract/integration tests; Android daemon build pass;
release zip integrity pass
```

```text
rtk sh -lc 'cd freezeitApp &&
  ANDROID_HOME=/home/admin/Android/Sdk JAVA_HOME=/usr/lib/jvm/java-17-openjdk
  sh ./gradlew :app:assembleRelease'
result: pass, BUILD SUCCESSFUL
```

## Scoped Speckit Convergence

Scope: T083 checked against US5 acceptance scenario 3, FR-015, FR-016, SC-008,
tasks, and constitution.

Convergence result: converged for T083. Archive integrity, install, reboot,
daemon readiness, active hook readiness, manager version, concrete app control,
and app restore are evidenced. T084 24-hour soak remains a separate release
gate.
