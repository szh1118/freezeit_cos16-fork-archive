# US2 Device Freeze Evidence

Date: 2026-07-04

Scope: T052 target-device freeze/unfreeze validation on three third-party apps
and one multi-process app.

Status: pass for T052 on the CPH2653 Android 16/COS16 target.

## Fixes Required During Validation

The first target run only proved signal fallback control. Device inspection then
showed the real app freezer paths under:

```text
/sys/fs/cgroup/apps/uid_<uid>/pid_<pid>/cgroup.freeze
```

The Rust daemon had only been checking `/sys/fs/cgroup/system`, so it incorrectly
treated third-party app freezer paths as unavailable. The final implementation:

- checks `/sys/fs/cgroup/apps` before `/sys/fs/cgroup/system`
- runs a 1-second daemon control loop gated by active hook health and non-empty
  manager app config
- fails closed when foreground UID query fails instead of assuming no foreground
  apps
- keeps empty hook config payloads parseable by the Java hook so test config can
  be cleared with an empty manager app config

## Verification Commands

Host and package checks after the final fixes:

```text
rtk sh -lc '. "$HOME/.cargo/env" &&
  cargo fmt --manifest-path freezeitDaemon/Cargo.toml &&
  cargo test --manifest-path freezeitDaemon/Cargo.toml --test freeze_unfreeze_state live_control_pass_treats_foreground_query_failure_as_fail_closed &&
  sh freezeitDaemon/scripts/test-host.sh &&
  sh freezeitDaemon/scripts/build-android.sh &&
  sh scripts/package-release.sh'
```

Result:

```text
freeze_unfreeze_state targeted test: pass
freezeitDaemon/scripts/test-host.sh: pass, 55 Rust contract/integration tests
freezeitDaemon/scripts/build-android.sh: pass
scripts/package-release.sh: release zip integrity pass
```

The resulting Magisk package was installed and rebooted on the target device.
Final daemon readiness after reboot:

```text
LISTEN 127.0.0.1:60613 users:(("freezeit",pid=3501,fd=3))
SetAppCfgEmpty=success
GetAppCfgLen=0
health={"status":"active","daemonReady":true,"hookHealth":"active"}
self_check={"controlAllowed":true}
stopped process check: no output
```

## Target App Set

The validation used only these four packages:

```text
com.reddit.frontpage          uid=10555
com.deepl.mobiletranslator    uid=10557
cn.com.omnimind.bot           uid=10572
moe.nb4a                      uid=10514 multi-process app
```

The manager `SetAppCfg` command was sent with mode `30` (`CFG_FREEZER`) for all
four UIDs before the control run.

## Automatic Freeze/Unfreeze Run

Local evidence log:

```text
/tmp/freezeit-t052-cgroup-20260704_022548.log
```

The test launched each selected package, moved it home/background, and then did
not read the operation log during the freeze wait. The daemon control loop had
to freeze the app by itself.

Freeze results:

```text
com.reddit.frontpage        pid=18921 freeze_elapsed_s=1.45 cgroup.freeze=1
com.deepl.mobiletranslator  pid=19345 freeze_elapsed_s=2.16 cgroup.freeze=1
cn.com.omnimind.bot         pid=19727 freeze_elapsed_s=1.48 cgroup.freeze=1
moe.nb4a                    pid=20054 freeze_elapsed_s=1.56 cgroup.freeze=1

freeze_passes=4/4
```

Foreground unfreeze results:

```text
com.reddit.frontpage        pid=18921 unfreeze_elapsed_s=0.86 cgroup.freeze=0
com.deepl.mobiletranslator  pid=19345 unfreeze_elapsed_s=1.32 cgroup.freeze=0
cn.com.omnimind.bot         pid=19727 unfreeze_elapsed_s=1.38 cgroup.freeze=0
moe.nb4a                    pid=20054 unfreeze_elapsed_s=1.47 cgroup.freeze=0

unfreeze_passes=4/4
```

The operation log contained package identity, UID, PID list, action, backend,
result, and reason. Representative entries:

```text
operationId=6  package=com.reddit.frontpage       uid=10555 pidList=[18921]       action=freeze   backend=cgroup.freeze reason="cgroup and binder freezer available" result=success
operationId=8  package=com.deepl.mobiletranslator uid=10557 pidList=[19345]       action=freeze   backend=cgroup.freeze reason="cgroup and binder freezer available" result=success
operationId=11 package=cn.com.omnimind.bot        uid=10572 pidList=[19727]       action=freeze   backend=cgroup.freeze reason="cgroup and binder freezer available" result=success
operationId=12 package=moe.nb4a                   uid=10514 pidList=[20054,20093] action=unfreeze backend=cgroup.freeze reason="foreground uid active" result=success
operationId=13 package=moe.nb4a                   uid=10514 pidList=[20054,20093] action=freeze   backend=cgroup.freeze reason="cgroup and binder freezer available" result=success
operationId=20 package=moe.nb4a                   uid=10514 pidList=[20054,20093] action=unfreeze backend=cgroup.freeze reason="foreground uid active" result=success
```

Some re-freeze entries are expected because the validation foregrounded apps
sequentially, then sent them home before moving to the next app. The important
behavior is that every foregrounded controlled app was thawed before the
validation timeout and every backgrounded selected app returned to
`cgroup.freeze=1`.

## Cleanup

After the run, the daemon was updated with the empty-config fix, the final
package was installed, and the target device was rebooted. Empty manager config
was then sent successfully:

```text
SetAppCfgEmpty=success
GetAppCfgLen=0
xp_log={"status":"active","system_server_ready":true,"config_ready":true,"screen_ready":true,"wakelock_ready":true,"network_ready":true}
```

The previous protected-record cleanup had produced repeated skip logs while the
app config was non-empty; the final empty config stops the control loop because
`app_config.is_empty()`.

Post-cleanup process state:

```text
ps -A -o PID,S,NAME | awk '$2=="T"{print}'
no output

known cgroup.freeze values checked before the final reboot:
0
0
0
0
0
```

## Scoped Brooks Review

Mode: PR Review

Scope:

- `freezeitDaemon/src/sys/procfs.rs`
- `freezeitDaemon/src/sys/socket.rs`
- `freezeitDaemon/src/protocol/manager_v1.rs`
- T052 target-device validation evidence

Findings resolved during the review:

- Warning, Change Propagation: freezer path discovery encoded only the system
  cgroup root, while the target app freezer path lives under `apps`. Remedy:
  discover app cgroup roots before system roots and add
  `discovers_uid_processes_with_app_cgroup_root_before_system_root`.
- Warning, Accidental Runtime Coupling: live control previously ran only when
  `GetOperationLogJson` was requested. Remedy: add a daemon control loop gated
  by active hook health and configured apps, with
  `live_control_pass_requires_active_hook_and_configured_apps`.
- Critical, Fail-Closed Safety: foreground UID query failure previously became
  an empty UID list. Remedy: return the foreground query error and add
  `live_control_pass_treats_foreground_query_failure_as_fail_closed`.
- Warning, Protocol Compatibility: empty app config encoded as `settings\n\n`,
  which Java `String.split("\n")` reduces to one line. Remedy: preserve empty
  managed/permissive lines with space placeholders and add
  `xposed_config_payload_keeps_empty_config_parseable_by_hook_split`.

Remaining findings for T052: none blocking.

## Scoped Speckit Convergence

Scope checked against US2 acceptance scenarios, FR-003, FR-005, FR-006,
FR-007, FR-009, FR-010, FR-011, SC-002, SC-003, and SC-005.

Convergence result: converged for T052. The target device run proves configured
third-party apps reach cgroup freezer controlled state within the configured
delay plus 5 seconds, foreground launches thaw within 2 seconds, the
multi-process app has multi-PID log entries, and every validation control or
skip/fallback entry contains identity, action, result, backend, and reason.
