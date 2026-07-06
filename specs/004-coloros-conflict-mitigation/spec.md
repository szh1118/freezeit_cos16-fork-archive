# Feature Specification: ColorOS Conflict Mitigation

**Feature Branch**: `main`
**Created**: 2026-07-06
**Status**: Ready for Planning
**Input**: "Handle the power waste caused by OPPO/ColorOS power management fighting Freezeit, using the full spec-kit flow."

## User Scenarios & Testing

### User Story 1 - Athena Cleanup Does Not Fight Managed Freezing (Priority: P1)

As a Freezeit user on ColorOS, I want the ColorOS background cleanup service to stop killing or force-stopping apps behind Freezeit's back so that Freezeit does not repeatedly freeze apps after Athena wakes, unfreezes, or cleans them.

**Why this priority**: The reverse-engineering report identified `com.oplus.athena` as the package-level policy brain and the current source has no OPPO hook.

**Independent Test**: Load the hook source and verify `com.oplus.athena` is in both legacy and modern Xposed scopes, then verify the hook registers ColorOS cleanup strategy and kill/force-stop utility targets.

**Acceptance Scenarios**:

1. **Given** the Xposed framework loads `com.oplus.athena`, **When** Freezeit handles that package, **Then** it attempts ColorOS-specific hooks instead of ignoring the package.
2. **Given** Athena external cleanup chooses force-stop, kill-pid, kill-uid, or force-stop-or-kill strategies, **When** the hooked strategy entry is reached, **Then** the strategy is short-circuited before invoking the real cleanup.
3. **Given** Athena reaches its shared force-stop or kill utility methods, **When** the hook is installed, **Then** those execution exits return without killing the target process or force-stopping the package.
4. **Given** the hooked ColorOS class is absent or renamed on another build, **When** the hook tries to install, **Then** Freezeit logs the missing hook and continues loading.

---

### User Story 2 - Conflict Behavior Is Visible For Diagnosis (Priority: P2)

As a Freezeit user investigating ColorOS behavior, I want hook coverage and ColorOS policy changes to be visible in logs so that I can tell whether the mitigation actually loaded on my ROM.

**Why this priority**: OPPO class names can change across builds, so silent failure would make the mitigation hard to trust.

**Independent Test**: Inspect the hook code and smoke test output for explicit Athena package, cleanup hook, and GuardElf policy logging coverage.

**Acceptance Scenarios**:

1. **Given** Freezeit installs an Athena hook, **When** the hook is attempted, **Then** the existing Xposed log records success or failure for each method.
2. **Given** Battery changes GuardElf power protection policy through Athena, **When** the binder service method is reached, **Then** Freezeit records the package and policy values without blocking the UI policy write.
3. **Given** a future source edit removes Athena scope or hook targets, **When** the smoke test runs, **Then** it fails before release.

## Requirements

### Functional Requirements

- **FR-001**: Freezeit MUST include `com.oplus.athena` in the Xposed package dispatch path.
- **FR-002**: Freezeit MUST include `com.oplus.athena` in both legacy recommended Xposed scope resources and modern Xposed `scope.list`.
- **FR-003**: Freezeit MUST add a ColorOS/Athena hook module separate from the MIUI PowerKeeper hook.
- **FR-004**: The Athena hook MUST attempt to short-circuit external cleanup strategy entries for force-stop, kill-pid, kill-uid, and force-stop-or-kill strategy classes.
- **FR-005**: The Athena hook MUST attempt to short-circuit shared Athena utility methods that perform force-stop and kill execution.
- **FR-006**: The Athena hook MUST log GuardElf power policy and whitelist switch changes without blocking them.
- **FR-007**: Missing or renamed ColorOS classes MUST be non-fatal and visible through the existing Xposed log mechanism.
- **FR-008**: The implementation MUST include a repository-local smoke test that fails if Athena scope, package dispatch, or key hook targets are removed.
- **FR-009**: The implementation MUST preserve the existing Android and MIUI hook behavior.

### Key Entities

- **Athena Hook Scope**: The Xposed package coverage needed for `com.oplus.athena`.
- **Cleanup Strategy Hook**: A hook on Athena's external cleanup strategy entry methods.
- **Cleanup Utility Hook**: A hook on Athena's shared force-stop and kill execution methods.
- **GuardElf Policy Log**: A diagnostic log entry for power protection policy or whitelist changes.

## Success Criteria

- **SC-001**: `com.oplus.athena` is present in legacy and modern Xposed scope declarations.
- **SC-002**: The hook entry dispatches Athena to a dedicated ColorOS hook module.
- **SC-003**: Source-level smoke tests verify all targeted strategy and utility hooks remain present.
- **SC-004**: Existing MIUI PowerKeeper hook dispatch remains present.
- **SC-005**: The Android app compiles after the hook and scope changes.

## Assumptions

- The mitigation favors Freezeit on ColorOS by neutralizing Athena cleanup exits. Users who want ColorOS to own the policy can disable the Freezeit Xposed scope for `com.oplus.athena`.
- The targeted class and method names are based on the captured ColorOS/Athena build documented in `.codex/oppo-power-re/oppo-coloros-power-manager-re-notes.md`.
- Full import of OPPO runtime allowlists into the Freezeit UI is valuable but outside this MVP; this feature first stops the most expensive cleanup fight and makes hook coverage diagnosable.
