# Final Build Evidence

Date: 2026-07-04

Scope: T085 final Rust, Gradle, and package build checks.

## Verification Commands

```text
rtk sh -lc '. "$HOME/.cargo/env" && sh freezeitDaemon/scripts/test-host.sh'
result: pass
observed: full Rust host suite: 48 passed
```

```text
rtk sh -lc '. "$HOME/.cargo/env" && sh freezeitDaemon/scripts/build-android.sh'
result: pass
observed: release daemon built for aarch64-linux-android
```

```text
ANDROID_HOME=/home/admin/Android/Sdk JAVA_HOME=/usr/lib/jvm/java-17-openjdk rtk sh ./gradlew :app:assembleRelease
working directory: freezeitApp
result: pass
observed: BUILD SUCCESSFUL
```

## Post-Phase-9 Fix Re-run

After the Phase 9 fixes for manager log timeouts, real-time chart history,
temperature selection, batched procfs discovery, startup hook config sync, and
v2 diagnostic command IDs, the core build checks were run again on 2026-07-04:

```text
rtk sh freezeitDaemon/scripts/test-host.sh
result: pass
observed: 79 Rust unit/contract/integration tests passed, 0 failed
```

```text
rtk sh -lc '. "$HOME/.cargo/env" && sh freezeitDaemon/scripts/build-android.sh'
result: pass
observed: release daemon built for aarch64-linux-android
```

```text
rtk sh scripts/package-release.sh
result: pass
observed:
- release zip integrity: pass
- packaged release: `freezeitRelease/freezeit_3.2.0SelfUse.zip`
- final candidate sha256:
  a01bd6cde0de11cfe3a0e4daa69deeeb7c5004dec1546dd5cc4fbc60b0a7d73d
```

Target install smoke check for the rebuilt package:

```text
rtk adb -s 3B1F4LE5MS142WJY shell 'su -c "magisk --install-module /data/local/tmp/freezeit_3.2.0SelfUse.zip"'
result: pass
observed: Magisk module install completed

post-reboot daemon:
pid=3574
listener=127.0.0.1:60613
module binary size=834456
```

Manager APK check after Phase 9 evidence update:

```text
rtk sh -lc 'ANDROID_HOME=/home/admin/Android/Sdk JAVA_HOME=/usr/lib/jvm/java-17-openjdk sh ./gradlew :app:assembleRelease'
working directory: freezeitApp
result: pass
observed: BUILD SUCCESSFUL in 717ms; 42 actionable tasks, 1 executed, 41 up-to-date
```

```text
rtk sh scripts/package-release.sh
result: pass
observed:
- release zip integrity: pass
- packaged release: `freezeitRelease/freezeit_3.2.0SelfUse.zip`
- archive stores Magisk module entries without `./` prefixes
- release zip contains exactly one Rust daemon payload: `freezeitRustARM64`
- release zip contains one manager APK: `freezeit-3.2.0SelfUse.apk`
```

## Scoped Brooks Review

Mode: PR Review

Scope: T085 build scripts, release archive contents, and final local build
outputs.

Health Score: 100/100.

Finding summary: no Critical, Warning, or Suggestion findings.

Rationale:

- The package script now discovers the local Gradle release APK name, supports
  `zip` or `bsdtar`, and validates required archive entries after packaging.
- The `bsdtar` path now writes entries without `./` prefixes so Magisk detects
  the archive as a module.
- Build evidence covers Rust host tests, Android daemon release build, manager
  release APK assembly, and Magisk zip packaging.

## Scoped Speckit Convergence

Scope: T085 checked against final build expectations, release packaging plan,
FR-015, FR-016, SC-008, tasks, and constitution.

Convergence result: no additional build-script tasks required for T085.

Open limitations:

- T083 target release install/control/restore validation and T084 24-hour soak
  remain required before release completion.

## Post-T052 Re-run

After T052 fixed app cgroup discovery, the daemon control loop, foreground
fail-closed handling, and empty hook config encoding, the build checks were run
again on 2026-07-04:

```text
rtk sh -lc '. "$HOME/.cargo/env" &&
  cargo fmt --manifest-path freezeitDaemon/Cargo.toml &&
  cargo test --manifest-path freezeitDaemon/Cargo.toml --test freeze_unfreeze_state live_control_pass_treats_foreground_query_failure_as_fail_closed &&
  sh freezeitDaemon/scripts/test-host.sh &&
  sh freezeitDaemon/scripts/build-android.sh &&
  sh scripts/package-release.sh'
```

Observed:

```text
targeted foreground fail-closed test: pass
freezeitDaemon/scripts/test-host.sh: pass, 55 Rust contract/integration tests
freezeitDaemon/scripts/build-android.sh: pass
scripts/package-release.sh: release zip integrity pass
packaged release: `freezeitRelease/freezeit_3.2.0SelfUse.zip`
```

Manager release APK check:

```text
rtk sh -lc 'cd freezeitApp &&
  ANDROID_HOME=/home/admin/Android/Sdk JAVA_HOME=/usr/lib/jvm/java-17-openjdk
  sh ./gradlew :app:assembleRelease'

observed: BUILD SUCCESSFUL
```
