# US1 Build Evidence

Date: 2026-07-04

Scope: T037 build the Rust daemon, manager APK, and test Magisk zip.

## Environment

- Rustup user toolchain installed for this build gate.
- `rustc 1.96.1 (31fca3adb 2026-06-26)`
- `cargo 1.96.1 (356927216 2026-06-26)`
- Android SDK: `/home/admin/Android/Sdk`
- Android NDK linker:
  `/home/admin/Android/Sdk/ndk/28.2.13676358/toolchains/llvm/prebuilt/linux-x86_64/bin/aarch64-linux-android34-clang`
- Gradle Java: `/usr/lib/jvm/java-17-openjdk`

## Build Commands

All commands were run from the repository root unless noted.

```text
rtk sh -lc '. "$HOME/.cargo/env" && rustup target add aarch64-linux-android'
result: pass
```

```text
rtk sh -lc '. "$HOME/.cargo/env" && rustup component add rustfmt'
result: pass
```

```text
rtk sh -lc '. "$HOME/.cargo/env" && sh freezeitDaemon/scripts/build-android.sh'
result: pass
observed: Finished `release` profile [optimized]
```

```text
ANDROID_HOME=/home/admin/Android/Sdk JAVA_HOME=/usr/lib/jvm/java-17-openjdk rtk sh ./gradlew :app:assembleRelease
working directory: freezeitApp
result: pass
observed: BUILD SUCCESSFUL
```

```text
rtk sh -lc 'mkdir -p freezeitRelease && rm -f freezeitRelease/freezeit_us1_test.zip && cd /tmp/freezeit-us1-zip && jar cf "$OLDPWD/freezeitRelease/freezeit_us1_test.zip" . && cd "$OLDPWD" && sh scripts/validate-magisk-zip.sh freezeitRelease/freezeit_us1_test.zip'
result: pass
observed: magisk zip integrity: pass
```

```text
rtk sh -lc '. "$HOME/.cargo/env" && sh freezeitDaemon/scripts/test-host.sh'
result: pass
observed: 12 Rust tests passed
```

## Artifacts

```text
freezeitDaemon/target/aarch64-linux-android/release/freezeit
size: 447.0K
```

```text
freezeitApp/app/build/outputs/apk/release/freezeit_v3.1.0Alpha_release.apk
size: 3.1M
```

```text
freezeitRelease/freezeit_us1_test.zip
size: 1.7M
validation: scripts/validate-magisk-zip.sh pass
```

The test zip was staged from `freezeitVS/magisk`, with the newly
built manager APK copied to `freezeit.apk` and the newly built Rust daemon copied
to `freezeitRustARM64`. `customize.sh` handles that Rust artifact name and
installs it as `freezeit`.

## Scoped Brooks Review

Mode: PR Review

Scope: T037 build configuration, build outputs, and zip validation path.

Health Score: 100/100.

Finding summary: no Critical, Warning, or Suggestion findings.

Rationale:

- The Rust Android linker configuration points at the installed NDK linker rather
  than relying on an ambient PATH entry.
- The package validator accepts the Rust daemon artifact names that
  `customize.sh` now supports.
- The test zip contains module metadata, install/start scripts, manager APK, and
  daemon artifact, and it passed archive integrity validation.

## Scoped Speckit Convergence

Scope: T037 checked against US1 build requirements, `plan.md`, `tasks.md`, and
the constitution.

Convergence result: no additional work required for T037.

Requirement evidence:

- Rust daemon builds for `aarch64-linux-android`.
- Manager release APK builds successfully.
- Test Magisk zip exists and passes `scripts/validate-magisk-zip.sh`.
- Host Rust tests pass after the toolchain change.

Open limitations:

- This is not target-device validation. T038 remains required for install,
  reboot, unlock, manager readiness, missing LSPosed scope, and hook-inactive
  degraded evidence.
