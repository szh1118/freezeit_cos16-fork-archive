# Feature Specification: Modern Freezer Rewrite

**Feature Branch**: `002-modern-freezer-rewrite`

**Created**: 2026-07-03

**Status**: Draft

**Input**: User selected the full rewrite direction for a long-term maintained Freezeit COS16 adaptation, prioritizing a safer daemon architecture, clearer freeze state management, better Android 16 system-freezer cooperation, and reduced legacy behavior inherited from older Android/Magisk/Xposed generations.

## Clarifications

### Session 2026-07-03

- Q: How should technical uncertainty be handled when the owner is not familiar with Magisk, Android internals, Xposed, C++, or Rust? → A: The implementer must resolve technical uncertainty through authoritative documentation, source review, and device evidence; ask the owner only when a verified better baseline alternative would change daily-use behavior, compatibility scope, or risk posture.

## User Scenarios & Testing *(mandatory)*

### User Story 1 - Install and Boot Reliably (Priority: P1)

As the device owner, I need the rewritten module to install, boot, and report a healthy active state on the target self-use device so I can keep using the app-freezing workflow without manual recovery after each reboot.

**Why this priority**: If installation or boot startup is unreliable, every other freezer improvement is unusable and could risk a boot loop or broken manager state.

**Independent Test**: Can be fully tested by installing the module on the target device, rebooting, unlocking once, opening the manager, and confirming module health, hook readiness, and daemon readiness are all reported without requiring shell intervention.

**Acceptance Scenarios**:

1. **Given** the target device has the required root and hook environment enabled, **When** the rewritten module is installed and the device reboots, **Then** the manager reports the module as active and ready within 30 seconds after unlock.
2. **Given** the hook environment is not active or the required scope is missing, **When** the manager is opened, **Then** the module reports a clear inactive state and does not attempt destructive app control.
3. **Given** the module starts before user storage is fully available, **When** the device finishes boot and unlock, **Then** the module recovers automatically and loads the persisted app policies.

---

### User Story 2 - Freeze Background Apps Using Modern System Semantics (Priority: P1)

As the device owner, I need selected background apps to be frozen through the most compatible mechanism for the target Android 16/COS16 environment so that apps stop doing background work while foreground use remains smooth.

**Why this priority**: This is the core value of Freezeit; the rewrite must improve the freeze path without regressing the actual user-facing behavior.

**Independent Test**: Can be tested by selecting several third-party apps, moving them between foreground and background, and verifying that each app transitions to the intended controlled state and returns to usable foreground state without crash, ANR, or stale frozen state.

**Acceptance Scenarios**:

1. **Given** an app is configured for freezing and leaves the foreground, **When** its configured delay expires and the app is safe to control, **Then** the module freezes the app and records the reason and affected processes.
2. **Given** a frozen app is opened by the user, **When** it becomes foreground or otherwise user-visible, **Then** the module unfreezes it before the user observes a broken UI.
3. **Given** the preferred freeze mechanism is unavailable or rejects a target process, **When** the module attempts control, **Then** it uses the next safe fallback or postpones the operation without leaving the app half-controlled.

---

### User Story 3 - Preserve Safe App Classification (Priority: P1)

As the device owner, I need the module to distinguish regular apps, protected apps, foreground apps, and risky system components so that aggressive freezing does not destabilize the ROM.

**Why this priority**: The target environment is a daily-use phone; freezing the wrong process can cause missed notifications, broken services, or system instability.

**Independent Test**: Can be tested by refreshing the app list, checking default classifications, changing several app policies, and confirming protected/system-critical apps remain protected unless explicitly and safely allowed.

**Acceptance Scenarios**:

1. **Given** a package is a launcher, input method, root manager, module manager, or other trusted component, **When** the app list is generated, **Then** it is protected from normal freeze policies by default.
2. **Given** an app has visible UI, foreground service, persistent notification, overlay, or audio activity, **When** permissive foreground recognition is enabled for that app, **Then** the module does not freeze it until it no longer matches those conditions.
3. **Given** a package UID changes after update or reinstall, **When** policies are loaded, **Then** the module reconciles policies by package identity and does not apply stale UID control to the wrong app.

---

### User Story 4 - Diagnose and Recover Cleanly (Priority: P2)

As the device owner, I need clear logs, health checks, and recovery behavior so I can understand why an app was frozen, skipped, unfrozen, postponed, or killed.

**Why this priority**: A deep system module will occasionally hit ROM-specific behavior; diagnostics are required for safe self-maintenance.

**Independent Test**: Can be tested by triggering normal freeze, fallback, hook-missing, config-corrupt, and daemon-restart scenarios, then verifying that manager logs explain the outcome and recovery path.

**Acceptance Scenarios**:

1. **Given** an app control operation succeeds, **When** logs are viewed, **Then** the log includes package identity, UID, process count, action, and reason.
2. **Given** an operation is skipped or postponed, **When** logs are viewed, **Then** the log identifies the blocker and the next planned action.
3. **Given** the daemon restarts while apps are controlled, **When** it comes back online, **Then** it reconciles current process state before issuing new freeze or unfreeze actions.

---

### User Story 5 - Maintain Long-Term Self-Use Compatibility (Priority: P3)

As the maintainer, I need the rewritten module to have explicit compatibility checks and verification evidence so future ROM or root-framework updates do not silently break app control.

**Why this priority**: The selected direction is a long-term rewrite; maintainability and verifiability are part of the requested outcome.

**Independent Test**: Can be tested by running the compatibility report and release validation checklist on the target device and confirming the module records the device baseline, feature availability, and any unsupported conditions before packaging.

**Acceptance Scenarios**:

1. **Given** the target ROM or root-framework baseline changes, **When** the module starts, **Then** it records the difference and continues only when required capabilities are still available.
2. **Given** a required capability is missing, **When** the module starts, **Then** it disables unsafe control paths and reports the missing capability.
3. **Given** a release package is produced, **When** validation is run, **Then** the package includes source, manager, daemon, module metadata, and verification notes for the target environment.

### Edge Cases

- Hook framework active in the manager but not in the system process.
- Required hook scope missing after manager reinstall or LSPosed database reset.
- App process exits between discovery and freeze operation.
- Multi-process app has a mix of safe and unsafe-to-freeze processes.
- A controlled app receives synchronous or asynchronous IPC while frozen.
- The preferred freezer path exists but rejects the process.
- Network break, wake-lock suppression, or screen-state detection is unavailable.
- Config file is missing, corrupted, or from an older module version.
- Device boots before user storage, package manager, or hook service is ready.
- The target ROM reports a changed fingerprint or build increment.
- A protected package is updated and its UID changes.
- Manager app itself is cached, pending freeze, or restarted.

## Requirements *(mandatory)*

### Functional Requirements

- **FR-001**: System MUST preserve the current user workflow for installing the module, enabling the hook scope, opening the manager, configuring per-app policies, viewing logs, and rebooting into an active state.
- **FR-002**: System MUST maintain a persistent policy for each managed package, including whether it is protected, free, terminable, freezable, or freezable with additional restrictions.
- **FR-003**: System MUST discover currently running processes for managed packages and reconcile them with package identity before applying any control action.
- **FR-004**: System MUST distinguish foreground, visible, permissive-foreground, pending-control, frozen, and unknown runtime states for each managed app.
- **FR-005**: System MUST prefer the safest supported freeze path for the current device baseline and MUST verify that a target process is eligible before freezing it.
- **FR-006**: System MUST provide a fallback decision when the preferred freeze path is unavailable, rejected, or unsafe, including postpone, alternate control, terminate, or skip.
- **FR-007**: System MUST unfreeze a controlled app when it returns to foreground or user-visible use.
- **FR-008**: System MUST avoid freezing protected apps by default, including the manager, launcher, input method, root manager, hook manager, and system-critical components.
- **FR-009**: System MUST detect hook readiness, daemon readiness, root readiness, package inventory readiness, and required freezer capability before enforcing app control.
- **FR-010**: System MUST fail closed when readiness checks fail, meaning it must report degraded status and avoid unsafe freeze or terminate actions.
- **FR-011**: System MUST record every freeze, unfreeze, terminate, postpone, fallback, skip, and recovery decision with enough context to diagnose the reason.
- **FR-012**: System MUST recover from daemon restart by reading current process state before issuing new control actions.
- **FR-013**: System MUST migrate existing app labels, app policies, and settings when upgrading from the current module format.
- **FR-014**: System MUST provide a compatibility report for the target device baseline, including detected ROM/build identity, root environment, hook readiness, freezer capability, and known degraded paths.
- **FR-015**: System MUST package a complete release artifact that installs the manager and daemon together and preserves required module metadata.
- **FR-016**: System MUST support manual validation of install, reboot, manager activation, app freeze, app unfreeze, degraded hook state, and release artifact integrity.
- **FR-017**: Implementation decisions about Magisk, Android internals, hook framework behavior, native language choices, and low-level freezer interfaces MUST be resolved by documented research and runtime evidence rather than owner technical input, except when choosing a verified alternative would materially affect daily-use behavior, compatibility scope, or accepted risk.

### Key Entities *(include if feature involves data)*

- **Managed App**: A package known to the module, identified by package name, current UID, display label, protection status, and user policy.
- **Freeze Policy**: The user's selected behavior for a managed app, including protected/free/freeze/terminate behavior and strict or permissive foreground recognition.
- **Runtime Process**: A currently running process belonging to a managed app, including PID, UID, package/process name, foreground classification, and current control state.
- **Control Capability**: A device-specific capability that indicates whether a freeze, unfreeze, fallback, network restriction, wake-lock restriction, or state query path is available and safe.
- **Control Operation**: A recorded attempt to freeze, unfreeze, terminate, postpone, skip, or recover an app process, including result and reason.
- **Module Health**: The module's active/degraded/inactive status across manager, daemon, hook readiness, root access, package inventory, and freezer capability.
- **Compatibility Baseline**: The target device and ROM identity used to decide whether the release is operating in its validated environment.

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: After installation and reboot on the target device, the manager shows active module, daemon, and hook readiness within 30 seconds after first unlock.
- **SC-002**: At least 95% of selected third-party app background transitions result in the configured controlled state within the configured delay plus 5 seconds.
- **SC-003**: At least 95% of controlled apps return to foreground usability within 2 seconds of user launch during validation.
- **SC-004**: A 24-hour self-use validation run completes with no boot loop, no daemon crash, and no manager crash attributable to the module.
- **SC-005**: Every validation freeze/unfreeze/fallback operation has a corresponding log entry containing package identity, UID, action, result, and reason.
- **SC-006**: Compatibility checks identify missing hook, root, package inventory, or freezer capability before any unsafe app-control action is attempted.
- **SC-007**: Existing app policies and labels from the current module are preserved after upgrade with no wrong-package policy assignment in validation.
- **SC-008**: Release validation confirms the packaged module installs, reboots, activates, controls a test app, restores that test app, and passes archive integrity checks.
- **SC-009**: Planning and release notes cite evidence for selected system, root, hook, and freezer interfaces, and record any owner-approved tradeoff decisions that affect daily-use behavior or risk.

## Assumptions

- The primary target is the owner's currently verified self-use device and ROM baseline, not broad public compatibility across unrelated devices.
- The manager app remains the user's primary configuration and diagnostic surface.
- Existing app policies, labels, settings, and release packaging behavior are expected to be preserved unless a later approved plan explicitly changes them.
- Protected package defaults are conservative; third-party app control is prioritized over system-component control.
- Existing implementation behavior can be used as a behavioral reference, but the rewritten control system is allowed to replace legacy internals when the user-facing workflow and safety guarantees are preserved.
- Technical choices for the daemon language, build tooling, hook call sites, and low-level freezer implementation will be finalized in the planning phase using authoritative documentation, source review, and target-device evidence.
- The owner is expected to decide user-facing tradeoffs, not low-level implementation details.
