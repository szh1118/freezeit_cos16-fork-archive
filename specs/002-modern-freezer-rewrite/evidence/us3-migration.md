# US3 Target Migration Evidence

Date: 2026-07-04

Target: `3B1F4LE5MS142WJY` / OnePlus `CPH2653`

Scope: T064 target-device legacy policy and label migration validation.

## Device Commands

```text
rtk adb -s 3B1F4LE5MS142WJY shell 'su -c "wc -l /data/adb/modules/freezeit/appcfg.txt /data/adb/modules/freezeit/applabel.txt; xxd -l 32 /data/adb/modules/freezeit/settings.db"'
result: pass
observed:
- /data/adb/modules/freezeit/appcfg.txt: 380 lines
- /data/adb/modules/freezeit/applabel.txt: 259 lines
- settings.db begins with bytes: 08 00 0a 04 14 00 02 00 ...
```

```text
rtk adb -s 3B1F4LE5MS142WJY shell 'su -c "head -5 /data/adb/modules/freezeit/appcfg.txt; echo LABELS; head -5 /data/adb/modules/freezeit/applabel.txt"'
result: pass
observed:
- policy line shape: `<package> <mode> <permissive>`
- label line shape: `<package>####<label>`
```

## Host Migration Validation

The host migration contract now covers:

- legacy policy parsing and mapping into `FreezePolicy`
- legacy label parsing with the `####` separator used by target files
- byte-for-byte settings preservation for `settings.db`
- manager binary app config triple compatibility

```text
rtk sh -lc '. "$HOME/.cargo/env" && cargo fmt --manifest-path freezeitDaemon/Cargo.toml && sh freezeitDaemon/scripts/test-host.sh'
result: pass
observed:
- policy_migration: 3 passed
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

Scope: T064 target evidence plus migration changes in
`freezeitDaemon/src/config/migration.rs` and
`freezeitDaemon/tests/contract/policy_migration.rs`.

Health Score: 100/100.

Finding summary: no Critical, Warning, or Suggestion findings.

Rationale:

- The migration parser keeps policy, label, and settings concerns explicit and
  does not mutate target files during validation.
- Invalid or empty legacy lines are ignored through the existing normalization
  path.
- Settings are preserved as opaque bytes because the legacy manager format is a
  fixed binary settings block.

## Scoped Speckit Convergence

Scope: T064 checked against US3 acceptance scenario 3, FR-002, FR-013, SC-007,
tasks, plan touch-points, and constitution.

Convergence result: no additional T064 tasks required.
