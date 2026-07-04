# Legacy C++ Behavior Gap Audit

Date: 2026-07-04

Scope: T088 audit of legacy `freezeitVS/include/*.hpp` and
`freezeitVS/src/main.cpp` against the Rust daemon implementation.

## Legacy Areas Reviewed

- `freezeitVS/include/server.hpp`: localhost manager protocol and
  command dispatch.
- `freezeitVS/include/freezer.hpp`: freezer/control behavior
  reference.
- `freezeitVS/include/managedApp.hpp`: app policy/classification
  reference.
- `freezeitVS/include/settings.hpp`: legacy settings/config
  reference.
- `freezeitVS/include/systemTools.hpp`, `doze.hpp`, `utils.hpp`:
  system integration helpers.
- `freezeitVS/src/main.cpp`: daemon startup reference.

## Covered By Rust Rewrite

- Manager localhost protocol frame shape and core v1 commands are covered by
  `manager_v1.rs` tests.
- `getAppCfg`/`setAppCfg` binary 12-byte triples are covered by manager health
  and policy migration tests.
- cgroup and binder primitive wrappers are covered by sys freezer primitive
  tests.
- Runtime process PID/UID/package matching is covered by procfs runtime tests.
- Protected classification, foreground classification, UID reconciliation, and
  migration behavior are covered by US3 tests.
- Operation diagnostics, recovery recording, degraded health, and compatibility
  reporting are covered by US4/US5 tests.

## Remaining Gaps

- Real freeze/unfreeze timing and foreground restore behavior are not yet proven
  on target device. Tracked by T052.
- Deliberate missing LSPosed scope and hook-inactive validation is not yet
  complete. Tracked by T038 and T074.
- Full legacy settings semantics are preserved as opaque bytes so far; detailed
  per-setting behavior remains a release validation risk until quickstart and
  self-use checks complete.
- Legacy C++ real-time CPU/memory graph payload behavior is not implemented in
  the Rust daemon beyond preserving manager build compatibility.
- Public/broad-device compatibility behavior remains out of scope for this
  self-use rewrite.

## Scoped Brooks Review

Mode: PR Review

Scope: T088 audit documentation.

Health Score: 100/100.

Finding summary: no Critical, Warning, or Suggestion findings.

## Scoped Speckit Convergence

Scope: T088 checked against plan legacy-reference decision, US1-US5 remaining
task list, and constitution.

Convergence result: no additional audit task required. Remaining behavior gaps
are already represented by open validation tasks T038, T052, T074, T083, T084,
and T086.

## Correction After Phase 9 Convergence

Date: 2026-07-04

The scoped convergence conclusion above was too optimistic. Phase 9 later found
that the prior T088 audit did not provide a command-by-command legacy C++ parity
matrix and missed user-visible regressions in real-time chart/history, manager
logs, settings writes, temperature, battery power, UID CPU time, and diagnostic
command IDs.

Current status:

- Fixed and revalidated in `evidence/phase9-device-fixes.md`: real-time chart
  history, RAM metrics, CPU deltas, battery power, temperature, UID CPU time,
  hook readiness, startup hook config sync, and v2 diagnostic command IDs.
- Partially fixed but not yet target-validated: manager-visible legacy log
  surface. The prior audit treated Rust's `operationId=...` text as acceptable
  manager log compatibility. That was wrong: the C++ manager log is the
  `Freezeit::log/logFmt` in-memory text buffer with `[HH:MM:SS]` prefixes and
  emoji operation lines such as `☀️解冻`, `❄️冻结`, `🧊冻结`, and `😭关闭`.
  Rust now has host-tested legacy emoji formatting and a rebuilt Android
  package containing those strings, but T093 remains open until it is installed
  and `cmd=4` is validated on the target device.
- Still open: T099 must redo the C++ parity audit as a command-by-command matrix
  covering remaining legacy behaviors and intentional deviations. This file is
  no longer sufficient evidence to close that task.

## Command-By-Command Legacy Parity Matrix

Date: 2026-07-04

This matrix supersedes the incomplete T088 conclusion above and closes T099.

| Legacy surface | C++ reference | Rust rewrite status | Evidence / deviation |
| --- | --- | --- | --- |
| `getPropInfo` | `server.hpp` returns module/version/environment, cluster count, Android/kernel, work mode, ext memory | Preserved with daemon/hook health strings | `manager_health_v1::get_prop_info_returns_legacy_six_line_payload`; target `cmd=2` in `phase9-device-fixes.md` |
| `getChangelog` | returns module changelog text | Preserved | `remaining_legacy_commands_return_compatibility_payloads` |
| `getLog` | in-memory text log from `Freezeit::log/logFmt`, with timestamped Chinese/emoji operation lines | Preserved with structured operation fields embedded in legacy-style lines | `phase10-convergence.md` target `cmd=4`; `operation_log_legacy_text_formats_*` |
| `getAppCfg` / `setAppCfg` | 12-byte UID/mode/permissive triples shared with manager and hook config | Preserved | `app_config_read_and_write_remain_manager_compatible`; `xposed_config_payload_translates_manager_binary_records` |
| `getRealTimeInfo` | bitmap chart plus 23 integer metrics, persistent CPU/RAM history | Preserved for manager payload shape, CPU delta, RAM bars, battery power, temperature | `phase9-device-fixes.md`; unit tests for CPU history, temperature, battery power |
| `getSettings` / `setSettingsVar` | 256-byte legacy settings block; index 2 freeze timeout, index 4 terminate timeout, validation on writes | Preserved as opaque settings with write validation; daemon now uses index 2/4 in live control loop | `set_settings_var_*`; T105 target 180-second temporary setting validation in `phase10-convergence.md` |
| `getUidTime` | UID CPU time delta records from `/proc/uid_cputime/show_uid_stat` | Preserved when source exists, stable empty response otherwise | `get_uid_time_returns_managed_legacy_cpu_records_with_delta` |
| `getProcState` / `printFreezerProc` | text table with process memory and freezer state | Compatibility text preserved; Rust appends structured operation log text | `remaining_legacy_commands_return_compatibility_payloads`; target logs in phase 9/10 |
| freeze/unfreeze | legacy freezer/binder/signal paths with timestamped freeze/unfreeze text | Replaced with system-aware app cgroup v2 primary backend, signal fallback, structured operation log | `us2-device-freeze.md`; `phase10-convergence.md` |
| pending freeze delay | C++ pending queue driven by `freezeTimeout`/`terminateTimeout` | Preserved in Rust live control loop via PendingFreeze state | `control_pass_uses_manager_freeze_delay_before_freezing`; target 180-second validation |
| binder freezer | C++ probes Binder feature and uses ioctl with rollback on failed/pending transactions | Rust does not claim Binder freezer availability yet; reports `untested` and relies on cgroup plus fallback | Intentional conservative deviation in T101 |
| cgroup freezer path | C++ supports legacy `/dev/freezer` and FreezerV2 variants | Rust prefers Android 16 app cgroup v2 `cgroup.freeze`; records ROM `/dev/freezer` as legacy evidence only | T100 evidence in `phase10-convergence.md` |
| wake-lock/network/screen-state behavior | C++ integrates hook/system helper behavior and logs failures | Host-mode degraded reasons exist; target fault injection remains open | Tracked by T074/T095, not closed by T099 |
| recovery after daemon restart | C++ runtime state is rebuilt through process scans and freezer status | Rust records restart reconciliation and scans current processes before new control | `recover_after_restart` integration test; `us4-recovery.md` restart evidence |
| compatibility diagnostics | C++ had implicit ROM/runtime assumptions | Rust adds explicit health, capability, compatibility, operation log JSON commands | `manager_health_v1` v2 diagnostics; target `cmd=72/73/74/75` |

Intentional deviations:

- Binder freezer is not marked available until a target-safe ioctl probe is
  implemented and validated.
- ROM legacy `/dev/freezer` is not the preferred path on the verified Android 16
  baseline; app cgroup v2 paths are preferred.
- Broad public device compatibility remains out of scope for `3.2.0SelfUse`.
- Network, wake-lock, and screen-state unavailable fault injection remains open
  under T074/T095 because breaking those services on the daily-use phone was
  not performed.
