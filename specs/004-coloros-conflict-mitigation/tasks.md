# Tasks: ColorOS Conflict Mitigation

**Input**: Design documents from `specs/004-coloros-conflict-mitigation/`
**Prerequisites**: `plan.md`, `spec.md`, `research.md`, `data-model.md`, `contracts/athena-hook-coverage.md`, `quickstart.md`

## Phase 1: Setup

- [X] T001 Verify current dirty worktree and preserve unrelated user changes in `/home/admin/code/freezeit`

## Phase 2: Foundational

- [X] T002 Add Athena package/class/method constants in `/home/admin/code/freezeit/freezeitApp/app/src/main/java/io/github/jark006/freezeit/hook/Enum.java`
- [X] T003 Add Athena to legacy and modern Xposed scopes in `/home/admin/code/freezeit/freezeitApp/app/src/main/res/values/arrays.xml` and `/home/admin/code/freezeit/freezeitApp/app/src/main/resources/META-INF/xposed/scope.list`

## Phase 3: User Story 1 - Athena Cleanup Does Not Fight Managed Freezing (P1)

**Goal**: Freezeit loads for Athena and short-circuits Athena cleanup exits.

**Independent Test**: `scripts/test-coloros-athena-hook.sh` confirms scope, dispatch, strategy hooks, and utility hooks.

- [X] T004 [US1] Add dedicated Athena hook class in `/home/admin/code/freezeit/freezeitApp/app/src/main/java/io/github/jark006/freezeit/hook/app/OplusAthena.java`
- [X] T005 [US1] Dispatch `com.oplus.athena` from `/home/admin/code/freezeit/freezeitApp/app/src/main/java/io/github/jark006/freezeit/hook/FreezeitHookEntry.java`
- [X] T006 [US1] Allow `com.oplus.athena` in `/home/admin/code/freezeit/freezeitApp/app/src/main/java/io/github/jark006/freezeit/hook/ModernHook.java`

## Phase 4: User Story 2 - Conflict Behavior Is Visible For Diagnosis (P2)

**Goal**: Hook coverage and GuardElf policy changes are visible through logs and regression checks.

**Independent Test**: Smoke test fails if diagnostic hook targets or Athena scope are removed.

- [X] T007 [US2] Add GuardElf diagnostic hook logging in `/home/admin/code/freezeit/freezeitApp/app/src/main/java/io/github/jark006/freezeit/hook/app/OplusAthena.java`
- [X] T008 [US2] Add source smoke test in `/home/admin/code/freezeit/scripts/test-coloros-athena-hook.sh`

## Phase 5: Validation

- [X] T009 Run `/home/admin/code/freezeit/scripts/test-coloros-athena-hook.sh`
- [X] T010 Run Android Gradle build from `/home/admin/code/freezeit/freezeitApp`
- [X] T011 Review changed files and mark completed tasks in `/home/admin/code/freezeit/specs/004-coloros-conflict-mitigation/tasks.md`

## Dependencies

- T002 before T004-T007.
- T003 before T009.
- T004 before T005-T007.
- T008 before T009.
- T009 before T010.

## Parallel Opportunities

- T002 and T003 can be done independently.
- T008 can be written after target strings are known and before implementation is complete.

## Implementation Strategy

Complete Athena scope and dispatch first, then add cleanup hook coverage, then add diagnostic smoke testing and build validation.
