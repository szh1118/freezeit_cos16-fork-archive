# Implementation Plan: ColorOS Conflict Mitigation

**Branch**: `main`
**Spec**: `specs/004-coloros-conflict-mitigation/spec.md`
**Date**: 2026-07-06

## Summary

Add a dedicated OPPO/ColorOS Athena Xposed hook alongside the existing MIUI PowerKeeper hook. The hook covers Athena's external clear strategy entries and shared kill/force-stop utility methods, adds diagnostic logging for GuardElf policy changes, and updates Xposed scopes so the hook can load in both legacy and modern frameworks.

## Technical Context

**Android manager**: Java Android app under `freezeitApp/app/src/main`.
**Hook framework**: Existing legacy Xposed and modern libxposed compatibility layer.
**Vendor package**: ColorOS package-level power policy service `com.oplus.athena`.
**Reverse-engineering source**: `.codex/oppo-power-re/oppo-coloros-power-manager-re-notes.md` and captured decompiled Athena sources.
**Build system**: Gradle Android app build under `freezeitApp`.
**Validation**: Source smoke test plus Gradle compile/build.

## Constitution Check

No project constitution file exists in this repository. The implementation follows local patterns: string constants in `Enum`, package dispatch in `FreezeitHookEntry`, one vendor hook class under `hook/app`, existing Xposed logging, and shell smoke tests in `scripts`.

## Project Structure

### Documentation

```text
specs/004-coloros-conflict-mitigation/
  spec.md
  checklists/requirements.md
  plan.md
  research.md
  data-model.md
  contracts/athena-hook-coverage.md
  quickstart.md
  tasks.md
```

### Source

```text
freezeitApp/app/src/main/java/io/github/jark006/freezeit/hook/
  Enum.java
  FreezeitHookEntry.java
  ModernHook.java

freezeitApp/app/src/main/java/io/github/jark006/freezeit/hook/app/
  OplusAthena.java

freezeitApp/app/src/main/res/values/
  arrays.xml

freezeitApp/app/src/main/resources/META-INF/xposed/
  scope.list

scripts/
  test-coloros-athena-hook.sh
```

## Phase 0: Research

See `research.md`.

## Phase 1: Design

See `data-model.md`, `contracts/athena-hook-coverage.md`, and `quickstart.md`.

## Phase 2: Implementation Approach

1. Add Athena package and class/method constants to the hook enum catalog.
2. Add a dedicated `OplusAthena` hook module using the existing `XpUtils` backend.
3. Wire Athena package dispatch in legacy and modern hook paths.
4. Add Athena to both Xposed scope declarations.
5. Add a smoke test that checks scope, dispatch, and hook coverage.
6. Run the smoke test and Android build.

## Validation

- Run `scripts/test-coloros-athena-hook.sh`.
- Run Gradle build from `freezeitApp` where the Android SDK/Gradle environment is available.
- Review tasks and mark completed items in `tasks.md`.

## Agent Context Update

No `AGENTS.md` file exists in this repository, so there is no marker block to update. The active feature path is persisted in `.specify/feature.json`.
