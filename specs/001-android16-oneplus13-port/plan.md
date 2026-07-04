# Implementation Plan: Android 16 OnePlus 13 Port

**Branch**: `main` | **Date**: 2026-07-03 | **Spec**: [spec.md](./spec.md)

**Input**: Feature specification from `specs/001-android16-oneplus13-port/spec.md`

## Summary

Produce a self-use installable Freezeit Magisk/KernelSU module package for the single target ROM archive `/home/admin/code/Rom/oneplus13.zip`. The implementation will reuse the existing Android manager app, native C++ module service, Magisk package layout, app configuration files, and manager/native socket protocol, then add the minimum Android 16/OnePlus 13 compatibility work needed for boot safety, app control, protected-state handling, diagnostics, and validation evidence.

## Technical Context

**Language/Version**: Java 11 Android app; C++20 native service; POSIX shell Magisk scripts; Python 3.8+ ROM tooling.

**Primary Dependencies**: Android Gradle Plugin 8.3.2 and Gradle wrapper 8.4 from `freezeitApp`; AndroidX/AppCompat/Material dependencies already declared in `freezeitApp/app/build.gradle`; compileOnly Xposed API 82; local Android NDK at `/home/admin/Android/Sdk/ndk/28.2.13676358`; Magisk/KernelSU module environment; LSPosed/Xposed hook environment; local `adevtool` 1.0.0; local MIO-KITCHEN-SOURCE checkout.

**Storage**: Existing module files under `/data/adb/modules/freezeit`; existing config files `appcfg.txt`, `applabel.txt`, and `settings.db`; logs exposed through the existing native manager command and file logs such as `boot.log` and optional `/sdcard/Android/freezeit.log`.

**Testing**: Build checks through `bash freezeitApp/gradlew :app:assembleRelease`; native compile through the local NDK clang toolchain; Magisk zip inspection; ROM baseline extraction and compatibility report; manual target-device install, reboot, status, freeze/unfreeze, diagnostics, and root/module-manager recovery validation.

**Target Platform**: OnePlus 13 device running the Android 16 ROM represented by `/home/admin/code/Rom/oneplus13.zip`. Other devices, ROMs, Android versions, and public release channels remain out of scope.

**Project Type**: Android manager app plus native Magisk/KernelSU module package with Xposed/LSPosed hooks.

**Performance Goals**: Manager status visible within 60 seconds after first post-install unlock; configured app control applied within 30 seconds after eligible background transition; foreground restore within 5 seconds in at least 9 of 10 attempts; 3 selected non-critical third-party apps pass freeze/restore validation.

**Constraints**: App control starts only after first user unlock and after native service plus hook state are available; installed build mismatch against the ROM archive is warning-only and must not block control operations; unselected system apps are protected by default; current foreground app, media playback, calls, audio recording, and screen recording are protected from freezing; recovery requirement is disable/uninstall through the root/module manager.

**Scale/Scope**: One self-use ARM64 phone target, one ROM archive, one produced module package, and a small validation set of at least 3 non-critical third-party apps.

## Constitution Check

*GATE: Must pass before Phase 0 research. Re-check after Phase 1 design.*

- [x] Simplicity: the plan reuses the existing app, native service, Magisk package layout, config files, and manager/native protocol; new work is limited to compatibility checks, protected-state handling, Linux build packaging, and evidence documents.
- [x] Verified interfaces: existing project interfaces were checked through the codebase-memory graph and local files; Gradle wrapper usage was checked with `ctx7` against Gradle current user guide; local `adevtool --help`, NDK paths, Gradle wrapper output, and MIO startup behavior were verified locally.
- [x] Human confirmation: target ROM path, self-use scope, build-mismatch behavior, protected app states, post-boot control timing, and recovery path are recorded in `spec.md` clarifications.
- [x] Reuse and architecture: implementation stays inside `freezeitApp`, `freezeitVS`, `freezeitRelease`, and this feature's `specs/` directory; direct code discovery used the project knowledge graph where applicable.
- [x] Verification gates: build checks, artifact inspection, ROM baseline evidence, manual device validation, `/brooks-review`, and `/speckit-converge` are planned before completion.

## Project Structure

### Documentation (this feature)

```text
specs/001-android16-oneplus13-port/
├── plan.md
├── research.md
├── data-model.md
├── quickstart.md
├── contracts/
│   ├── compatibility-evidence.md
│   └── manager-native-protocol.md
└── tasks.md
```

### Source Code (repository root)

```text
freezeitApp/
├── build.gradle
├── gradlew
└── app/
    ├── build.gradle
    └── src/main/
        ├── AndroidManifest.xml
        ├── java/io/github/jark006/freezeit/
        │   ├── ManagerCmd.java
        │   ├── Utils.java
        │   ├── fragment/
        │   ├── activity/
        │   └── hook/
        └── res/

freezeitVS/
├── src/main.cpp
├── include/
│   ├── doze.hpp
│   ├── freezer.hpp
│   ├── freezeit.hpp
│   ├── managedApp.hpp
│   ├── server.hpp
│   ├── settings.hpp
│   ├── systemTools.hpp
│   └── utils.hpp
├── magisk/
│   ├── customize.sh
│   ├── module.prop
│   ├── service.sh
│   ├── uninstall.sh
│   ├── appcfg.txt
│   └── applabel.txt
└── build_pack.ps1

freezeitRelease/
└── produced self-use zip and update metadata, if kept locally

/home/admin/code/Rom/
└── oneplus13.zip

/home/admin/code/MIO-KITCHEN-SOURCE/
└── local ROM unpacking workflow and binaries
```

**Structure Decision**: Keep the current two-part architecture: `freezeitApp` remains the manager/Xposed APK project and `freezeitVS` remains the native service plus Magisk package source. Add only feature documentation under `specs/001-android16-oneplus13-port/`; implementation tasks may add a Linux packaging script or adjust existing scripts, but should not introduce a new application/module boundary.

## Phase 0: Research Summary

Detailed decisions are recorded in [research.md](./research.md). The key planning decisions are:

- Use the local ROM archive as the source of truth for build identity and Android/framework compatibility evidence.
- Use MIO-KITCHEN-SOURCE for ROM unpacking, but first repair its local Python dependency issue (`No module named 'google'`) by installing/checking requirements in an isolated environment.
- Use existing manager/native command IDs and data payloads instead of creating a new API.
- Build the Android app through the existing Gradle wrapper and build the native service with the local Linux NDK path.
- Verify Android 16 hook compatibility against the target ROM framework before changing hook class/method names.
- Treat protected-state detection as implementation research against the ROM/framework, not as an optional fallback behavior.

## Phase 1: Design Summary

Design artifacts are recorded in:

- [data-model.md](./data-model.md)
- [contracts/manager-native-protocol.md](./contracts/manager-native-protocol.md)
- [contracts/compatibility-evidence.md](./contracts/compatibility-evidence.md)
- [quickstart.md](./quickstart.md)

## Post-Design Constitution Check

- [x] Simplicity: design artifacts model existing domain concepts and existing protocol surfaces; no new runtime subsystem is introduced in the plan.
- [x] Verified interfaces: local Gradle, NDK, `adevtool`, MIO, module scripts, and code graph evidence are captured in research; Android 16 framework compatibility is explicitly planned as ROM-derived research before implementation.
- [x] Human confirmation: all product-scope decisions used in the plan are backed by `spec.md` clarifications.
- [x] Reuse and architecture: design routes through existing module/app files and documented contracts for current manager/native communication.
- [x] Verification gates: quickstart includes build, artifact, ROM baseline, device validation, diagnostics, and recovery checks; later tasks must include `/brooks-review` and `/speckit-converge`.

## Complexity Tracking

No constitution violations are planned.
