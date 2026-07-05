# Implementation Plan: Download Freeze Deferral

**Branch**: `main`
**Spec**: `specs/003-download-freeze-deferral/spec.md`
**Date**: 2026-07-06

## Summary

Add a download-aware freeze guard to the legacy native daemon. Matching cloud drive packages near the end of their pending freeze countdown get UID receive-byte samples. If the measured receive rate is greater than 5 MiB/s, the app remains pending and the freeze attempt is delayed.

## Technical Context

**Native manager service**: C++ service under `freezeitVS/include`.
**Tracked release path**: Legacy native daemon packaged from `freezeitVS`.
**Target platform**: Rooted Android 16 / OnePlus 13 where legacy per-UID `/proc` stats are absent.
**Traffic source**: `dumpsys netstats` BPF section `mAppUidStatsMap`, parsed for UID receive bytes.
**Existing delay model**: `pendingHandleList` countdown in `Freezer::processPendingApp`.
**Build systems**: Existing `freezeitVS/build_arm64_linux.sh` or release packaging scripts.

## Constitution Check

No project constitution file exists in this repository. The implementation follows local patterns: header-only daemon helpers, small shell regression tests, existing pending-freeze countdown semantics, and no unrelated release artifact changes.

## Project Structure

### Documentation

```text
specs/003-download-freeze-deferral/
  spec.md
  checklists/requirements.md
  plan.md
  research.md
  data-model.md
  contracts/download-deferral.md
  quickstart.md
  tasks.md
```

### Source

```text
freezeitVS/include/
  downloadDeferral.hpp
  freezer.hpp

scripts/
  test-download-freeze-deferral.sh
```

## Phase 0: Research

See `research.md`.

## Phase 1: Design

See `data-model.md`, `contracts/download-deferral.md`, and `quickstart.md`.

## Phase 2: Implementation Approach

1. Add a small testable C++ helper for candidate package matching and receive-byte rate decisions.
2. Add a shell regression test that compiles and runs the helper behavior, then checks daemon integration points.
3. Add a `Freezer` receive-byte cache that parses `dumpsys netstats` once per second when needed.
4. Prime candidate package samples when a pending freeze has one second remaining.
5. At freeze time, wait for a sample if none exists, defer when rate is above threshold, or proceed normally otherwise.
6. Clear stale samples when apps leave or complete the pending queue.

## Validation

- Run `scripts/test-download-freeze-deferral.sh` and verify it fails before implementation and passes after implementation.
- Run `scripts/test-legacy-refreeze-schedule.sh` to protect the recent refreeze behavior.
- Run the native ARM64 build from `freezeitVS` where the Android NDK environment is available.
