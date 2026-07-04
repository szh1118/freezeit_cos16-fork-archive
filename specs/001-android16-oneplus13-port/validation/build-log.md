# Build Log

Captured: 2026-07-03

## Manager APK Baseline Build

### Documented Command Check

- Command: `bash freezeitApp/gradlew :app:assembleRelease`
- Result: `FAIL`
- Reason: when run from the repository root, Gradle reports that the
  workspace root does not contain `settings.gradle`.

### Corrected Linux Command

- Command:
  `env JAVA_HOME=/usr/lib/jvm/java-17-openjdk ANDROID_HOME=/home/admin/Android/Sdk ANDROID_SDK_ROOT=/home/admin/Android/Sdk PATH=/usr/lib/jvm/java-17-openjdk/bin:$PATH bash freezeitApp/gradlew -p freezeitApp :app:assembleRelease`
- Result: `PASS`
- Output summary: `BUILD SUCCESSFUL in 2m 20s`, 41 actionable tasks executed.
- Notes:
  - Java 26 is the system default and is not compatible with this Gradle 8.4
    build script evaluation path (`Unsupported class file major version 70`).
  - Build Tools 34 were installed by Gradle during the successful build.
  - Release signing uses the portable Gradle config in
    `freezeitApp/app/build.gradle`: local release keystore via properties or
    environment variables when present, otherwise Android debug signing for the
    self-use Linux build.
- APK:
  `freezeitApp/app/build/outputs/apk/release/freezeit_v3.1.0Alpha_release.apk`

## Native ARM64 Compile

- Command: `bash freezeitVS/build_arm64_linux.sh`
- Result: `PASS`
- Output:
  `Built freezeitVS/magisk/freezeitARM64`
- Compiler:
  `/home/admin/Android/Sdk/ndk/28.2.13676358/toolchains/llvm/prebuilt/linux-x86_64/bin/aarch64-linux-android35-clang++`
- Binary:
  `freezeitVS/magisk/freezeitARM64`

## Service Startup And ROM Baseline Warning Checks

- Command: `sh -n freezeitVS/magisk/service.sh`
- Result: `PASS`
- Command: `bash freezeitVS/build_arm64_linux.sh`
- Result after ROM baseline warning changes: `PASS`
- Behavior:
  - `service.sh` still waits for boot completion, writable `/sdcard`, then
    checks `disable` and `remove` flags before starting the native service.
  - `service.sh` compares `rom_baseline.prop` fingerprint and incremental
    values with device `getprop` values and writes warning-only mismatch lines
    to `boot.log`.
  - `Freezeit` reads `rom_baseline.prop`, compares fingerprint and incremental
    values with runtime system properties, and writes warning-only mismatch
    diagnostics into the manager-visible native log.
  - No mismatch branch exits, disables startup, or blocks control operations.

## Hook Readiness Gate

- Command: `bash freezeitVS/build_arm64_linux.sh`
- Result after readiness-gate changes: `PASS`
- Behavior:
  - `Freezer::processPendingApp()` delays pending freeze/control work until
    `isHookReadyForControl()` confirms the Xposed local socket responds.
  - Successful `GET_FOREGROUND` or `UPDATE_PENDING` socket exchanges mark the
    hook state ready.
  - Failed, malformed, or explicit-failure socket replies mark hook state not
    ready and write manager-visible diagnostics.
  - `getPropInfo` response formatting in `server.hpp` was not changed, so the
    existing field order is preserved.

## Protected-State And Failure Diagnostics

- Command: `bash freezeitVS/build_arm64_linux.sh`
- Result after protected-state diagnostics: `PASS`
- Behavior:
  - `Freezer::handleProcess()` skips freeze attempts for whitelisted/unselected
    apps, foreground apps, and global active audio playback.
  - Skip reasons are logged with `Skip freeze [...]`.
  - Empty process matches are logged rather than treated as a successful
    control operation.
  - Binder freeze failures and pending Binder transactions log that freeze was
    aborted/restored and leave the app usable for later retry.

## LSPosed Modern API 102 Adaptation Build

- Command:
  `env JAVA_HOME=/usr/lib/jvm/java-17-openjdk ANDROID_HOME=/home/admin/Android/Sdk ANDROID_SDK_ROOT=/home/admin/Android/Sdk PATH=/usr/lib/jvm/java-17-openjdk/bin:/home/admin/Android/Sdk/platform-tools:/home/admin/Android/Sdk/emulator:/home/admin/Android/Sdk/cmdline-tools/latest/bin:$PATH bash gradlew :app:assembleRelease --console=plain`
- Working directory: `freezeitApp`
- Result: `PASS`
- Output summary: latest rerun `BUILD SUCCESSFUL in 7s`, 42 actionable tasks,
  4 executed and 38 up-to-date.
- Adaptation evidence:
  - `compileOnly 'io.github.libxposed:api:102.0.0'`
  - `META-INF/xposed/java_init.list`
  - `io.github.jark006.freezeit.hook.ModernHook`
  - `io.github.jark006.freezeit.hook.ModernXposedBackend`
  - R8 rules include `-dontwarn io.github.libxposed.annotation.**` and
    `-adaptresourcefilecontents META-INF/xposed/java_init.list`
  - legacy `de.robv.android.xposed` calls isolated in
    `io.github.jark006.freezeit.hook.LegacyXposedBackend`
