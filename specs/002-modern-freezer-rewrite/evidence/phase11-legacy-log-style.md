# Phase 11 Legacy Log Style Evidence

Date: 2026-07-04

Scope: T107.

## Change Summary

- Restored the manager Logcat switch to the original default surfaces:
  work log uses `ManagerCmd.getLog`, Xposed log uses `ManagerCmd.getXpLog`.
- Reset Logcat's cached payload length whenever the log timer is reset so a
  work-log/Xposed-log switch always refreshes, even when both payloads have the
  same byte length.
- Added contract coverage proving `clearLog` clears only the default manager
  log surface while `getOperationLogJson` remains available as a structured
  diagnostic endpoint.
- Preserved the existing Rust legacy text formatter for timestamped
  Chinese/emoji freeze, unfreeze, launch, terminate, postpone, skip, fallback,
  Binder/blocker, recovery, config-change, label-update, clear-log, and
  process-state behavior.

## Scoped Brooks Review

Mode: PR Review

Scope:

- `freezeitApp/app/src/main/java/io/github/jark006/freezeit/fragment/Logcat.java`
- `freezeitDaemon/tests/contract/manager_health_v1.rs`
- Existing formatter/handler coverage in:
  - `freezeitDaemon/src/app/operation_log.rs`
  - `freezeitDaemon/src/protocol/manager_v1.rs`
  - `freezeitDaemon/tests/contract/operation_log_json.rs`

Result: PASS, no findings.

Review notes:

- Change propagation is bounded to the manager Logcat command selector and the
  contract test that guards it.
- No new abstraction, dependency, or protocol command was introduced.
- Tests verify the observable behavior: default manager log stays legacy text,
  Xposed switch uses the Xposed log command, and structured JSON diagnostics
  remain on the v2 diagnostic command.

## Scoped Speckit Converge

Result: Converged for T107; no new tasks appended.

Checked sources:

- `spec.md`: FR-001, FR-011, SC-005, US4/AC1, US4/AC2.
- `plan.md`: existing manager log surfaces preserved; v2 diagnostics additive.
- `tasks.md`: T107 file list and behavior categories.
- `.specify/memory/constitution.md`: active verification and honest delivery
  gates.

Convergence findings: none.

## Verification

```text
rtk cargo test --manifest-path freezeitDaemon/Cargo.toml --test manager_health_v1 logcat_switches_between_work_log_and_xposed_log_not_json_diagnostics
result: pass, 1 test
```

```text
rtk cargo test --manifest-path freezeitDaemon/Cargo.toml --test manager_health_v1
result: pass, 26 tests
```

```text
rtk cargo test --manifest-path freezeitDaemon/Cargo.toml --test operation_log_json
result: pass, 7 tests
```

```text
rtk sh freezeitDaemon/scripts/test-host.sh
result: pass, Rust unit/contract/integration/doc tests passed
```

```text
rtk env JAVA_HOME=/usr/lib/jvm/java-17-openjdk ANDROID_HOME=/home/admin/Android/Sdk ANDROID_SDK_ROOT=/home/admin/Android/Sdk sh ./gradlew :app:assembleDebug
result: pass, BUILD SUCCESSFUL
```

Environment note: Gradle requires Java 17 and a configured Android SDK path in
this workspace. Running without those environment values failed before the
final verified command above.
