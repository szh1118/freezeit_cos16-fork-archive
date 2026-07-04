# Tasks: Android 16 OnePlus 13 Port

**Input**: Design documents from `specs/001-android16-oneplus13-port/`

**Prerequisites**: [plan.md](./plan.md), [spec.md](./spec.md), [research.md](./research.md), [data-model.md](./data-model.md), [quickstart.md](./quickstart.md), [contracts/](./contracts/)

**Verification**: Required for every user-facing and system-facing behavior by the Freezeit constitution. Automated tests are used where practical; ROM/device behavior is verified with concrete manual evidence.

**Organization**: Tasks are grouped by user story so each story can be implemented and validated as an independent increment after foundational work.

## Phase 1: Setup (Shared Infrastructure)

**Purpose**: Establish the local build, ROM, and evidence workspace used by all stories.

- [X] T001 Create validation artifact directory and README in specs/001-android16-oneplus13-port/validation/README.md
- [X] T002 Record local toolchain facts for Gradle, NDK, adevtool, ROM archive, and MIO in specs/001-android16-oneplus13-port/validation/environment.md
- [X] T003 [P] Create a Linux native compile helper for ARM64 using /home/admin/Android/Sdk/ndk/28.2.13676358 in freezeitVS/build_arm64_linux.sh
- [X] T004 [P] Create a Linux package helper skeleton that validates input paths and prepares the Magisk tree in freezeitVS/build_pack_linux.sh
- [X] T005 [P] Create compatibility evidence template from contracts/compatibility-evidence.md in specs/001-android16-oneplus13-port/validation/compatibility-note.md
- [X] T006 [P] Create device validation log template from contracts/compatibility-evidence.md in specs/001-android16-oneplus13-port/validation/device-validation-log.md

---

## Phase 2: Foundational (Blocking Prerequisites)

**Purpose**: Complete ROM baseline research, build pipeline readiness, and interface mapping before any user story implementation.

**Critical**: No user story implementation should begin until this phase is complete.

- [X] T007 Repair or document the MIO-KITCHEN-SOURCE Python dependency issue (`No module named 'google'`) in specs/001-android16-oneplus13-port/validation/mio-setup.md
- [X] T008 Unpack enough of /home/admin/code/Rom/oneplus13.zip to collect build properties and record command/output paths in specs/001-android16-oneplus13-port/validation/compatibility-note.md
- [X] T009 Extract target ROM build identity, Android version, product/device family, fingerprint, and security patch into specs/001-android16-oneplus13-port/validation/compatibility-note.md and freezeitVS/magisk/rom_baseline.prop
- [X] T010 Inspect target ROM framework artifacts for ActivityManager, broadcast, AppOps/wakelock, display/screen, foreground UID, pending UID, media/call/recording, and screen-recording compatibility in specs/001-android16-oneplus13-port/validation/android16-hook-map.md
- [X] T011 Map existing manager/native command IDs from freezeitApp/app/src/main/java/io/github/jark006/freezeit/ManagerCmd.java to native handling in freezeitVS/include/server.hpp and record compatibility in specs/001-android16-oneplus13-port/validation/manager-native-protocol-check.md
- [X] T012 Verify existing upgrade config preservation paths for appcfg.txt, applabel.txt, and settings.db in freezeitVS/magisk/customize.sh and record result in specs/001-android16-oneplus13-port/validation/config-preservation.md
- [X] T013 Run baseline manager APK build with `bash freezeitApp/gradlew :app:assembleRelease` and record output in specs/001-android16-oneplus13-port/validation/build-log.md
- [X] T014 Run baseline native ARM64 compile through freezeitVS/build_arm64_linux.sh and record output in specs/001-android16-oneplus13-port/validation/build-log.md
- [X] T015 Confirm the generated module package contains module.prop, service.sh, customize.sh, uninstall.sh, rom_baseline.prop, ARM64 native binary, manager APK, and config seed files in specs/001-android16-oneplus13-port/validation/package-inspection.md

**Checkpoint**: Foundation ready. ROM baseline, hook map, local build path, package inspection, and protocol mapping exist.

---

## Phase 3: User Story 1 - Installable Single-ROM Build (Priority: P1)

**Goal**: Produce an installable self-use module package that boots, starts after first unlock, exposes manager status/logs, and fails closed for unsupported runtime conditions other than build mismatch.

**Independent Test**: Install the produced package on the target phone, reboot once, unlock, open the manager app, and confirm service health, status, version, and logs without bootloop or crash.

### Verification for User Story 1

- [X] T016 [P] [US1] Define install, first unlock, service readiness, hook readiness, manager status, and log evidence fields in specs/001-android16-oneplus13-port/validation/device-validation-log.md
- [X] T017 [P] [US1] Define package inspection pass/fail criteria for self-use scope, module metadata, service script, manager APK, and native binary in specs/001-android16-oneplus13-port/validation/package-inspection.md
- [X] T018 [US1] Record target device root/module manager and LSPosed/Xposed prerequisites in specs/001-android16-oneplus13-port/validation/device-validation-log.md

### Implementation for User Story 1

- [X] T019 [US1] Update Linux packaging flow to copy the release manager APK into the Magisk package in freezeitVS/build_pack_linux.sh
- [X] T020 [US1] Update Linux packaging flow to produce a self-use target-ROM zip name and output path in freezeitVS/build_pack_linux.sh
- [X] T021 [US1] Ensure post-unlock startup still waits for storage, user unlock, and disable/remove flags in freezeitVS/magisk/service.sh
- [X] T022 [US1] Compare packaged ROM baseline metadata from freezeitVS/magisk/rom_baseline.prop with device getprop values and log installed-build mismatch warnings without blocking service startup or control operations in freezeitVS/magisk/service.sh and freezeitVS/include/freezeit.hpp
- [X] T023 [US1] Implement an app-control hook-readiness gate before freeze/control operations and preserve existing getPropInfo field order while exposing readiness diagnostics through logs in freezeitVS/include/freezer.hpp and freezeitVS/include/server.hpp
- [X] T024 [US1] Build the manager APK and native package, then record artifact paths and hashes in specs/001-android16-oneplus13-port/validation/package-inspection.md
- [ ] T025 [US1] Install the package on the target phone through the root/module manager and record install result in specs/001-android16-oneplus13-port/validation/device-validation-log.md
- [ ] T026 [US1] Reboot once, unlock, open manager status, and record service health/version/log result within 60 seconds in specs/001-android16-oneplus13-port/validation/device-validation-log.md
- [ ] T027 [US1] Run 3 consecutive reboot validations and record launcher reachability, bootloop absence, and recovery intervention absence in specs/001-android16-oneplus13-port/validation/device-validation-log.md

**Checkpoint**: US1 is independently installable and boot-safe on the target phone.

---

## Phase 4: User Story 2 - Core Freeze/Unfreeze Works (Priority: P2)

**Goal**: Selected non-critical third-party apps can be controlled in the background and restored to foreground usability while protected states are not frozen.

**Independent Test**: Select three non-critical third-party apps, send each to background, confirm control within 30 seconds, reopen each, and confirm restoration within 5 seconds.

### Verification for User Story 2

- [X] T028 [P] [US2] Define the 3-app freeze/restore validation matrix with timing fields in specs/001-android16-oneplus13-port/validation/freeze-restore-matrix.md
- [X] T029 [P] [US2] Define protected-state checks for unselected system apps, foreground app, media playback, calls, audio recording, and screen recording in specs/001-android16-oneplus13-port/validation/protected-state-matrix.md
- [X] T030 [P] [US2] Define manager/native protocol exercises for getAppCfg, setAppCfg, getLog, getXpLog, and printFreezerProc in specs/001-android16-oneplus13-port/validation/manager-native-protocol-check.md

### Implementation for User Story 2

- [X] T031 [US2] Verify system apps remain whitelist/unselected by default and adjust default classification only if ROM evidence requires it in freezeitVS/include/managedApp.hpp
- [X] T032 [US2] Verify foreground UID protection against the Android 16 hook map and update hook enum or method signatures if needed in freezeitApp/app/src/main/java/io/github/jark006/freezeit/hook/Enum.java
- [X] T033 [US2] Verify foreground UID tracking implementation against the Android 16 hook map and update hook logic if needed in freezeitApp/app/src/main/java/io/github/jark006/freezeit/hook/android/FreezeitService.java
- [X] T034 [US2] Verify broadcast/pending UID protection against the Android 16 hook map and update hook logic if needed in freezeitApp/app/src/main/java/io/github/jark006/freezeit/hook/android/BroadCastHook.java
- [X] T035 [US2] Verify media playback detection on the target ROM and update audio-state handling if needed in freezeitVS/include/systemTools.hpp
- [X] T036 [US2] Identify ROM-derived signal sources for call, audio recording, and screen recording protection and record required Java hook or native detector changes in specs/001-android16-oneplus13-port/validation/android16-hook-map.md
- [X] T037 [US2] Implement or verify call, audio recording, and screen recording protection using the ROM-derived signal sources in freezeitApp/app/src/main/java/io/github/jark006/freezeit/hook/android/FreezeitService.java and freezeitVS/include/systemTools.hpp
- [X] T038 [US2] Ensure freeze/unfreeze skips protected apps and logs actionable diagnostics in freezeitVS/include/freezer.hpp
- [X] T039 [US2] Ensure failed or unsupported control operations leave affected apps usable and produce manager-visible diagnostics in freezeitVS/include/freezer.hpp
- [ ] T040 [US2] Exercise getAppCfg and setAppCfg from the manager app and record config preservation/control results in specs/001-android16-oneplus13-port/validation/manager-native-protocol-check.md
- [ ] T041 [US2] Run the 3-app freeze/restore validation and record all timing/results in specs/001-android16-oneplus13-port/validation/freeze-restore-matrix.md
- [ ] T042 [US2] Run protected-state validation for unselected system apps, foreground, media playback, calls, audio recording, and screen recording in specs/001-android16-oneplus13-port/validation/protected-state-matrix.md

**Checkpoint**: US2 controls selected apps and avoids protected states on the target phone.

---

## Phase 5: User Story 3 - Target-ROM Compatibility Evidence (Priority: P3)

**Goal**: Produce clear evidence of ROM baseline, validation results, degraded behavior, unsafe behavior, and out-of-scope targets.

**Independent Test**: Review the compatibility note and validation log and confirm they identify the ROM build, Android version, device family, checked framework/runtime areas, passing checks, degraded features, unsafe or unverified areas, and explicit out-of-scope targets.

### Verification for User Story 3

- [X] T043 [P] [US3] Define review checklist for compatibility-note required sections in specs/001-android16-oneplus13-port/validation/compatibility-note-review.md
- [X] T044 [P] [US3] Define review checklist for device-validation-log required sections in specs/001-android16-oneplus13-port/validation/device-validation-review.md

### Implementation for User Story 3

- [X] T045 [US3] Complete ROM archive, build identity, unpacking method, framework artifacts inspected, and scope statement in specs/001-android16-oneplus13-port/validation/compatibility-note.md
- [X] T046 [US3] Complete hook compatibility observations and runtime assumptions in specs/001-android16-oneplus13-port/validation/compatibility-note.md
- [ ] T047 [US3] Record build mismatch comparison between ROM baseline and installed phone build as warning-only evidence in specs/001-android16-oneplus13-port/validation/device-validation-log.md
- [X] T048 [US3] Separate PASS, DEGRADED, FAIL, and UNVERIFIED validation results in specs/001-android16-oneplus13-port/validation/device-validation-log.md
- [ ] T049 [US3] Record diagnostics evidence for every failed compatibility or control operation in specs/001-android16-oneplus13-port/validation/device-validation-log.md
- [ ] T050 [US3] Validate root/module manager disable or uninstall recovery within 10 minutes and record result in specs/001-android16-oneplus13-port/validation/device-validation-log.md
- [X] T051 [US3] Review compatibility-note completeness against specs/001-android16-oneplus13-port/validation/compatibility-note-review.md
- [X] T052 [US3] Review device-validation-log completeness against specs/001-android16-oneplus13-port/validation/device-validation-review.md

**Checkpoint**: US3 evidence is complete enough for future maintenance without guessing.

---

## Phase 6: Polish & Cross-Cutting Concerns

**Purpose**: Final cleanup, repeatable validation, and project-required completion gates.

- [X] T053 [P] Update self-use scope and validation notes in freezeitVS/changelog.txt
- [X] T054 [P] Update release-side self-use scope and validation notes in freezeitRelease/changelog.txt
- [X] T055 [P] Update quickstart command results and any corrected paths in specs/001-android16-oneplus13-port/quickstart.md
- [X] T056 Run package inspection, manager APK build, native compile, and artifact checks from specs/001-android16-oneplus13-port/quickstart.md and record final results in specs/001-android16-oneplus13-port/validation/final-verification.md
- [X] T057 Run `/brooks-review` and record findings plus resolutions or explicit human acceptance in specs/001-android16-oneplus13-port/validation/final-verification.md
- [X] T058 Run `/speckit-converge` and record findings plus resolutions or explicit human acceptance in specs/001-android16-oneplus13-port/validation/final-verification.md
- [ ] T059 Confirm no PASS/DEGRADED/FAIL/UNVERIFIED table leaves core install, boot, service, freeze/unfreeze, protected-state, diagnostics, or recovery checks unverified in specs/001-android16-oneplus13-port/validation/final-verification.md

---

## Dependencies & Execution Order

### Phase Dependencies

- **Setup (Phase 1)**: No dependencies.
- **Foundational (Phase 2)**: Depends on Setup; blocks all user stories.
- **US1 (Phase 3)**: Depends on Foundational; first verified increment.
- **US2 (Phase 4)**: Depends on Foundational and benefits from US1 package/install path.
- **US3 (Phase 5)**: Depends on Foundational; final evidence should incorporate US1 and US2 validation.
- **Polish (Phase 6)**: Depends on selected user stories being complete.

### User Story Dependencies

- **US1**: Can start after Foundational and does not depend on US2/US3.
- **US2**: Can start after Foundational, but device validation is cleaner after US1 produces an installable package.
- **US3**: Can start evidence drafting after Foundational, but completion depends on US1 and US2 validation evidence.

### Parallel Opportunities

- Setup tasks T003-T006 can run in parallel after T001/T002 ownership is clear.
- Foundational evidence tasks T011-T012 can run while ROM unpacking tasks T008-T010 progress.
- US1 verification-definition tasks T016-T018 can run in parallel.
- US2 verification-definition tasks T028-T030 can run in parallel.
- US3 review checklist tasks T043-T044 can run in parallel.
- Polish documentation tasks T053-T055 can run in parallel.

## Parallel Example: User Story 2

```bash
Task: "T028 [P] [US2] Define the 3-app freeze/restore validation matrix with timing fields in specs/001-android16-oneplus13-port/validation/freeze-restore-matrix.md"
Task: "T029 [P] [US2] Define protected-state checks for unselected system apps, foreground app, media playback, calls, audio recording, and screen recording in specs/001-android16-oneplus13-port/validation/protected-state-matrix.md"
Task: "T030 [P] [US2] Define manager/native protocol exercises for getAppCfg, setAppCfg, getLog, getXpLog, and printFreezerProc in specs/001-android16-oneplus13-port/validation/manager-native-protocol-check.md"
```

## Implementation Strategy

### First Verified Increment

1. Complete Phase 1 and Phase 2.
2. Complete US1 package/install/startup tasks.
3. Stop and validate US1 independently on the target phone.
4. Record evidence before moving to US2.

### Incremental Delivery

1. US1 delivers installable, boot-safe module package and manager status.
2. US2 delivers freeze/unfreeze behavior and protected-state behavior.
3. US3 delivers maintainable compatibility evidence and validation records.
4. Final polish runs package/build validation plus `/brooks-review` and `/speckit-converge`.

### Scope Control

- Do not add public release support.
- Do not add support for other ROMs, devices, or Android versions.
- Do not create new manager/native interfaces unless existing command IDs cannot satisfy verified requirements.
- Do not claim completion while core validation entries remain `FAIL` or `UNVERIFIED` without explicit human acceptance.
