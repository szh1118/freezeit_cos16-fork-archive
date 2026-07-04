# Implementation Plan: Modern Freezer Rewrite

**Branch**: `002-modern-freezer-rewrite` | **Date**: 2026-07-03 | **Spec**: [spec.md](spec.md)

**Input**: Feature specification from `specs/002-modern-freezer-rewrite/spec.md`

## Summary

Rewrite the Freezeit daemon as a pure Rust Android/Magisk daemon while preserving the existing manager workflow and LSPosed hook surface. The new design prioritizes daily-use stability on the verified OnePlus CPH2653 Android 16/COS16 baseline by isolating unsafe kernel/system calls, using system-aware freeze capability detection, keeping the current manager protocol compatible, and treating hook/root/freezer readiness failures as fail-closed degraded states.

The manager APK and LSPosed Modern API entry remain in `freezeitApp`; the Magisk packaging surface remains compatible with the current module layout. The old C++ daemon under `freezeitVS` becomes the behavioral reference and migration source, not the long-term control core.

## Technical Context

**Language/Version**: Rust stable for the new daemon targeting `aarch64-linux-android`; Java for the existing Android manager and LSPosed module; POSIX shell for Magisk service/customize scripts.

**Primary Dependencies**: Android NDK toolchain, `cargo-ndk` for Rust cross-compilation, Rust `libc` for binder ioctl and low-level syscalls, `serde`/`serde_json` for typed config/health/report serialization, existing Gradle/Android plugin stack for `freezeitApp`, LSPosed Modern Xposed API metadata/API 100-102.

**Storage**: Magisk module directory under `/data/adb/modules/freezeit`; existing policy/label/settings files migrated from the current module format; daemon health and compatibility reports in module-owned data files; existing manager log surfaces preserved.

**Testing**: Rust unit and contract tests, protocol fixture tests against existing manager frames, Gradle build for manager APK, Magisk zip integrity checks, ADB validation on the verified physical device, and 24-hour self-use soak before release completion.

**Target Platform**: Owner device baseline verified on 2026-07-03: OnePlus CPH2653 / Android 16 / SDK 36 / fingerprint `OnePlus/CPH2653EEA/OP5D55L1:16/BP2A.250605.015/V.R4T3.1338e95_e24685_de185d:user/release-keys` / Linux `6.6.89-android15-8-g096cdb6ecefc-ab14358676-4k` / arm64 / Magisk root context `u:r:magisk:s0` / LSPosed API 102 module metadata.

**Project Type**: Android Magisk module with native daemon, Android manager APK, and LSPosed system_server/package hooks.

**Performance Goals**: Manager reports active daemon/hook/root/freezer readiness within 30 seconds after first unlock; at least 95% of selected app background transitions reach the configured state within configured delay plus 5 seconds; at least 95% of controlled apps return to foreground usability within 2 seconds.

**Constraints**: Fail closed on missing hook/root/package/freezer capability; no unsafe control of protected apps by default; no boot-loop-prone startup behavior; no broad-device compatibility claims beyond the verified baseline without new evidence; all unsafe Rust remains isolated behind safe interfaces.

**Scale/Scope**: Single owner daily-use device first; hundreds of installed packages and multiple processes per package; preserve current manager operations before adding new diagnostics.

## Constitution Check

*GATE: Must pass before Phase 0 research. Re-check after Phase 1 design.*

- [x] Simplicity: pure Rust daemon avoids mixed C++/Rust build plumbing; manager and Magisk boundaries are reused; new modules are limited to daemon core, capability backends, protocol compatibility, and migration.
- [x] Verified interfaces: LSPosed Modern API was checked through Context7 (`/lsposed/lsposed`); Magisk module behavior, Rust Android targets, cargo-ndk, AOSP freezer/binder source, kernel cgroup v2 docs, codebase graph, and target-device runtime evidence are recorded in [research.md](research.md).
- [x] Human confirmation: owner confirmed daily-use stability as the goal and requested independent technical research; owner-facing tradeoff questions are deferred only for verified alternatives that change daily-use behavior, compatibility scope, or risk.
- [x] Reuse and architecture: current `freezeitApp`, LSPosed resources, manager command protocol, Magisk packaging scripts, and release layout remain the integration contracts; code discovery used codebase-memory MCP before falling back to `rg` for resources/configs.
- [x] Verification gates: build, unit/contract tests, ADB manual validation, release zip integrity, 24-hour soak, `/brooks-review`, and `/speckit-converge` are planned before implementation completion is claimed.

**Post-Design Re-check**: PASS. Phase 1 contracts keep external behavior compatible and add diagnostics without forcing UI changes. The only new source root is justified by the full-rewrite decision and avoids a higher-risk mixed C++/Rust build.

## Project Structure

### Documentation (this feature)

```text
specs/002-modern-freezer-rewrite/
├── plan.md
├── research.md
├── data-model.md
├── quickstart.md
├── contracts/
│   ├── freezer-backend.md
│   ├── manager-daemon-protocol.md
│   └── xposed-daemon-bridge.md
└── tasks.md
```

### Source Code (repository root)

```text
freezeitDaemon/
├── Cargo.toml
├── src/
│   ├── main.rs
│   ├── app/
│   │   ├── compatibility.rs
│   │   ├── controller.rs
│   │   ├── error.rs
│   │   ├── foreground.rs
│   │   ├── freezer_backend.rs
│   │   ├── health.rs
│   │   ├── logging.rs
│   │   ├── operation_log.rs
│   │   ├── package_inventory.rs
│   │   └── scheduler.rs
│   ├── config/
│   │   ├── loader.rs
│   │   └── migration.rs
│   ├── domain/
│   │   ├── capability.rs
│   │   ├── operation.rs
│   │   ├── policy.rs
│   │   └── runtime.rs
│   ├── protocol/
│   │   ├── manager_v1.rs
│   │   ├── manager_v2.rs
│   │   └── xposed.rs
│   └── sys/
│       ├── binder.rs
│       ├── cgroup.rs
│       ├── procfs.rs
│       ├── signal.rs
│       └── socket.rs
├── tests/
│   ├── contract/
│   ├── integration/
│   └── fixtures/
└── scripts/
    ├── build-android.sh
    └── test-host.sh

freezeitApp/
└── app/src/main/
    ├── java/io/github/jark006/freezeit/
    └── resources/META-INF/xposed/

freezeitVS/
├── include/                  # legacy C++ behavioral reference during migration
├── magisk/                   # packaging scripts and module metadata integration point
└── src/

freezeitRelease/
└── release packaging outputs and update metadata

scripts/
├── capture-rom-baseline.sh
├── package-release.sh
├── validate-device-baseline.sh
├── validate-degraded-state.sh
├── validate-freeze-unfreeze.sh
├── validate-install-boot.sh
├── validate-magisk-zip.sh
└── validate-release-zip.sh
```

**Structure Decision**: Add `freezeitDaemon` as a pure Rust source root. Keep `freezeitApp` as the manager/hook module and keep the current Magisk packaging path as the installer/release integration point. Do not create a mixed C++/Rust daemon: the old C++ daemon remains available as a reference for behavior, protocol compatibility, config migration, and regression tests.

## Complexity Tracking

No constitution violations. The new Rust daemon root is required by the owner-selected full rewrite and is simpler than maintaining a mixed C++/Rust daemon build on Android NDK.
