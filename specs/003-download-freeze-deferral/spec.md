# Feature Specification: Download Freeze Deferral

**Feature Branch**: `main`
**Created**: 2026-07-06
**Status**: Implemented
**Input**: "Add a no-freeze decision for cloud drive / downloader apps. When an app package contains baidu.netdisk, quark.clouddrive, com.google.android.apps.docs, pikpak, or com.trim.app, and its download speed is greater than 5 MB/s, delay freezing it."

## User Scenarios & Testing

### User Story 1 - Active Downloads Are Not Interrupted (Priority: P1)

As a Freezeit user downloading large files through a cloud drive app, I want that app to avoid freezing while it is actively downloading faster than the threshold so that downloads are not interrupted after the app leaves the foreground.

**Why this priority**: The feature exists to protect active downloads from the normal background freeze timeout.

**Independent Test**: Put a matching package in the pending freeze queue, provide two UID receive-byte samples at least one second apart where the rate is greater than 5 MiB/s, and verify the freeze attempt is deferred rather than executed.

**Acceptance Scenarios**:

1. **Given** a matching cloud drive package is pending freeze, **When** its measured download rate is greater than 5 MiB/s, **Then** the app remains in the pending queue and freezing is delayed.
2. **Given** a matching cloud drive package is pending freeze, **When** there is no previous receive-byte sample, **Then** Freezeit waits for one short measurement interval before making the freeze decision.
3. **Given** a matching cloud drive package is pending freeze, **When** its measured download rate is 5 MiB/s or lower, **Then** the normal freeze flow proceeds.
4. **Given** a non-matching package is pending freeze, **When** it has any download speed, **Then** this feature does not change the normal freeze decision.

---

### User Story 2 - Known Cloud Drive Packages Are Covered (Priority: P1)

As a Freezeit user, I want common cloud drive package families and my NAS client package covered by default so that I do not need to manually maintain a separate whitelist for active downloads.

**Why this priority**: The requested behavior depends on package-name matching being explicit and complete for the named apps.

**Independent Test**: Check package matching against the requested substrings and verify `com.trim.app` is included.

**Acceptance Scenarios**:

1. **Given** a package contains `baidu.netdisk`, **When** it is checked for download protection, **Then** it is eligible for active-download deferral.
2. **Given** a package contains `quark.clouddrive`, **When** it is checked for download protection, **Then** it is eligible for active-download deferral.
3. **Given** a package contains `com.google.android.apps.docs`, **When** it is checked for download protection, **Then** it is eligible for active-download deferral.
4. **Given** a package contains `pikpak`, **When** it is checked for download protection, **Then** it is eligible for active-download deferral.
5. **Given** the package is `com.trim.app`, **When** it is checked for download protection, **Then** it is eligible for active-download deferral.

## Requirements

### Functional Requirements

- **FR-001**: Freezeit MUST recognize packages containing these substrings as active-download deferral candidates: `baidu.netdisk`, `quark.clouddrive`, `com.google.android.apps.docs`, `pikpak`, and `com.trim.app`.
- **FR-002**: Active-download deferral MUST apply only when a candidate app is already pending a freeze attempt.
- **FR-003**: Freezeit MUST measure candidate app download rate from UID receive-byte deltas over at least one second.
- **FR-004**: Freezeit MUST defer freezing only when measured download rate is strictly greater than 5 MiB/s.
- **FR-005**: When deferral is triggered, the app MUST remain pending for a short retry interval and be evaluated again later.
- **FR-006**: When no previous sample exists for a candidate app at freeze time, Freezeit MUST take a sample and wait one short measurement interval instead of freezing immediately.
- **FR-007**: When receive-byte statistics are unavailable or invalid, Freezeit MUST continue the normal freeze flow rather than treating the app as protected.
- **FR-008**: Non-candidate apps MUST keep the existing freeze behavior.
- **FR-009**: Deferral events SHOULD be logged with app identity and observed download speed so the user can understand why an app stayed unfrozen.

### Key Entities

- **Download Deferral Candidate**: A managed app whose package name contains one of the requested cloud drive substrings.
- **UID Receive-Byte Sample**: A point-in-time record of total received bytes for an app UID.
- **Deferral Decision**: The result of comparing two samples and deciding whether to wait for a sample, defer freezing, or proceed with normal freeze.

## Success Criteria

- **SC-001**: All five requested package patterns are covered by automated checks.
- **SC-002**: A candidate app measured above 5 MiB/s is not frozen on that freeze attempt.
- **SC-003**: A candidate app measured at or below 5 MiB/s follows the existing freeze flow.
- **SC-004**: A candidate app without a prior sample waits for one measurement interval before final freeze decision.
- **SC-005**: A non-candidate app is unaffected by the new logic.

## Assumptions

- "Delay freezing" means skipping the current freeze attempt, keeping the app in the pending queue, and re-evaluating later. It does not create a permanent whitelist.
- The 5 MB/s threshold is implemented as 5 MiB/s (`5 * 1024 * 1024` bytes per second), matching the rest of the daemon's binary memory-size conventions.
- On Android 16, per-UID receive-byte data is read from the system netstats dump because legacy `/proc/net/xt_qtaguid/stats` and `/proc/uid_stat` are absent on the target phone.
