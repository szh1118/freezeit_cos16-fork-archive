# US3 Target Classification Evidence

Date: 2026-07-04

Target: `3B1F4LE5MS142WJY` / OnePlus `CPH2653`

Scope: T063 target-device app list and protected classification validation.

## Device Commands

```text
rtk adb -s 3B1F4LE5MS142WJY shell 'pm list packages -U | head -20'
result: pass
observed: package inventory with UID values is available from package manager.
```

```text
rtk adb -s 3B1F4LE5MS142WJY shell 'pm list packages -3 -U | wc -l; pm list packages -s -U | wc -l'
result: pass
observed:
- third-party package records: 139
- system package records: 418
```

```text
rtk adb -s 3B1F4LE5MS142WJY shell 'pm list packages -U | grep -E "^package:(io.github.jark006.freezeit|com.topjohnwu.magisk|org.lsposed.manager|android|com.android.systemui|com.android.phone|com.android.launcher|com.google.android.inputmethod.latin|com.sohu.inputmethod.sogouoem) uid:"'
result: pass
observed:
- package:io.github.jark006.freezeit uid:10570
- package:com.google.android.inputmethod.latin uid:10189
- package:com.android.launcher uid:10255
- package:com.android.systemui uid:10260
- package:android uid:1000
- package:com.android.phone uid:1001
- package:com.sohu.inputmethod.sogouoem uid:10430
```

```text
rtk adb -s 3B1F4LE5MS142WJY shell 'cmd package resolve-activity --brief -a android.intent.action.MAIN -c android.intent.category.HOME | tail -5'
result: pass
observed: com.android.launcher/.Launcher
```

```text
rtk adb -s 3B1F4LE5MS142WJY shell 'settings get secure default_input_method'
result: pass
observed: com.sohu.inputmethod.sogouoem/.SogouIME
```

```text
rtk adb -s 3B1F4LE5MS142WJY shell 'su -c "pidof freezeit; ss -ltnp | grep 60613 || true"'
result: pass
observed:
- daemon PID: 3467
- daemon listening on 127.0.0.1:60613
```

## Host Regression Added From Device Evidence

The target's active input method is package `com.sohu.inputmethod.sogouoem`,
which is not one of the static protected package names. The classifier now
accepts a device-derived `ProtectedPackageContext` so launcher, input method,
root manager, and hook manager packages discovered from the device role state
are protected even when package names vary.

```text
rtk sh -lc '. "$HOME/.cargo/env" && cargo fmt --manifest-path freezeitDaemon/Cargo.toml && sh freezeitDaemon/scripts/test-host.sh'
result: pass
observed:
- protected_classification: 4 passed, including device role context coverage
- full Rust host suite: 37 passed
```

```text
ANDROID_HOME=/home/admin/Android/Sdk JAVA_HOME=/usr/lib/jvm/java-17-openjdk rtk sh ./gradlew :app:compileDebugJavaWithJavac
working directory: freezeitApp
result: pass
observed: BUILD SUCCESSFUL
```

## Scoped Brooks Review

Mode: PR Review

Scope: T063 target evidence plus classifier changes in
`freezeitDaemon/src/app/package_inventory.rs` and
`freezeitDaemon/tests/contract/protected_classification.rs`.

Health Score: 100/100.

Finding summary: no Critical, Warning, or Suggestion findings.

Rationale:

- Role-derived protection is represented as explicit classification context,
  avoiding hard-coded assumptions about third-party input method or launcher
  package names.
- Existing static protection remains for manager, known root/hook managers, and
  system-critical packages.
- No device policy state was mutated during validation.

## Scoped Speckit Convergence

Scope: T063 checked against US3 acceptance scenario 1, FR-008, FR-009,
SC-006, tasks, plan touch-points, and constitution.

Convergence result: no additional T063 tasks required.

Open limitations:

- Magisk and LSPosed manager packages were not visible under package-manager
  names matching `magisk`, `lsposed`, `kernelsu`, `apatch`, or `zygisk` on this
  target. The classifier still protects those roles when package inventory or
  role detection supplies package names.
