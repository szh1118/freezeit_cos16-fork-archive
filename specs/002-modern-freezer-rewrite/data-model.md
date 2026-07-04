# Data Model: Modern Freezer Rewrite

## ManagedApp

Represents a package known to Freezeit.

**Fields**:
- `package_name`: stable package identifier.
- `user_id`: Android user/profile ID.
- `uid`: current Linux UID resolved from package manager.
- `label`: display label migrated from the current label file or package manager.
- `is_system_app`: true for system/privileged packages.
- `protected_reason`: optional reason such as manager, launcher, input method, root manager, hook manager, system-critical, or user-protected.
- `policy_id`: link to the active `FreezePolicy`.
- `last_seen_baseline`: ROM/package inventory version that last confirmed this package.

**Validation**:
- `package_name` plus `user_id` is the policy identity; UID alone is never sufficient.
- UID changes must trigger package reconciliation before any control operation.
- Protected packages cannot enter freeze/terminate policy through normal defaults.

## FreezePolicy

User-selected behavior for a managed app.

**Fields**:
- `mode`: `protected`, `free`, `freeze`, `freeze_with_restrictions`, or `terminate`.
- `delay_ms`: delay after background eligibility before control.
- `foreground_strategy`: `strict` or `permissive`.
- `allow_network_restriction`: whether network break behavior is allowed.
- `allow_wakelock_restriction`: whether wake-lock handling is allowed.
- `fallback_strategy`: ordered set of `postpone`, `alternate_freezer`, `signal`, `terminate`, `skip`.
- `updated_at`: monotonic or wall-clock update marker.

**Validation**:
- `terminate` requires explicit policy, never default migration.
- `signal` and `terminate` fallbacks are disabled for protected/system-critical apps.
- Delay must be non-negative and bounded to avoid immediate boot-time mass control.

## RuntimeProcess

Current process belonging to a managed app.

**Fields**:
- `pid`: process ID.
- `uid`: process UID from `/proc/<pid>` stat ownership.
- `package_name`: resolved package membership.
- `process_name`: command line or ActivityManager process name.
- `proc_state`: foreground, visible, service, cached, empty, unknown.
- `control_state`: `running`, `pending_freeze`, `frozen`, `unfreezing`, `unknown`.
- `cgroup_freeze_path`: detected `cgroup.freeze` path when present.
- `binder_state`: optional binder freezer/readiness info.
- `last_seen_at`: observation timestamp.

**Validation**:
- PID must still exist and UID must match the package immediately before control.
- Multi-process apps are controlled process-by-process, with app-level result aggregation.
- Unknown or foreground-visible processes are not frozen unless the policy and safety checks allow it.

## ControlCapability

Runtime evidence that a control path is available and safe.

**Fields**:
- `name`: `root`, `package_inventory`, `lsposed_system_server`, `cgroup_v2_freezer`, `binder_freezer`, `signal_control`, `network_break`, `wakelock_control`.
- `status`: `available`, `missing`, `degraded`, or `untested`.
- `evidence`: path, command result, API readiness, or error string.
- `checked_at`: timestamp.
- `risk_level`: `normal`, `caution`, or `disabled`.

**Validation**:
- Missing required capability disables enforcement.
- Degraded optional capability must select a documented fallback.
- Capability checks run at boot, daemon restart, manager request, and before release validation.

## ControlOperation

Recorded attempt to affect an app/process.

**Fields**:
- `operation_id`: monotonic local ID.
- `timestamp`: operation time.
- `package_name`, `uid`, `pid_list`: target identity.
- `action`: `freeze`, `unfreeze`, `terminate`, `postpone`, `fallback`, `skip`, `recover`.
- `backend`: selected backend.
- `reason`: foreground transition, delay elapsed, user launch, capability missing, hook missing, config migration, or recovery.
- `result`: `success`, `partial`, `failed`, `skipped`, `postponed`.
- `details`: compact diagnostic message.

**Validation**:
- Every attempted state change creates exactly one operation record.
- Failed and partial operations include a blocker and next action.
- Operation logs must be readable from the manager and from release validation artifacts.

## ModuleHealth

Current readiness of the whole module.

**Fields**:
- `manager_ready`: manager can reach daemon.
- `daemon_ready`: daemon initialized config, package inventory, and scheduler.
- `hook_ready`: LSPosed system_server bridge active.
- `root_ready`: Magisk root operations available.
- `freezer_ready`: required freezer backend available.
- `policy_ready`: migrated or loaded policy available.
- `status`: `active`, `degraded`, or `inactive`.
- `degraded_reasons`: list of missing or degraded capabilities.

**Validation**:
- `active` requires daemon, root, package inventory, hook readiness, and freezer capability.
- `degraded` allows diagnostics and safe manager operations but blocks unsafe app control.
- `inactive` is reported when manager cannot use the module.

## CompatibilityBaseline

Device/ROM/root/hook baseline used to decide whether the release is validated.

**Fields**:
- `device_model`
- `android_release`
- `sdk_level`
- `build_fingerprint`
- `build_incremental`
- `kernel_release`
- `magisk_context`
- `lsposed_api_target`
- `freezer_paths_sample`
- `validated_at`

**Validation**:
- Fingerprint or incremental changes are warnings at startup and blockers for release claims until revalidated.
- Baseline mismatch does not force destructive behavior; it disables unsafe paths if capability checks fail.

## State Transitions

```text
unknown -> running
running -> pending_freeze
pending_freeze -> frozen
pending_freeze -> postponed
pending_freeze -> skipped
frozen -> unfreezing
unfreezing -> running
frozen -> recovered
any -> unknown
```

**Transition rules**:
- `running -> pending_freeze`: app is background/cached and delay timer starts.
- `pending_freeze -> frozen`: eligibility and backend capability pass immediately before operation.
- `pending_freeze -> postponed`: foreground, audio/call/screen-recording, hook uncertainty, or process churn blocks safe control.
- `frozen -> unfreezing`: app becomes foreground/user-visible or policy changes.
- `any -> unknown`: daemon restart, package update, process exit during operation, or capability loss.
- Recovery always reconciles current process state before new actions.
