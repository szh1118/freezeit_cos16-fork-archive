# Research: Android 16 OnePlus 13 Port

## Decision: Use `/home/admin/code/Rom/oneplus13.zip` as the compatibility baseline

**Rationale**: The spec limits the feature to one ROM archive. The archive exists locally at `/home/admin/code/Rom/oneplus13.zip` and is about 8.5 GB. Compatibility evidence must identify ROM build identity, Android version, device family, and framework/runtime observations from this archive, then compare with the installed phone build as a warning-only condition.

**Alternatives considered**:

- Use the installed device only: rejected because the spec requires evidence from the provided ROM archive.
- Support multiple OnePlus/OxygenOS builds: rejected because it violates the single-ROM scope.

## Decision: Use MIO-KITCHEN-SOURCE for ROM unpacking after repairing local dependencies

**Rationale**: The user explicitly approved the local ROM unpacking workflow based on `/home/admin/code/MIO-KITCHEN-SOURCE`. The checkout is present and documents payload, ext4, erofs, boot, and other Android image support. A local smoke run currently fails with `No module named 'google'`, so implementation must first set up the Python environment from `requirements.txt` and confirm MIO can unpack enough of `oneplus13.zip` to read build props and framework artifacts.

**Alternatives considered**:

- Use only raw `unzip`: acceptable as a fallback for metadata, but insufficient if framework images need unpacking.
- Download a different ROM tool: rejected unless MIO cannot unpack the required artifacts after dependency repair.

## Decision: Use the existing manager app project and Gradle wrapper

**Rationale**: `freezeitApp` already declares Android Gradle Plugin 8.3.2, Gradle wrapper 8.4, Java 11 compile options, package `io.github.jark006.freezeit`, and dependencies used by the current manager. Gradle current user guide confirms the wrapper is the recommended way to run a build. The local wrapper works via `bash freezeitApp/gradlew --version`; the script lacks direct executable permission, so build commands should call it through `bash`.

**Alternatives considered**:

- Upgrade AGP/compile SDK preemptively: rejected unless implementation proves Android 16 requires it for the manager app.
- Replace the Android project structure: rejected because it increases risk without satisfying a spec requirement.

## Decision: Build native module artifacts with the local Linux NDK

**Rationale**: Existing PowerShell scripts compile C++20 with Android NDK and package the Magisk tree, but they assume Windows paths. The local Linux NDK exists at `/home/admin/Android/Sdk/ndk/28.2.13676358`, with a Linux x86_64 LLVM toolchain and aarch64 Android clang++ wrappers including API 31 through API 35. The OnePlus 13 self-use target is ARM64, but the existing `customize.sh` also handles x64. Implementation should either produce both current artifact names or update the packaging script consistently for ARM64-only self-use.

**Alternatives considered**:

- Keep only the Windows PowerShell build: rejected because the current workspace is Linux.
- Build inside the Android app project with CMake: rejected unless native build failures prove the existing direct clang path is unsuitable.

## Decision: Keep the existing manager/native protocol

**Rationale**: The manager already calls `ManagerCmd` IDs for status, logs, config read/write, settings, real-time info, Xposed logs, and process state. The native service handles these commands in `Server.handleCmd`. This is the simplest contract surface for status, diagnostics, and configuration preservation.

**Alternatives considered**:

- Add a new Binder/service API: rejected because existing local socket commands cover the required manager/module interaction.
- Store validation evidence only in the app: rejected because native and boot logs are required for module-level diagnostics.

## Decision: Verify Android 16 hook compatibility from the target ROM before changing hooks

**Rationale**: Current hook code contains version branches and comments for Android 10-14 era framework internals, including ActivityManagerService, BroadcastQueue/BroadcastQueueImpl, AppOpsService, NetworkManagementService/Netd, display power state, process records, foreground UID tracking, pending UID tracking, ANR and broadcast hooks. Android 16/OnePlus changes must be derived from the target ROM framework artifacts and runtime Xposed logs rather than guessed.

**Alternatives considered**:

- Assume Android 14 hooks still work: rejected because the spec requires Android 16 compatibility evidence.
- Rewrite the hook layer wholesale: rejected because current hook logic already provides the required behavior and should be adapted only where verified mismatches exist.

## Decision: Treat protected-state detection as ROM/framework research

**Rationale**: The spec requires protecting unselected system apps, current foreground app, media playback, calls, audio recording, and screen recording. Current code has default system-app whitelist behavior, foreground UID tracking through Xposed/local socket paths, and `/dev/snd` playback detection. Implementation must verify and, if needed, extend these mechanisms against the target ROM for calls, recording, and screen recording.

**Alternatives considered**:

- Add a generic "detection failed" product fallback: rejected after clarification; state detection reliability is an implementation responsibility.
- Protect all apps whenever any media subsystem is active: rejected unless ROM research shows per-app attribution is unavailable.

## Decision: Preserve existing upgrade configuration files

**Rationale**: `customize.sh` already copies `appcfg.txt`, `applabel.txt`, and `settings.db` from an existing `/data/adb/modules/freezeit` installation. This directly supports FR-005 and should be kept unless validation proves a format migration is needed.

**Alternatives considered**:

- Reset config on install: rejected because it violates FR-005.
- Introduce a new config store: rejected because it adds migration risk with no current requirement.

## Decision: Use root/module manager disable or uninstall as the recovery path

**Rationale**: The spec clarification states recovery is through the root/module manager. Current module scripts already check `disable` and `remove` flags in `service.sh`; `uninstall.sh` removes the app and `/sdcard/Android/freezeit*` after login. Validation must prove disabling or uninstalling through the root/module manager restores normal phone operation within 10 minutes.

**Alternatives considered**:

- Add a manager-app pause switch: rejected because it is outside the clarified recovery requirement.
- Add an offline rescue workflow: rejected for this feature scope unless implementation validation shows root/module manager recovery is insufficient.

## Decision: Record mismatch as warning-only

**Rationale**: The user clarified that if the installed phone build differs from the ROM archive, service startup and control operations must still be allowed, with only a warning in logs. The compatibility report must capture the mismatch, but it must not block runtime behavior.

**Alternatives considered**:

- Disable all control operations on mismatch: rejected by clarification.
- Refuse module startup on mismatch: rejected by clarification.
