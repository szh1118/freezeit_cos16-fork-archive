# Feature Specification: Android 16 OnePlus 13 Port

**Feature Branch**: `N/A - non-git workspace`

**Created**: 2026-07-02

**Status**: Draft

**Input**: User description: "Create an Android 16 self-use runnable build for this Magisk module, targeting only the ROM archive at `/home/admin/code/Rom/oneplus13.zip`. Use the local ROM unpacking workflow based on `/home/admin/code/MIO-KITCHEN-SOURCE`; user-level Android development skills and `adevtool` should be available for later implementation."

## Clarifications

### Session 2026-07-03

- Q: What should happen if the installed phone build differs from the ROM archive? → A: Allow service startup and control operations, but record the mismatch only as a warning in logs.
- Q: Which app states must be protected from freezing? → A: Unselected system apps, the current foreground app, media playback, calls, audio recording, and screen recording must be protected.
- Q: When may app control operations begin after boot? → A: App control operations may begin only after first user unlock and after the module service and hook state are available.
- Q: What recovery path is required if the module causes instability? → A: The user must be able to disable or uninstall the module through the root/module manager.

## User Scenarios & Testing *(mandatory)*

### User Story 1 - Installable Single-ROM Build (Priority: P1)

As the device owner, I want a build of the module that installs and starts on my OnePlus 13 Android 16 ROM so I can use the existing freezing workflow again on my own phone.

**Why this priority**: If the module cannot be installed, rebooted, and started on the target ROM, no other compatibility work has value.

**Independent Test**: Can be tested by installing the produced module package on the target phone, rebooting once, opening the manager app, and confirming the service reports a healthy running state.

**Acceptance Scenarios**:

1. **Given** the target phone is running the ROM represented by `/home/admin/code/Rom/oneplus13.zip`, **When** the produced module package is installed and the phone reboots, **Then** the phone reaches the launcher without bootloop or recovery intervention.
2. **Given** the phone has completed boot and the user has unlocked it, **When** the module service and hook state become available, **Then** the manager app shows module status, version, and runtime logs without crashing.
3. **Given** the module cannot safely start on the target ROM for a reason other than an installed-build mismatch, **When** the service detects an unsupported runtime condition, **Then** it must fail closed with visible diagnostics rather than leaving apps frozen or causing repeated system crashes.

---

### User Story 2 - Core Freeze/Unfreeze Works (Priority: P2)

As the device owner, I want selected background apps to be paused and later restored correctly so I can reduce background activity without breaking foreground app use.

**Why this priority**: The core value of the project is controlled background suspension; install success alone is insufficient.

**Independent Test**: Can be tested by selecting a small set of non-critical third-party apps, sending them to the background, confirming they stop background activity, then bringing them foreground again and confirming they resume normally.

**Acceptance Scenarios**:

1. **Given** three non-critical third-party apps are configured for freezing, **When** each app leaves the foreground, **Then** the module marks it as controlled and applies the configured background state within 30 seconds.
2. **Given** a controlled app is brought back to the foreground, **When** the user opens the app from launcher or recent tasks, **Then** it becomes usable again within 5 seconds without requiring a manual force stop.
3. **Given** a controlled app cannot be safely paused on this ROM, **When** the module detects the failure, **Then** it records the failure and leaves that app in a safe usable state.

---

### User Story 3 - Target-ROM Compatibility Evidence (Priority: P3)

As the device owner and maintainer, I want the work to produce clear evidence of what was checked against my ROM so future changes can be made without guessing.

**Why this priority**: This project depends on ROM-specific behavior; a documented compatibility baseline reduces regressions and avoids accidental scope expansion.

**Independent Test**: Can be tested by reviewing the generated compatibility notes and validation log after the build is tested on the phone.

**Acceptance Scenarios**:

1. **Given** the ROM archive is available locally, **When** compatibility analysis is performed, **Then** the output identifies the target ROM build, Android version, device family, and relevant framework/runtime compatibility observations.
2. **Given** validation has been run on the phone, **When** logs are collected, **Then** the report separates passing checks, degraded features, and unsafe or unverified areas.
3. **Given** another ROM or device is requested later, **When** the current specification is reviewed, **Then** it clearly states that those targets are out of scope for this feature.

### Edge Cases

- The ROM archive is missing, corrupted, encrypted, or cannot be unpacked enough to establish a compatibility baseline.
- The target phone has a different installed build than the provided ROM archive; this condition must be logged as a warning, but service startup and control operations remain allowed.
- The root manager, module environment, or hook environment is missing, disabled, or incompatible on the phone.
- The module starts before storage, user unlock, or framework hooks are ready; app control operations must wait until first user unlock and service/hook readiness.
- A system app is not explicitly selected for freezing; system apps are unselected by default and must remain protected unless the user opts them in.
- A controlled app is currently foreground, playing media, in a call, recording audio, or recording the screen, and must not be interrupted.
- A freeze action partially succeeds and must be rolled back to avoid leaving a user app unusable.
- System security policy blocks a required runtime action and the module must degrade safely.
- Reboot or module removal through the root/module manager must not leave persistent frozen state.

## Requirements *(mandatory)*

### Functional Requirements

- **FR-001**: The feature MUST produce a self-use module build intended only for the OnePlus 13 Android 16 ROM represented by `/home/admin/code/Rom/oneplus13.zip`.
- **FR-002**: The feature MUST explicitly exclude support claims for other devices, ROMs, Android versions, or public release channels.
- **FR-003**: The feature MUST verify the provided ROM archive sufficiently to establish that the build target matches the intended Android 16 OnePlus 13 environment.
- **FR-004**: The feature MUST provide an installable package containing the module service, manager app, and required metadata for the target phone's module environment.
- **FR-005**: The feature MUST preserve the user's existing app configuration when upgrading from an older installed version, when such configuration is present.
- **FR-006**: The feature MUST start app control operations only after first user unlock and after the module service and hook state are available.
- **FR-007**: The feature MUST expose visible status and diagnostic logs in the manager app or an accessible log file.
- **FR-008**: The feature MUST support selecting controlled apps and applying the existing freeze, unfreeze, and safe fallback behavior on the target ROM.
- **FR-009**: The feature MUST avoid freezing or disrupting apps that are currently foreground or otherwise identified as user-active by the compatibility checks.
- **FR-010**: The feature MUST detect failed or unsupported control operations and leave affected apps in a usable state.
- **FR-011**: The feature MUST allow the user to disable or uninstall the module through the root/module manager if it causes instability.
- **FR-012**: The feature MUST produce a target-ROM compatibility note that records what was validated, what is degraded, and what remains unverified.
- **FR-013**: If the installed phone build differs from the provided ROM archive, the feature MUST allow service startup and control operations while recording the mismatch as a diagnostic warning.
- **FR-014**: The feature MUST leave system apps unselected for freezing by default and MUST NOT freeze an unselected system app.
- **FR-015**: The feature MUST NOT freeze an app that is currently foreground, playing media, in a call, recording audio, or recording the screen.

### Key Entities *(include if feature involves data)*

- **Target ROM Baseline**: Represents the single supported system image source, including device family, Android version, build identity, and compatibility observations.
- **Self-Use Module Build**: Represents the produced installable artifact for the user's target phone, including version, package contents, and scope limitations.
- **Controlled App**: Represents an app selected by the user for background control, with label, package identity, runtime state, configured control behavior, and protected-state indicators for foreground use, media playback, calls, audio recording, and screen recording.
- **Validation Result**: Represents evidence from install, boot, service startup, freeze/unfreeze, recovery, and diagnostic checks.
- **Recovery State**: Represents conditions and actions needed to safely disable or uninstall the module through the root/module manager if the target phone becomes unstable.

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: The target phone completes 3 consecutive reboots after module installation without bootloop, forced recovery, or manual module removal.
- **SC-002**: The manager app opens and displays module status within 60 seconds after the first post-install unlock.
- **SC-003**: At least 3 user-selected non-critical third-party apps can be controlled in the background and restored to usable foreground state with a 100% pass rate during validation.
- **SC-004**: A controlled app returns to user-interactive state within 5 seconds in at least 9 out of 10 foreground restore attempts.
- **SC-005**: The module records actionable diagnostics for every failed compatibility or control operation observed during validation.
- **SC-006**: The user can disable or uninstall the module through the root/module manager and regain normal phone operation within 10 minutes without data loss.

## Assumptions

- The target device is a OnePlus 13 running the same Android 16 ROM represented by `/home/admin/code/Rom/oneplus13.zip`.
- The user accepts a self-use build with no compatibility promise for other ROMs, devices, or public distribution.
- The phone has a working root/module environment and hook environment available before final device validation begins.
- ROM inspection will use the approved local ROM unpacking workflow already present on this machine.
- Newly installed user-level Android support skills and `adevtool` are preparation for the planning and implementation phases, not standalone acceptance criteria.
- Validation will use non-critical third-party apps selected by the user or maintainer to avoid risking data loss in essential apps.
