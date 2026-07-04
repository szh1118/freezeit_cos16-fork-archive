# Tasks: Modern Freezer Rewrite

**Input**: Design documents from `specs/002-modern-freezer-rewrite/`

**Prerequisites**: `plan.md`, `spec.md`, `research.md`, `data-model.md`, `contracts/`, `quickstart.md`

**Tests**: Verification tasks are required for each user story by the feature spec and constitution. Automated tests come before implementation where practical; device validation tasks record expected evidence when automation is not sufficient.

**Organization**: Tasks are grouped by user story so each story can be implemented and tested independently after the shared foundation is complete.

**Constitution task-completion gate**: No checkbox in this file may be marked complete until the implementer has run the narrowest practical `/brooks-review` and `/speckit-converge` check for that task's changed scope, resolved blockers, and recorded the evidence in the task's referenced evidence file or phase evidence file. T089 and T090 are final aggregate review/convergence gates; they do not replace the per-task gate.

## Format: `[ID] [P?] [Story] Description`

- **[P]**: Can run in parallel because it touches different files or only adds independent test/evidence files.
- **[Story]**: Required only for user-story phases.
- Every task includes an exact repository-relative file path.
- Every task checkbox inherits the constitution task-completion gate above.

## Phase 1: Setup (Shared Infrastructure)

**Purpose**: Create the Rust daemon workspace, build hooks, fixture locations, and validation evidence directories without changing runtime behavior.

- [X] T001 Create the Rust daemon Cargo project entrypoint in `freezeitDaemon/Cargo.toml` and `freezeitDaemon/src/main.rs`
- [X] T002 [P] Create the Rust daemon module scaffold in `freezeitDaemon/src/lib.rs`
- [X] T003 [P] Configure Rust formatting and Android target defaults in `freezeitDaemon/rustfmt.toml` and `freezeitDaemon/.cargo/config.toml`
- [X] T004 [P] Add the cargo-ndk Android build helper in `freezeitDaemon/scripts/build-android.sh`
- [X] T005 [P] Create daemon test fixture documentation in `freezeitDaemon/tests/fixtures/README.md`
- [X] T006 Add Rust daemon packaging integration notes in `freezeitVS/magisk/rust-daemon-integration.md`
- [X] T007 [P] Create validation evidence directory documentation in `specs/002-modern-freezer-rewrite/evidence/README.md`
- [X] T008 [P] Add the read-only target baseline validation helper in `scripts/validate-device-baseline.sh`

---

## Phase 2: Foundational (Blocking Prerequisites)

**Purpose**: Implement shared daemon data types, protocol parsers, low-level wrappers, configuration loading, and health skeletons required before any user story can be completed.

**Checkpoint**: No user story work is complete until every foundational task passes host checks and has baseline evidence.

- [X] T009 [P] Implement `ManagedApp` and `FreezePolicy` domain types in `freezeitDaemon/src/domain/policy.rs`
- [X] T010 [P] Implement `RuntimeProcess` domain type in `freezeitDaemon/src/domain/runtime.rs`
- [X] T011 [P] Implement `ControlCapability` domain type in `freezeitDaemon/src/domain/capability.rs`
- [X] T012 [P] Implement `ControlOperation` domain type in `freezeitDaemon/src/domain/operation.rs`
- [X] T013 Implement shared daemon error handling in `freezeitDaemon/src/app/error.rs`
- [X] T014 Implement structured daemon logging primitives in `freezeitDaemon/src/app/logging.rs`
- [X] T015 [P] Implement manager v1 frame parser/encoder and checksum tests in `freezeitDaemon/src/protocol/manager_v1.rs` and `freezeitDaemon/tests/contract/manager_v1_frame.rs`
- [X] T016 [P] Implement LSPosed bridge frame parser and command constants in `freezeitDaemon/src/protocol/xposed.rs` and `freezeitDaemon/tests/contract/xposed_bridge_frame.rs`
- [X] T017 Implement typed config loading and legacy migration skeletons in `freezeitDaemon/src/config/loader.rs` and `freezeitDaemon/src/config/migration.rs`
- [X] T018 [P] Implement read-only procfs helpers in `freezeitDaemon/src/sys/procfs.rs`
- [X] T019 [P] Implement cgroup freezer path discovery helpers in `freezeitDaemon/src/sys/cgroup.rs`
- [X] T020 [P] Implement binder device discovery and ioctl wrapper skeleton in `freezeitDaemon/src/sys/binder.rs`
- [X] T021 [P] Implement signal wrapper with test-mode safeguards in `freezeitDaemon/src/sys/signal.rs`
- [X] T022 Implement daemon controller startup skeleton in `freezeitDaemon/src/app/controller.rs` and `freezeitDaemon/src/main.rs`
- [X] T023 Implement module health aggregation skeleton in `freezeitDaemon/src/app/health.rs`
- [X] T024 [P] Add host test runner script in `freezeitDaemon/scripts/test-host.sh`
- [X] T025 Run foundational Rust checks and record results in `specs/002-modern-freezer-rewrite/evidence/foundation-checks.md`

---

## Phase 3: User Story 1 - Install and Boot Reliably (Priority: P1)

**Goal**: The rewritten module installs, boots, starts the daemon after unlock, and reports manager, hook, daemon, root, and freezer readiness without manual shell intervention.

**Independent Test**: Install the Magisk package on the target device, reboot, unlock once, open the manager, and confirm active or clearly degraded readiness within 30 seconds.

### Verification for User Story 1

- [X] T026 [P] [US1] Add manager health protocol contract tests in `freezeitDaemon/tests/contract/manager_health_v1.rs`
- [X] T027 [P] [US1] Add Magisk archive content verification script in `scripts/validate-magisk-zip.sh`
- [X] T028 [P] [US1] Add install, reboot, unlock, readiness, missing LSPosed scope, and hook-inactive degraded ADB validation script in `scripts/validate-install-boot.sh`

### Implementation for User Story 1

- [X] T029 [US1] Implement manager v1 read-only command handlers for `getPropInfo`, `getLog`, `getSettings`, and `getXpLog` in `freezeitDaemon/src/protocol/manager_v1.rs`
- [X] T030 [US1] Implement the localhost daemon server on `127.0.0.1:60613` in `freezeitDaemon/src/sys/socket.rs` and `freezeitDaemon/src/app/controller.rs`
- [X] T031 [US1] Implement active, degraded, and inactive `ModuleHealth` evaluation in `freezeitDaemon/src/app/health.rs`
- [X] T032 [US1] Add `GET_HOOK_HEALTH` support to the hook bridge in `freezeitApp/app/src/main/java/io/github/jark006/freezeit/hook/android/FreezeitService.java` and `freezeitDaemon/src/protocol/xposed.rs`
- [X] T033 [US1] Update manager readiness rendering for daemon and hook health in `freezeitApp/app/src/main/java/io/github/jark006/freezeit/fragment/Home.java`
- [X] T034 [US1] Integrate the Rust daemon binary into Magisk startup and install scripts in `freezeitVS/magisk/service.sh` and `freezeitVS/magisk/customize.sh`
- [X] T035 [US1] Implement boot-after-unlock policy loading and recovery retry in `freezeitDaemon/src/config/loader.rs` and `freezeitDaemon/src/app/controller.rs`
- [X] T036 [US1] Implement fail-closed degraded behavior for missing hook scope or missing bridge in `freezeitDaemon/src/app/health.rs` and `freezeitDaemon/src/protocol/xposed.rs`
- [X] T037 [US1] Build the daemon, manager APK, and test Magisk zip and record results in `specs/002-modern-freezer-rewrite/evidence/us1-build.md`
- [X] T038 [US1] Run target-device install, reboot, unlock, manager readiness, missing LSPosed scope, and hook-inactive degraded validation and record evidence in `specs/002-modern-freezer-rewrite/evidence/us1-install-boot.md`

**Checkpoint**: User Story 1 is complete only when the manager shows active readiness on the target device, or a clear degraded state with no unsafe control when a required dependency is deliberately disabled.

---

## Phase 4: User Story 2 - Freeze Background Apps Using Modern System Semantics (Priority: P1)

**Goal**: Selected background apps are frozen through the safest supported Android 16/COS16 freezer path and unfrozen before foreground use breaks.

**Independent Test**: Select several third-party apps, move them foreground/background, verify freeze/unfreeze timing and logs, and confirm no crash, ANR, or stale frozen state.

### Verification for User Story 2

- [X] T039 [P] [US2] Add freezer backend capability and fallback decision tests, including network, wake-lock, and screen-state unavailable cases, in `freezeitDaemon/tests/contract/freezer_backend_decisions.rs`
- [X] T040 [P] [US2] Add freeze/unfreeze state transition integration tests in `freezeitDaemon/tests/integration/freeze_unfreeze_state.rs`
- [X] T041 [P] [US2] Add target-device freeze/unfreeze validation helper in `scripts/validate-freeze-unfreeze.sh`

### Implementation for User Story 2

- [X] T042 [US2] Implement cgroup v2 `cgroup.freeze` read/write operations in `freezeitDaemon/src/sys/cgroup.rs`
- [X] T043 [US2] Implement binder freezer ioctl operations with capability detection in `freezeitDaemon/src/sys/binder.rs`
- [X] T044 [US2] Implement `SystemAwareCgroupBinderBackend` in `freezeitDaemon/src/app/freezer_backend.rs`
- [X] T045 [US2] Implement background delay scheduling and foreground cancellation in `freezeitDaemon/src/app/scheduler.rs`
- [X] T046 [US2] Implement runtime process discovery with immediate PID/UID recheck in `freezeitDaemon/src/sys/procfs.rs` and `freezeitDaemon/src/domain/runtime.rs`
- [X] T047 [US2] Add LSPosed runtime app state commands for foreground, cached, pending freeze, and frozen hints in `freezeitApp/app/src/main/java/io/github/jark006/freezeit/hook/android/FreezeitService.java` and `freezeitDaemon/src/protocol/xposed.rs`
- [X] T048 [US2] Implement freeze and unfreeze controller flows in `freezeitDaemon/src/app/controller.rs`
- [X] T049 [US2] Implement fallback order `postpone`, `alternate_freezer`, `signal`, `terminate`, and `skip` in `freezeitDaemon/src/app/freezer_backend.rs`
- [X] T050 [US2] Implement compatible `getAppCfg` and `setAppCfg` policy handling in `freezeitDaemon/src/protocol/manager_v1.rs` and `freezeitDaemon/src/config/loader.rs`
- [X] T051 [US2] Run host freezer tests and record results in `specs/002-modern-freezer-rewrite/evidence/us2-host-tests.md`
- [X] T052 [US2] Run target-device freeze/unfreeze validation on three third-party apps and one multi-process app and record evidence in `specs/002-modern-freezer-rewrite/evidence/us2-device-freeze.md`
- [X] T053 [US2] Record the system_server bridge promotion or rejection decision with evidence in `specs/002-modern-freezer-rewrite/research.md`

**Checkpoint**: User Story 2 is complete only when freeze and unfreeze behavior meets the timing target on the real device and every fallback or skip has an operation log entry.

---

## Phase 5: User Story 3 - Preserve Safe App Classification (Priority: P1)

**Goal**: The daemon and manager preserve conservative classification for regular apps, protected apps, foreground apps, and risky system components.

**Independent Test**: Refresh the app list, inspect default classifications, change sample policies, and confirm protected/system-critical packages cannot be frozen by default or by stale UID assignment.

### Verification for User Story 3

- [X] T054 [P] [US3] Add legacy policy migration tests in `freezeitDaemon/tests/contract/policy_migration.rs`
- [X] T055 [P] [US3] Add protected package classification tests in `freezeitDaemon/tests/contract/protected_classification.rs`
- [X] T056 [P] [US3] Add UID reconciliation tests in `freezeitDaemon/tests/contract/uid_reconciliation.rs`

### Implementation for User Story 3

- [X] T057 [US3] Implement package inventory collection in `freezeitDaemon/src/app/package_inventory.rs`
- [X] T058 [US3] Implement protected defaults for manager, launcher, input method, root manager, hook manager, and system-critical packages in `freezeitDaemon/src/domain/policy.rs`
- [X] T059 [US3] Implement migration of legacy app policies, labels, and settings in `freezeitDaemon/src/config/migration.rs`
- [X] T060 [US3] Implement strict and permissive foreground classification in `freezeitDaemon/src/app/foreground.rs` and `freezeitDaemon/src/protocol/xposed.rs`
- [X] T061 [US3] Preserve manager policy serialization compatibility in `freezeitApp/app/src/main/java/io/github/jark006/freezeit/Utils.java` and `freezeitApp/app/src/main/java/io/github/jark006/freezeit/fragment/Config.java`
- [X] T062 [US3] Enforce package-name plus user-id reconciliation before UID-based control in `freezeitDaemon/src/app/controller.rs` and `freezeitDaemon/src/app/package_inventory.rs`
- [X] T063 [US3] Run target-device app list and protected classification validation and record evidence in `specs/002-modern-freezer-rewrite/evidence/us3-classification.md`
- [X] T064 [US3] Run legacy policy and label migration validation and record evidence in `specs/002-modern-freezer-rewrite/evidence/us3-migration.md`

**Checkpoint**: User Story 3 is complete only when protected defaults, permissive foreground handling, and UID-change reconciliation all pass on the target device.

---

## Phase 6: User Story 4 - Diagnose and Recover Cleanly (Priority: P2)

**Goal**: The module explains freeze, unfreeze, fallback, skip, and recovery decisions and reconciles state after daemon restart or config corruption.

**Independent Test**: Trigger normal freeze, fallback, hook-missing, config-corrupt, and daemon-restart scenarios, then verify manager logs and recovery state.

### Verification for User Story 4

- [X] T065 [P] [US4] Add structured operation log JSON contract tests in `freezeitDaemon/tests/contract/operation_log_json.rs`
- [X] T066 [P] [US4] Add recovery-after-restart integration tests in `freezeitDaemon/tests/integration/recover_after_restart.rs`
- [X] T067 [P] [US4] Add degraded-state ADB validation helper for hook, root, package inventory, freezer, network, wake-lock, and screen-state failures in `scripts/validate-degraded-state.sh`

### Implementation for User Story 4

- [X] T068 [US4] Implement persistent operation log ring buffer in `freezeitDaemon/src/app/operation_log.rs`
- [X] T069 [US4] Implement v2 diagnostic commands `getHealthReport`, `getCapabilityReport`, `getOperationLogJson`, and `runSelfCheck` in `freezeitDaemon/src/protocol/manager_v2.rs` and `freezeitDaemon/src/app/controller.rs`
- [X] T070 [US4] Update manager home and log screens for diagnostic responses in `freezeitApp/app/src/main/java/io/github/jark006/freezeit/fragment/Home.java` and `freezeitApp/app/src/main/java/io/github/jark006/freezeit/fragment/Logcat.java`
- [X] T071 [US4] Implement daemon restart reconciliation before new control actions in `freezeitDaemon/src/app/controller.rs` and `freezeitDaemon/src/app/freezer_backend.rs`
- [X] T072 [US4] Implement missing and corrupted config recovery behavior in `freezeitDaemon/src/config/loader.rs`
- [X] T073 [US4] Implement degraded hook, root, package inventory, and freezer reporting in `freezeitDaemon/src/app/health.rs`
- [ ] T074 [US4] Run fallback, hook-missing, config-corrupt, restart, network-unavailable, wake-lock-unavailable, and screen-state-unavailable validation and record evidence in `specs/002-modern-freezer-rewrite/evidence/us4-recovery.md`

**Checkpoint**: User Story 4 is complete only when every validation operation has a corresponding log entry with identity, action, result, and reason.

---

## Phase 7: User Story 5 - Maintain Long-Term Self-Use Compatibility (Priority: P3)

**Goal**: The module records compatibility baselines, detects changed runtime assumptions, and packages a release with source, metadata, manager, daemon, and validation evidence.

**Independent Test**: Run the compatibility report and release checklist on the target device and confirm the module disables unsafe paths when required capabilities are missing.

### Verification for User Story 5

- [X] T075 [P] [US5] Add compatibility baseline report contract tests in `freezeitDaemon/tests/contract/compatibility_baseline.rs`
- [X] T076 [P] [US5] Add release zip integrity validation helper in `scripts/validate-release-zip.sh`
- [X] T077 [P] [US5] Add 24-hour self-use soak checklist in `specs/002-modern-freezer-rewrite/evidence/us5-soak-checklist.md`

### Implementation for User Story 5

- [X] T078 [US5] Implement compatibility baseline collection in `freezeitDaemon/src/app/compatibility.rs` and `freezeitDaemon/src/protocol/manager_v2.rs`
- [X] T079 [US5] Implement ROM baseline capture and update flow in `scripts/capture-rom-baseline.sh` and `freezeitVS/magisk/rom_baseline.prop`
- [X] T080 [US5] Implement release packaging for Rust daemon, manager APK, Magisk metadata, and validation notes in `scripts/package-release.sh`
- [X] T081 [US5] Update module metadata, changelog, and release identity in `freezeitVS/magisk/module.prop` and `freezeitVS/magisk/changelog.txt`
- [X] T082 [US5] Update maintenance and environment notes in `README.md` and `freezeitRelease/README.md`
- [X] T083 [US5] Run full release validation for install, reboot, control, restore, and archive integrity and record evidence in `specs/002-modern-freezer-rewrite/evidence/us5-release-validation.md`
- [ ] T084 [US5] Run 24-hour self-use soak and record crash, daemon, manager, and boot observations in `specs/002-modern-freezer-rewrite/evidence/us5-24h-soak.md`

**Checkpoint**: User Story 5 is complete only when compatibility reports and release artifacts match the verified target baseline and release validation evidence is present.

---

## Phase 8: Polish & Cross-Cutting Concerns

**Purpose**: Final verification, documentation, review, convergence, and release readiness across all stories.

- [X] T085 Run final Rust, Gradle, and package build checks and record results in `specs/002-modern-freezer-rewrite/evidence/final-build.md`
- [ ] T086 Run the full quickstart validation flow and record results in `specs/002-modern-freezer-rewrite/evidence/quickstart-validation.md`
- [X] T087 [P] Update final implementation decisions and owner-visible tradeoffs in `specs/002-modern-freezer-rewrite/research.md`
- [X] T088 [P] Audit legacy C++ behavior gaps against the Rust daemon and record findings in `specs/002-modern-freezer-rewrite/evidence/legacy-gap-audit.md`
- [ ] T089 Run final aggregate `/brooks-review`, confirm per-task review evidence exists, and resolve or record every finding in `specs/002-modern-freezer-rewrite/evidence/brooks-review.md`
- [ ] T090 Run final aggregate `/speckit-converge`, confirm per-task convergence evidence exists, and resolve every blocking mismatch in `specs/002-modern-freezer-rewrite/evidence/speckit-converge.md`
- [X] T091 Prepare final release artifact notes and source archive location in `freezeitRelease/README.md`

---

## Dependencies & Execution Order

### Phase Dependencies

- **Setup (Phase 1)**: No dependencies.
- **Foundational (Phase 2)**: Depends on Phase 1 and blocks all user stories.
- **User Stories (Phases 3-7)**: Depend on Phase 2 for implementation. Device validation for US2, US3, US4, and US5 also depends on US1 readiness evidence from T038.
- **Polish (Phase 8)**: Depends on every selected user story and blocks final completion.

### User Story Dependencies

- **US1 Install and Boot Reliably (P1)**: Starts after foundation; must complete T038 before any later target-device validation is claimed.
- **US2 Freeze Background Apps (P1)**: Implementation can start after foundation; target-device freeze validation depends on US1 T038.
- **US3 Preserve Safe App Classification (P1)**: Implementation can start after foundation; target-device classification validation depends on US1 T038 and package inventory primitives.
- **US4 Diagnose and Recover Cleanly (P2)**: Implementation can start after US1 readiness and relevant US2/US3 operation paths; target-device recovery validation depends on US1 T038.
- **US5 Maintain Long-Term Compatibility (P3)**: Implementation can start after US1 readiness; final release validation depends on US2/US3/US4 evidence.

### Within Each User Story

- Verification tasks are defined first.
- Contract and integration tests should fail before implementation where practical.
- Domain and protocol changes precede controller integration.
- Device validation is the story completion gate and cannot be claimed before US1 T038 readiness evidence exists.
- A story is not complete until evidence is recorded in `specs/002-modern-freezer-rewrite/evidence/`.
- No task is complete until its scoped `/brooks-review` and `/speckit-converge` evidence has been recorded.

## Parallel Opportunities

- Setup tasks T002, T003, T004, T005, T007, and T008 can run in parallel after T001 is known.
- Foundational domain/sys parser tasks T009-T012 and T015-T021 can run in parallel, then converge into T022-T025.
- US1 verification tasks T026-T028 can run in parallel before implementation.
- US2 verification tasks T039-T041 can run in parallel before freezer implementation.
- US3 verification tasks T054-T056 can run in parallel before classification implementation.
- US4 verification tasks T065-T067 can run in parallel before diagnostic implementation.
- US5 verification tasks T075-T077 can run in parallel before release implementation.
- Final documentation/audit tasks T087 and T088 can run in parallel after implementation behavior is stable.

## Parallel Example: User Story 1

```bash
Task: "T026 [P] [US1] Add manager health protocol contract tests in freezeitDaemon/tests/contract/manager_health_v1.rs"
Task: "T027 [P] [US1] Add Magisk archive content verification script in scripts/validate-magisk-zip.sh"
Task: "T028 [P] [US1] Add install, reboot, unlock, readiness, missing LSPosed scope, and hook-inactive degraded ADB validation script in scripts/validate-install-boot.sh"
```

## Parallel Example: User Story 2

```bash
Task: "T039 [P] [US2] Add freezer backend capability and fallback decision tests, including network, wake-lock, and screen-state unavailable cases, in freezeitDaemon/tests/contract/freezer_backend_decisions.rs"
Task: "T040 [P] [US2] Add freeze/unfreeze state transition integration tests in freezeitDaemon/tests/integration/freeze_unfreeze_state.rs"
Task: "T041 [P] [US2] Add target-device freeze/unfreeze validation helper in scripts/validate-freeze-unfreeze.sh"
```

## Parallel Example: User Story 3

```bash
Task: "T054 [P] [US3] Add legacy policy migration tests in freezeitDaemon/tests/contract/policy_migration.rs"
Task: "T055 [P] [US3] Add protected package classification tests in freezeitDaemon/tests/contract/protected_classification.rs"
Task: "T056 [P] [US3] Add UID reconciliation tests in freezeitDaemon/tests/contract/uid_reconciliation.rs"
```

## Parallel Example: User Story 4

```bash
Task: "T065 [P] [US4] Add structured operation log JSON contract tests in freezeitDaemon/tests/contract/operation_log_json.rs"
Task: "T066 [P] [US4] Add recovery-after-restart integration tests in freezeitDaemon/tests/integration/recover_after_restart.rs"
Task: "T067 [P] [US4] Add degraded-state ADB validation helper for hook, root, package inventory, freezer, network, wake-lock, and screen-state failures in scripts/validate-degraded-state.sh"
```

## Parallel Example: User Story 5

```bash
Task: "T075 [P] [US5] Add compatibility baseline report contract tests in freezeitDaemon/tests/contract/compatibility_baseline.rs"
Task: "T076 [P] [US5] Add release zip integrity validation helper in scripts/validate-release-zip.sh"
Task: "T077 [P] [US5] Add 24-hour self-use soak checklist in specs/002-modern-freezer-rewrite/evidence/us5-soak-checklist.md"
```

## Implementation Strategy

### First Verified Increment: User Story 1 Only

1. Complete Phase 1 setup.
2. Complete Phase 2 foundation.
3. Complete Phase 3 User Story 1.
4. Stop and validate install, boot, unlock, manager readiness, hook readiness, daemon readiness, and fail-closed degraded behavior on the target device.
5. Record evidence before moving to freezer behavior.

### Incremental Delivery

1. Setup plus foundation produces a buildable Rust daemon skeleton.
2. US1 proves installation and readiness without app control risk.
3. US2 adds actual freeze/unfreeze behavior.
4. US3 hardens classification and migration.
5. US4 adds diagnosis and recovery.
6. US5 packages and validates long-term self-use release readiness.

### Completion Gate

The feature is not complete until final build checks, target-device validation, 24-hour soak evidence, `/brooks-review`, and `/speckit-converge` are all complete with no unresolved blocker.

## Notes

- [P] tasks must still avoid editing the same file at the same time.
- Device-facing tasks must use the verified target device serial `3B1F4LE5MS142WJY` unless a later baseline update records a new serial.
- Do not present mock, placeholder, or unvalidated freezer behavior as complete.
- Ask the owner only when a verified alternative would change daily-use behavior, compatibility scope, or accepted risk.

## Phase 9: Convergence

- [X] T092 Restore the legacy real-time chart semantics by replacing synthetic CPU line points with persistent sampled CPU history, `/proc/stat` delta usage, memory bars, and target/device tests proving successive `getRealTimeInfo` responses update over time per FR-001 and `manager-daemon-protocol.md:getRealTimeInfo` (partial)
- [X] T093 Bridge structured control operations into the manager-visible legacy log surface, or update the manager log view to show equivalent structured entries, so every freeze, unfreeze, terminate, postpone, fallback, skip, and recovery entry includes package identity, UID, process/PID context, action, result, backend/blocker, and reason per US4/AC1-2, FR-011, and SC-005 (validated in `specs/002-modern-freezer-rewrite/evidence/phase10-convergence.md`)
- [X] T094 Re-validate current-device hook readiness and control-loop gating after install, resolving any `Hook degraded` or manager-readiness mismatch so active/degraded status, `RunSelfCheck`, `xp_log`, configured app control, and `should_run_control_pass` agree before freeze behavior is claimed per US1/AC1, US2, FR-009, and FR-010 (partial)
- [ ] T095 Complete the target-observable degraded/recovery validation still open in T074, including config-corrupt recovery and safe network-unavailable, wake-lock-unavailable, and screen-state-unavailable evidence or an owner-approved safe fault-injection substitute per T074 and US4 (missing)
- [ ] T096 Complete the required 24-hour self-use soak with boot, daemon crash, manager crash, and module-attributable failure observations recorded in `specs/002-modern-freezer-rewrite/evidence/us5-24h-soak.md` per T084 and SC-004 (missing)
- [ ] T097 Complete the full quickstart validation flow after T074 and T084 are closed, recording install, reboot, activation, freeze, unfreeze, degraded hook state, release archive integrity, and unresolved risk evidence in `specs/002-modern-freezer-rewrite/evidence/quickstart-validation.md` per T086, FR-016, and SC-008 (missing)
- [X] T098 Reconcile the v2 diagnostic command IDs and compatibility-baseline endpoint across `specs/002-modern-freezer-rewrite/contracts/manager-daemon-protocol.md`, `freezeitApp/app/src/main/java/io/github/jark006/freezeit/ManagerCmd.java`, Rust `ManagerCommand`, and contract tests so FR-014 diagnostics match the published protocol per FR-014 (contradicts)
- [X] T099 Redo the legacy C++ parity audit as a command-by-command behavior matrix covering real-time chart/history, manager logs, settings semantics, CPU/RAM/battery/temperature data, UID time, process state, wake-lock/network/screen-state behavior, and known intentional deviations, correcting the prior "no additional audit task required" conclusion per T088 and Constitution V (validated in `specs/002-modern-freezer-rewrite/evidence/legacy-gap-audit.md`)

## Phase 10: Convergence

- [X] T100 Implement explicit cgroup v2 freezer capability detection by reading `/sys/fs/cgroup/cgroup.controllers`, preferring Android 16 `cgroup.freeze` paths over ROM legacy `/dev/freezer`, and recording OnePlus 13 ROM evidence from `system/etc/cgroups.json`, `system/etc/task_profiles.json`, and `system_ext/etc/init/hans.rc` in compatibility diagnostics per FR-005, FR-009, FR-014, and FR-017 (validated in `specs/002-modern-freezer-rewrite/evidence/phase10-convergence.md`)
- [X] T101 Replace hardcoded binder-freezer availability with a verified binder freezer capability probe or an explicit degraded/untested status, including target-safe evidence for `/dev/binder` or `/dev/binderfs/binder` access, ioctl support, rejection handling, and manager-visible reporting per T043, FR-005, FR-006, FR-009, and FR-010 (validated in `specs/002-modern-freezer-rewrite/evidence/phase10-convergence.md`)
- [X] T102 Route `std::io::ErrorKind::PermissionDenied` and unsafe `cgroup.freeze` write failures through the backend fallback order with structured operation logs, so permission-locked OnePlus kernel paths degrade to postpone, alternate freezer, signal, terminate, or skip instead of aborting the control pass per FR-006, FR-010, FR-011, and `freezer-backend.md` (validated in `specs/002-modern-freezer-rewrite/evidence/phase10-convergence.md`)
- [X] T103 Add a PendingFreeze idle gate before entering Frozen that uses target-observable Binder/process quiescence evidence, such as safe `/proc/binder` or ROM binderstats availability, process context-switch deltas, foreground/runtime hook state, and timeout policy, then postpones freeze when synchronous IPC risk is still high per FR-004, FR-005, SC-004, and SC-006 (validated in `specs/002-modern-freezer-rewrite/evidence/phase10-convergence.md`)
- [X] T104 Add post-freeze UID rescan and reconciliation for multi-process watchdog apps, marking partial freezes and retrying, thawing, or skipping if new same-UID processes appear during the freeze window per FR-003, FR-005, FR-011, and SC-005 (validated in `specs/002-modern-freezer-rewrite/evidence/phase10-convergence.md`)
- [X] T105 Wire manager policy delay into the live Rust control loop so selected apps pass through a real `PendingFreeze` state with foreground cancellation instead of immediate `delay_ms = 0` freezing, and validate 3-5 minute delayed freeze behavior on the target device per FR-004, FR-005, SC-002, and SC-004 (validated in `specs/002-modern-freezer-rewrite/evidence/phase10-convergence.md`)
- [X] T106 Document and validate the black-industry threat model boundary for Pinduoduo-style exploit chains, including what cgroup/SIGSTOP freezing mitigates after background control, what remains possible before freeze or from system/root components, and how protected packages, Magisk/LSPosed readiness, ROM whitelists, and release notes communicate that risk per FR-017 and SC-009 (validated in `specs/002-modern-freezer-rewrite/evidence/phase10-convergence.md`)

## Phase 11: Convergence

- [X] T107 Restore the manager-visible default log to the original C++ user-facing style, including timestamped Chinese/emoji freeze, unfreeze, launch, terminate, postpone, skip, fallback, Binder/blocker, config-change, label-update, clear-log, Xposed-log switch, and process-state check behavior in `freezeitDaemon/src/app/operation_log.rs`, `freezeitDaemon/src/protocol/manager_v1.rs`, `freezeitDaemon/src/app/controller.rs`, `freezeitDaemon/src/sys/socket.rs`, `freezeitApp/app/src/main/java/io/github/jark006/freezeit/fragment/Logcat.java`, `freezeitDaemon/tests/contract/operation_log_json.rs`, `freezeitDaemon/tests/contract/manager_health_v1.rs`, and `specs/002-modern-freezer-rewrite/evidence/phase11-legacy-log-style.md`, while keeping structured JSON diagnostics available outside the default manager log surface per FR-001, US4/AC1-2, FR-011, SC-005, and plan: existing manager log surfaces preserved (partial)
