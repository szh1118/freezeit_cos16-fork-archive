# Phase 10 Convergence Evidence

Date: 2026-07-04

Scope: T100-T106 plus the target validation needed to close T093 and T099.

## Code Changes

- `freezeitDaemon/src/sys/cgroup.rs`
  - Added explicit cgroup v2 freezer capability detection.
  - Reads `/sys/fs/cgroup/cgroup.controllers` as evidence.
  - Prefers Android app cgroup v2 `cgroup.freeze` paths under `apps` before
    system/generic cgroup paths.
  - Treats discovered Android `cgroup.freeze` files as available even when the
    root controllers file omits `freezer`, because the target ROM exposes
    working app freeze files there.
- `freezeitDaemon/src/sys/binder.rs`
  - Replaced "device exists means available" with a capability result.
  - Reports present binder devices as `untested` until a real freezer ioctl
    probe is proven safe on target.
- `freezeitDaemon/src/app/freezer_backend.rs`
  - Routes freezer write failures, including `PermissionDenied`, through the
    configured fallback order.
  - Adds a PendingFreeze idle blocker for busy Binder/process evidence.
- `freezeitDaemon/src/sys/procfs.rs`
  - Reads `/proc/<pid>/status` context-switch counters as target-observable
    process quiescence evidence.
- `freezeitDaemon/src/app/controller.rs`
  - Uses manager settings byte 2/4 as live freeze/terminate delay instead of
    immediate `delay_ms = 0` control.
  - Records PendingFreeze postpone operations before freezing.
  - Re-scans UID processes after freeze; if new same-UID processes appear, the
    operation is marked `partial` and thawed instead of claiming a complete
    freeze.
  - Adds idle evidence to operation details.

## Host Verification

```text
rtk cargo test --manifest-path freezeitDaemon/Cargo.toml --test sys_freezer_primitives
result: pass, 5 tests
```

```text
rtk cargo test --manifest-path freezeitDaemon/Cargo.toml --test freezer_backend_decisions
result: pass, 6 tests
```

```text
rtk cargo test --manifest-path freezeitDaemon/Cargo.toml --test freeze_unfreeze_state
result: pass, 10 tests
```

```text
rtk cargo test --manifest-path freezeitDaemon/Cargo.toml --test procfs_runtime_discovery discovers_uid_processes_with_cgroup_freeze_path
result: pass, context-switch evidence populated
```

```text
rtk sh freezeitDaemon/scripts/test-host.sh
result: pass, all Rust unit/contract/integration/doc tests passed
```

```text
rtk sh -lc '. "$HOME/.cargo/env" 2>/dev/null || true; sh freezeitDaemon/scripts/build-android.sh'
result: pass, release daemon built for aarch64-linux-android
```

```text
rtk sh scripts/package-release.sh
result: pass, release zip integrity pass
```

Manager APK build after setting `JAVA_HOME=/usr/lib/jvm/java-17-openjdk`:

```text
cd freezeitApp && sh ./gradlew :app:assembleRelease
result: BUILD SUCCESSFUL
```

## Target Device Evidence

Target: `3B1F4LE5MS142WJY`.

The rebuilt daemon was pushed to `/data/adb/modules/freezeit/freezeit`, chmodded
755, and restarted. The daemon listened on `127.0.0.1:60613`.

### T093 Manager-Visible Legacy Log

`cmd=4 getLog` returned legacy-style manager-visible operation lines:

```text
[14:34:30]  ⏳延迟冻结 com.reddit.frontpage 1进程 uid=10555 pid=[15331] backend=cgroup.freeze result=postponed reason=pending freeze delay 10000ms details=process_count=1
[14:34:41]  ❄️冻结 com.google.android.apps.nbu.files 1进程 uid=10124 pid=[13216] backend=cgroup.freeze result=success reason=cgroup and binder freezer available details=process_count=1
```

The same operations were visible through `cmd=74 getOperationLogJson` with
package identity, UID, pidList, action, backend, reason, result, and details.

### T100 cgroup v2 Capability And ROM Evidence

`cmd=72 getCapabilityReport`:

```json
{"name":"cgroup_v2_freezer","status":"available","reason":"/sys/fs/cgroup/cgroup.controllers contains freezer=false; freeze_files=248"}
```

`cmd=73 getCompatibilityBaseline` reported the same capability evidence in the
compatibility report.

Read-only ROM evidence:

```text
/sys/fs/cgroup/cgroup.controllers: empty
find /sys/fs/cgroup/apps /sys/fs/cgroup/system -name cgroup.freeze | wc -l: 498
sample:
/sys/fs/cgroup/apps/uid_10133/cgroup.freeze
/sys/fs/cgroup/apps/uid_10425/pid_10950/cgroup.freeze
/sys/fs/cgroup/apps/uid_10425/cgroup.freeze
```

`/system/etc/cgroups.json` records cgroup v2 freezer under
`/sys/fs/cgroup`:

```json
"Cgroups2": {
  "Path": "/sys/fs/cgroup",
  "Controllers": [
    { "Controller": "freezer", "Path": "." }
  ]
}
```

`/system_ext/etc/init/hans.rc` records a legacy ROM freezer mount under
`/dev/freezer`, but the Rust daemon prefers Android app cgroup v2 paths:

```text
mkdir /dev/freezer
mount cgroup none /dev/freezer freezer
mkdir /dev/freezer/frozen
mkdir /dev/freezer/thaw
```

No freezer lines were found in the sampled `system/etc/task_profiles.json`
grep.

### T101 Binder Freezer Capability

Device binder evidence:

```text
/dev/binder -> /dev/binderfs/binder
/dev/binderfs/binder crw-rw-rw- root root
```

Manager-visible capability:

```json
{"name":"binder_freezer","status":"untested","reason":"binder device present; BINDER_FREEZE ioctl=0x4004620e, BINDER_UNFREEZE ioctl=0x4004620f; target probe required before marking available"}
```

This closes the hardcoded-availability gap without claiming unproven binder
freezer ioctl safety.

### T102 Fallback On Permission/Unsafe Freezer Failure

Host coverage:

```text
permission_denied_freezer_write_uses_configured_fallback_order: pass
```

The control loop now records fallback/postpone/skip operations instead of
returning a control-pass error when cgroup freeze application fails.

### T103 PendingFreeze Idle Evidence

Target operation details now include `/proc/<pid>/status` context-switch
evidence:

```text
details="process_count=1 idle_evidence=pid28077:context_switches voluntary=742 nonvoluntary=383 total=1125"
details="process_count=3 idle_evidence=pid1306:context_switches voluntary=47193 nonvoluntary=7202 total=54395|pid2644:context_switches voluntary=1149 nonvoluntary=182 total=1331|pid3215:context_switches voluntary=90 nonvoluntary=101 total=191"
```

Host coverage also proves busy Binder/process evidence postpones PendingFreeze:

```text
pending_freeze_is_postponed_when_binder_or_process_evidence_is_busy: pass
```

### T104 Post-Freeze UID Rescan

Host integration coverage:

```text
control_pass_records_partial_freeze_when_uid_rescan_finds_new_processes: pass
```

The control loop re-discovers same-UID processes after freeze. If new PIDs are
observed, it records `result=partial`, includes the new PIDs, and thaws the
observed process set.

### T105 Manager Policy Delay

Normal target settings produced 10/20 second PendingFreeze entries and later
freeze entries:

```text
action=postpone reason="pending freeze delay 10000ms"
action=freeze result=success reason="cgroup and binder freezer available"
```

For the required 3-5 minute validation, `settings.db` was backed up, byte 2 was
temporarily set to `180`, daemon was restarted, and `com.reddit.frontpage` was
foregrounded then sent home. Evidence:

```text
operationId=23 packageName="com.reddit.frontpage" action="postpone"
reason="pending freeze delay 180000ms" result="postponed"

operationId=34 packageName="com.reddit.frontpage" action="freeze"
reason="cgroup and binder freezer available" result="success"
```

The original settings file was restored and verified:

```text
od -An -tu1 -j2 -N1 /data/adb/modules/freezeit/settings.db
10
```

### T106 Threat Model Boundary

The threat model is documented in `research.md` and release notes. Summary:

- Freezing mitigates background execution only after a process becomes eligible
  and the configured delay/idle gate passes.
- It does not prevent code execution before freeze, foreground abuse, system or
  root compromise, privileged ROM allowlists, or hook/root readiness failures.
- Protected packages, hook/root/freezer readiness gates, compatibility reports,
  and release notes communicate those boundaries.

## Scoped Review

Spec compliance review:

- T093: satisfied by target `cmd=4` legacy log evidence.
- T099: satisfied by the command-by-command matrix appended to
  `legacy-gap-audit.md`.
- T100-T105: satisfied by host tests and target evidence above.
- T106: satisfied by `research.md` and `freezeitRelease/README.md`.

Code quality review:

- The new code keeps platform probing under `sys/*`.
- Control policy and operation logging remain in `app/controller.rs` and
  `app/freezer_backend.rs`.
- No broad compatibility claim is introduced; binder freezer remains `untested`
  until a safe ioctl probe is proven.

## Remaining Blockers Outside Phase 10

The following tasks remain intentionally open:

- T074/T095: config-corrupt and network/wake-lock/screen unavailable
  target-observable fault validation is incomplete.
- T084/T096: the 24-hour self-use soak requires real elapsed time.
- T086/T097: full quickstart validation depends on T074/T095 and T084/T096.
- T089/T090: final aggregate review/convergence cannot close while the above
  required validation tasks remain open.
