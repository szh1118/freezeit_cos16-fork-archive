# freezeit_cos16

Freezeit / 冻它 的 COS16 自用维护版。

This workspace keeps the original Android manager, LSPosed module surface, and
Magisk packaging layout while moving new daemon control logic into
`freezeitDaemon/`. The verified target is OnePlus CPH2653 / Android 16 /
ColorOS 16.

Remote: https://github.com/szh1118/freezeit_cos16

## Layout

- `freezeitApp/`: Android manager APK and LSPosed/Xposed hook code.
- `freezeitDaemon/`: Rust daemon rewrite and host/device tests.
- `freezeitVS/`: legacy native service reference and Magisk module source.
- `freezeitRelease/`: release metadata and selected packaged artifacts.
- `specs/`: Spec Kit feature specs, tasks, and validation evidence.
- `scripts/`: repo-level build, packaging, and validation helpers.

## Build And Validate

- Target-device claims are limited to the recorded CPH2653 Android 16 baseline.
- Rust host checks run with `freezeitDaemon/scripts/test-host.sh`.
- Android manager compile checks run from `freezeitApp/` with
  `./gradlew :app:compileDebugJavaWithJavac`.
- Legacy package helper path is `freezeitVS/build_pack_linux.sh`.
- Release zips must pass `scripts/validate-release-zip.sh`.
- Device validation evidence lives under
  `specs/002-modern-freezer-rewrite/evidence/`.

Do not treat unchecked tasks in `specs/002-modern-freezer-rewrite/tasks.md` as
complete release behavior.
