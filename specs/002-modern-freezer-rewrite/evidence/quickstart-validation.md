# Quickstart Validation Evidence

Date: 2026-07-04

Scope: T086 full quickstart validation flow.

Status: partial. T086 remains unchecked.

## Passed Sections

Build validation:

- Rust host tests pass: 55 Rust contract/integration tests.
- Android daemon release build passes.
- Manager release APK assemble passes.
- Magisk release package builds and `validate-release-zip.sh` reports
  `release zip integrity: pass`.

Device baseline:

- Target device connected as `3B1F4LE5MS142WJY`.
- Android 16 / SDK 36 baseline is recorded in the feature plan and prior
  baseline evidence.
- Root command returns UID 0.
- App cgroup freezer paths exist under `/sys/fs/cgroup/apps`.
- Binder device is present.

Install and boot:

- `freezeit_3.2.0SelfUse.zip` installs through
  `magisk --install-module`.
- Reboot completes with daemon listening on `127.0.0.1:60613`.
- Final health after reboot:

```text
SetAppCfgEmpty=success
GetAppCfgLen=0
health={"status":"active","daemonReady":true,"hookHealth":"active"}
self_check={"controlAllowed":true}
```

Freeze/unfreeze:

- T052 passes on three third-party packages and one multi-process package.
- Automatic cgroup freeze: `freeze_passes=4/4`.
- Foreground thaw: `unfreeze_passes=4/4`.
- Operation logs contain package, UID, PID list, action, backend, result, and
  reason.

Release validation:

- T083 passes for archive integrity, install, reboot, activation, app control,
  and restore.

## Open Sections

The quickstart is not fully complete because it explicitly includes validation
that is still open elsewhere:

- Degraded-state validation depends on T074. Current T074 evidence is partial:
  read-only baseline and daemon restart pass, but config-corrupt and
  network/wake-lock/screen unavailable fault injection are not yet proven on
  target.
- Final release declaration depends on T084. The required 24-hour self-use soak
  has not elapsed and must not be simulated.
- Final aggregate `/brooks-review` and `/speckit-converge` remain open as T089
  and T090.

## Scoped Review And Convergence

Scoped review result: no new code finding is introduced by this partial
quickstart evidence.

Scoped convergence result: not converged for T086 until T074 and T084 are
complete.
