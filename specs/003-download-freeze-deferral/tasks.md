# Tasks: Download Freeze Deferral

**Input**: Design documents from `specs/003-download-freeze-deferral/`
**Prerequisites**: `plan.md`, `spec.md`, `research.md`, `data-model.md`, `contracts/download-deferral.md`, `quickstart.md`

## Phase 1: Setup

- [X] T001 Verify current branch is `main` and preserve unrelated dirty work

## Phase 2: Tests First

- [X] T002 [P] Add failing download deferral regression test in `scripts/test-download-freeze-deferral.sh`
- [X] T003 Run `scripts/test-download-freeze-deferral.sh` and confirm it fails because the helper/integration does not exist yet

## Phase 3: User Story 1 - Active Downloads Are Not Interrupted (P1)

**Goal**: A matching pending app above 5 MiB/s is kept pending instead of frozen.

**Independent Test**: Helper test verifies `WaitForSample`, `Defer`, and `Proceed` decisions from UID receive-byte samples.

- [X] T004 [US1] Add testable helper in `freezeitVS/include/downloadDeferral.hpp`
- [X] T005 [US1] Add UID receive-byte netstats parsing and per-second cache in `freezeitVS/include/freezer.hpp`
- [X] T006 [US1] Prime samples near freeze expiry and defer high-speed candidates in `freezeitVS/include/freezer.hpp`

## Phase 4: User Story 2 - Known Cloud Drive Packages Are Covered (P1)

**Goal**: All requested package substrings, including `com.trim.app`, are covered.

**Independent Test**: Helper test verifies every requested substring matches and unrelated packages do not.

- [X] T007 [US2] Include requested package substrings in `freezeitVS/include/downloadDeferral.hpp`
- [X] T008 [US2] Log active-download deferrals with label and observed speed in `freezeitVS/include/freezer.hpp`

## Phase 5: Analysis

- [X] T009 Run a spec/plan/tasks consistency pass for `specs/003-download-freeze-deferral`

## Phase 6: Validation

- [X] T010 Run `scripts/test-download-freeze-deferral.sh`
- [X] T011 Run `scripts/test-legacy-refreeze-schedule.sh`
- [X] T012 Build the native daemon with `freezeitVS/build_arm64_linux.sh`
- [X] T013 Review changed files and mark completed tasks in `specs/003-download-freeze-deferral/tasks.md`

## Dependencies

- T002 and T003 before T004-T008.
- T004 before T005-T008.
- T005 before T006.
- T009 after task generation and before final implementation completion.
- T010-T012 after implementation tasks.

## Parallel Opportunities

- T002 can be reviewed independently from documentation.
- T007 is covered by T004 when package matching is implemented in the helper.

## Implementation Strategy

Use TDD on the helper first, then connect the helper to the pending freeze queue. Keep the daemon behavior scoped to matching packages near freeze time.
