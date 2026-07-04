# Research: Modern Freezer Rewrite

## Decision: Use a pure Rust daemon rewrite

**Rationale**: The owner selected the full rewrite path for long-term daily-use stability. A pure Rust daemon avoids the mixed C++/Rust Android build complexity that would require both CMake and Cargo integration. Rust also gives safer parsing, state machines, and request handling while keeping low-level Android syscalls isolated.

**Evidence**:
- Rust platform support documents Android targets such as `aarch64-linux-android`: https://doc.rust-lang.org/rustc/platform-support/android.html
- `cargo-ndk` is the established tool for building Rust code for Android NDK targets: https://github.com/bbqsrc/cargo-ndk
- Existing C++ complexity is concentrated in `freezeitVS/include/freezer.hpp` and `server.hpp`; codebase-memory showed `handleProcess` and `serverThreadFunc` have high branching and protocol/capability responsibilities.

**Alternatives considered**:
- Keep C++ and patch paths: lower immediate cost, but preserves fragile parsing/threading/freezer control and does not meet the selected rewrite direction.
- Mixed C++/Rust incremental migration: useful for large organizations, but higher build-system risk for this module than a clean daemon rewrite.

## Decision: Isolate unsafe Android/kernel operations behind a narrow `sys` layer

**Rationale**: Binder ioctl, procfs parsing, signal delivery, socket handling, and cgroup file writes must touch unsafe or platform-specific APIs. The daemon will expose safe Rust functions such as `freeze_process`, `unfreeze_process`, `query_process`, and `read_capabilities`; policy, scheduling, config parsing, and logs stay in safe Rust.

**Evidence**:
- Kernel cgroup v2 freezer exposes `cgroup.freeze`: https://docs.kernel.org/admin-guide/cgroup-v2.html
- Android binder userspace UAPI contains freezer ioctls/structures in binder headers: https://android.googlesource.com/kernel/common/+/refs/heads/android-mainline/include/uapi/linux/android/binder.h
- Target device exposes binder devices through `/dev/binderfs/*` symlinks and root runs in Magisk context.

**Alternatives considered**:
- Use shell commands for every operation: fast to prototype but weaker error handling, slower, and harder to test.
- Put unsafe calls inline in business logic: rejected because daily-use stability needs auditable boundaries.

## Decision: Prefer a system-aware freezer backend over legacy freezer-only control

**Rationale**: Android's platform freezer coordinates cached process state, binder behavior, and cgroup freezer behavior. On the COS16 target, `dumpsys activity processes` exposes `isPendingFreeze`, `isFrozen`, and Oplus `virtualFreeze`, while `/sys/fs/cgroup/apps/uid_*/pid_*/cgroup.freeze` and `/sys/fs/cgroup/system/...` are present. The new daemon should detect and cooperate with these states rather than blindly writing old module-specific freezer paths.

**Evidence**:
- AOSP Cached App Freezer documentation: https://source.android.com/docs/core/perf/cached-apps-freezer
- AOSP `CachedAppOptimizer.java`: https://android.googlesource.com/platform/frameworks/base/+/master/services/core/java/com/android/server/am/CachedAppOptimizer.java
- AOSP `Process.setProcessFrozen` source surface: https://android.googlesource.com/platform/frameworks/base/+/master/core/java/android/os/Process.java
- Device evidence on 2026-07-03: cgroup freeze paths exist under `/sys/fs/cgroup/apps/uid_*/pid_*/cgroup.freeze` and `/sys/fs/cgroup/system/uid_*/pid_*/cgroup.freeze`; activity dumpsys exposes freezer state fields.

**Selected backend order**:
1. Capability discovery and safety checks through root, procfs, cgroup v2, binder device, package inventory, and LSPosed system_server readiness.
2. Preferred: system-aware cgroup v2 plus binder freezer backend, informed by LSPosed foreground/runtime state.
3. Experimental candidate during implementation: system_server bridge that invokes verified framework freezer paths only if runtime tests prove it keeps AMS state coherent and is safer than root-side control.
4. Fallback: postpone, skip, signal, or terminate only when explicitly safe for the policy and logged with reason.

**Alternatives considered**:
- SIGSTOP/SIGCONT as primary freezer: simple but risky with binder and foreground transitions.
- Use only `/sys/fs/cgroup/frozen` and `/sys/fs/cgroup/unfrozen`: present on the target but not the Android 16 per-process freezer layout; use only as compatibility evidence, not the primary model.
- Direct hidden API calls without system_server evidence: rejected until verified because it may desynchronize ActivityManager state.

## Decision: Keep LSPosed Modern API 102 integration and make it a health-gated bridge

**Rationale**: The existing APK is already adapted to Modern Xposed metadata and entrypoints. The rewrite should not regress hook activation. LSPosed should provide system_server/package observations and bridge commands, while daemon enforcement remains fail-closed if hook readiness is missing.

**Evidence**:
- Context7 `/lsposed/lsposed` documents Modern API resource files: `META-INF/xposed/module.prop`, `scope.list`, `java_init.list`, `minApiVersion`, `targetApiVersion`, and `staticScope`.
- Existing files:
  - `freezeitApp/app/src/main/resources/META-INF/xposed/module.prop`: `minApiVersion=100`, `targetApiVersion=102`, `staticScope=true`
  - `freezeitApp/app/src/main/resources/META-INF/xposed/java_init.list`: `io.github.jark006.freezeit.hook.ModernHook`
  - `ModernHook` uses `XposedModule`, `onPackageReady`, and `onSystemServerStarting`.
- LSPosed wiki: https://github.com/LSPosed/LSPosed/wiki/Develop-Xposed-Modules-Using-Modern-Xposed-API

**Alternatives considered**:
- Return to legacy Xposed entrypoints: rejected because current LSPosed Modern API adaptation already works on the target.
- Treat hooks as optional-only: rejected for daily-use safety; missing system_server readiness must degrade control.

## Decision: Preserve manager protocol compatibility first, then add v2 diagnostics

**Rationale**: The owner wants the daily manager workflow preserved. The existing manager connects to `127.0.0.1:60613` with a 6-byte little-endian header and byte command ID. The Rust daemon must implement v1 commands before any manager UI rewrite. New health/capability commands can be added as JSON reports behind new command IDs.

**Evidence**:
- Existing manager helper `Utils.freezeitTask` connects to `127.0.0.1:60613`.
- Existing daemon `Server.serverThreadFunc` accepts TCP localhost and dispatches `MANAGER_CMD`.
- Existing `MANAGER_CMD` includes `getPropInfo`, `getLog`, `getAppCfg`, `setAppCfg`, `setAppLabel`, `setSettingsVar`, `getProcState`, and `getXpLog`.

**Alternatives considered**:
- Replace protocol with HTTP/gRPC: rejected as unnecessary new surface for a local root daemon.
- Rewrite manager first: rejected because it increases blast radius before daemon behavior is proven.

## Decision: Keep Magisk module lifecycle and update package contents

**Rationale**: The current install/boot flow works after prior adaptation. The Rust binary can replace the daemon payload while reusing Magisk's module layout, `service.sh` startup timing after boot/unlock, `customize.sh`, APK install behavior, and release zip structure.

**Evidence**:
- Existing `freezeitVS/magisk/service.sh` waits for `sys.boot_completed` and `/sdcard` write permission before starting `freezeit`.
- Existing `module.prop` and `customize.sh` define the Magisk module metadata and payload selection.
- Magisk module guide: https://topjohnwu.github.io/Magisk/guides.html

**Alternatives considered**:
- Move install lifecycle into the manager app: rejected because root daemon boot behavior belongs in Magisk.
- Change module ID immediately: rejected for upgrade/migration stability unless release policy later requires it.

## Decision: Verification must include real-device ADB evidence

**Rationale**: This is a ROM-specific system module. Builds and unit tests are insufficient; release confidence depends on ADB validation on the actual COS16 phone.

**Evidence gathered on 2026-07-03**:
- `adb devices -l`: `3B1F4LE5MS142WJY`, model `CPH2653`
- `getprop ro.build.version.release`: `16`
- `getprop ro.build.version.sdk`: `36`
- `getprop ro.build.fingerprint`: `OnePlus/CPH2653EEA/OP5D55L1:16/BP2A.250605.015/V.R4T3.1338e95_e24685_de185d:user/release-keys`
- `uname -a`: `Linux localhost 6.6.89-android15-8-g096cdb6ecefc-ab14358676-4k`
- `su -c id`: `uid=0(root) ... context=u:r:magisk:s0`

**Alternatives considered**:
- AVD-only validation: insufficient because LSPosed/Magisk/COS freezer behavior is device-specific.
- Manual manager checks only: insufficient without daemon logs and cgroup/dumpsys evidence.

## Decision: Do not promote the system_server bridge as a freezer backend yet

**Rationale**: The US2 implementation keeps LSPosed/system_server bridge data as
safety evidence for readiness, foreground/runtime state, screen state,
pending-freeze hints, and freezer hints. It does not promote framework-side
system_server calls as a freeze/unfreeze backend yet. The selected backend
remains `SystemAwareCgroupBinderBackend`, with fail-closed fallback to postpone,
alternate freezer, signal, terminate, or skip.

Promoting `SystemServerBridgeBackend` would require target-device proof that
framework-side mutation keeps ActivityManager, binder, cgroup, and
manager-visible state coherent through foreground launch, process churn, and
daemon restart. That evidence is not present yet.

**Evidence gathered on 2026-07-04**:
- `specs/002-modern-freezer-rewrite/evidence/us2-host-tests.md`: host tests pass
  for backend decisions, fallback order, scheduler transitions, procfs identity,
  and primitive cgroup/binder wrappers.
- `specs/002-modern-freezer-rewrite/evidence/us1-install-boot.md`: target device
  install/reboot/daemon residency and manager degraded rendering are proven,
  including deliberate missing-LSPosed-scope validation.
- `specs/002-modern-freezer-rewrite/evidence/us2-device-freeze.md`: T052
  real-device freeze/unfreeze validation passes on four selected packages using
  app cgroup `cgroup.freeze`; no system_server freezer backend promotion is
  required for this self-use baseline.

**Alternatives considered**:
- Promote system_server bridge immediately: rejected because it would claim
  unverified framework-side control behavior.
- Remove bridge data from US2 decisions: rejected because hook/runtime/screen
  evidence is useful for conservative postpone/skip decisions.

## Decision: Keep the modern rewrite scoped to self-use release evidence

**Rationale**: The Rust daemon now has host coverage for protocol
compatibility, classification, migration, diagnostics, compatibility reports,
release packaging, and real-device cgroup freeze/unfreeze. Degraded fault
injection, release install/control/restore, quickstart validation, final
aggregate review/convergence, and 24-hour soak evidence are still open.
The release identity is therefore `3.2.0SelfUse`, not a broad public
compatibility claim.

**Evidence gathered on 2026-07-04**:
- `specs/002-modern-freezer-rewrite/evidence/final-build.md`: Rust host tests,
  Android daemon build, manager release assemble, and package zip integrity
  pass.
- `specs/002-modern-freezer-rewrite/evidence/us3-classification.md`: target
  package inventory, launcher, input method, manager, and system-critical
  protected classification inputs are recorded.
- `specs/002-modern-freezer-rewrite/evidence/us4-implementation.md`: diagnostic
  JSON, persistent operation log snapshot, recovery recording, config recovery,
  and degraded reasons pass host checks.
- `specs/002-modern-freezer-rewrite/evidence/us2-device-freeze.md`: target
  freezer control and foreground restore pass on three third-party apps and one
  multi-process app.
- Open tasks T074, T083, T084, T086, T089, and T090 still gate final release
  confidence.

**Owner-visible tradeoff**:
- The manager now shows diagnostic JSON directly in existing text surfaces.
  This preserves the current UI structure and reduces risk, but it is less
  polished than a dedicated diagnostics screen. A dedicated UI should wait
  until device validation proves the diagnostic fields are stable.

**Alternatives considered**:
- Claim release readiness after host/package builds: rejected because the spec
  requires real-device control, degraded-state, release, and soak evidence.
- Redesign diagnostics UI now: rejected because it would add UI churn before
  the daemon behavior is fully validated.

## Decision: Treat Android app cgroup v2 freeze files as the verified freezer path on CPH2653

**Rationale**: The target ROM exposes working app freezer files under
`/sys/fs/cgroup/apps` even though `/sys/fs/cgroup/cgroup.controllers` does not
print `freezer` at runtime. ROM config still declares cgroup v2 freezer in
`/system/etc/cgroups.json`, and prior target freeze validation proved app
`cgroup.freeze` mutation works. Therefore the daemon records the controllers
file as evidence but prefers discovered app `cgroup.freeze` files over legacy
`/dev/freezer`.

**Evidence gathered on 2026-07-04**:
- `cmd=72 getCapabilityReport`: `cgroup_v2_freezer` reported `available` with
  `/sys/fs/cgroup/cgroup.controllers contains freezer=false; freeze_files=248`.
- Read-only shell evidence found 498 `cgroup.freeze` files under
  `/sys/fs/cgroup/apps` and `/sys/fs/cgroup/system`.
- `/system/etc/cgroups.json` declares `Cgroups2.Path=/sys/fs/cgroup` and
  controller `freezer`.
- `/system_ext/etc/init/hans.rc` mounts legacy `/dev/freezer`, but that path is
  treated as ROM compatibility evidence rather than the preferred backend.

**Alternatives considered**:
- Require `cgroup.controllers` to list `freezer`: rejected because it would
  falsely mark the verified target freezer unavailable.
- Prefer `/dev/freezer`: rejected for Android 16/COS16 because per-app
  `cgroup.freeze` paths are present and target-validated.

## Decision: Binder freezer remains an untested capability until probed safely

**Rationale**: Device binder nodes are present, and the Rust daemon knows the
Binder freezer ioctl numbers, but a safe target ioctl probe and rejection
handling contract has not been proven. The daemon must not report Binder freezer
as available merely because `/dev/binder` exists.

**Evidence gathered on 2026-07-04**:
- `/dev/binder` is a symlink to `/dev/binderfs/binder`.
- `/dev/binderfs/binder` is present as a character device.
- `cmd=72` and `cmd=73` report `binder_freezer` as `untested` with ioctl number
  evidence and a target-probe-required reason.

**Alternatives considered**:
- Keep hardcoded Binder freezer availability: rejected because it could mask an
  unsupported ioctl path.
- Disable cgroup freezing until Binder freezer is proven: rejected because
  target app cgroup freezing is already validated and Binder freezer is an
  additional safety signal, not the only enforcement path.

## Threat Model Boundary For Aggressive Background Apps

**Rationale**: Freezeit is a background execution control module, not a malware
scanner, sandbox, exploit mitigator, or root trust boundary. For
Pinduoduo-style exploit-chain concerns, the owner-visible boundary must be
explicit: freezing reduces background runtime after eligibility, but it cannot
undo code execution that happened before freeze or defend against privileged
system/root compromise.

**What Freezeit mitigates after control applies**:
- Background CPU execution by selected third-party packages after foreground
  eligibility, configured delay, readiness gates, and idle evidence pass.
- Continued process work for frozen PIDs through Android app cgroup v2
  `cgroup.freeze`, with signal fallback only when policy permits.
- Repeated background re-entry by rechecking package identity, UID, foreground
  state, and process inventory before control.
- Silent unsafe operation when hook/root/freezer/package capability is missing,
  because readiness failures degrade and block unsafe control.

**What remains possible**:
- Any behavior before the configured delay expires.
- Foreground/user-visible behavior while the app is actively used.
- Activity from system, root, hook framework, ROM allowlist, privileged service,
  or exploit path outside the selected app process set.
- Network, wake-lock, alarm, notification, or IPC effects that occur before
  freeze or through privileged system components.
- Broad-device assumptions beyond the verified CPH2653 Android 16 baseline.

**Communication requirements**:
- Protected packages remain conservative by default.
- Capability reports must show hook/root/package/freezer readiness before
  control is trusted.
- Binder freezer is reported `untested`, not available, until a safe probe is
  implemented.
- Release notes must state that `3.2.0SelfUse` is validated for the owner
  device baseline only and does not claim general protection against exploit
  chains.

**Alternatives considered**:
- Market freezing as a full security boundary: rejected because it would be
  technically false and unsafe.
- Disable all risky apps immediately at boot: rejected because it would increase
  boot-loop and foreground usability risk.
