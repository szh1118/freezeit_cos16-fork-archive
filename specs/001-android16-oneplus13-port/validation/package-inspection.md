# Package Inspection

Captured: 2026-07-03

## Artifact

- Package:
  `freezeitRelease/freezeit_oneplus13_android16_selfuse_v3.1.0Alpha_301000.zip`
- SHA-256:
  `8025d22f87c97d4573f696850ec6482b7ecab5bf28cc8b824a95f5fa15401a07`
- Native ARM64 binary SHA-256:
  `55f187f464d242899da14479b2806dd91d52dbd5f63b321182cfa5eb82af6643`
- Manager APK SHA-256:
  `e14964b78f9e58faac547484f8b9bd1344949f8a95437c3d4ec9490aba96c523`

## Required Contents

| Required item | Archive path | Status |
| --- | --- | --- |
| Module metadata | `./module.prop` | `PASS` |
| No system overlay flag | `./skip_mount` | `PASS` |
| Service script | `./service.sh` | `PASS` |
| Customize script | `./customize.sh` | `PASS` |
| Uninstall script | `./uninstall.sh` | `PASS` |
| ROM baseline metadata | `./rom_baseline.prop` | `PASS` |
| ARM64 native binary | `./freezeitARM64` | `PASS` |
| Manager APK | `./freezeit.apk` | `PASS` |
| App config seed | `./appcfg.txt` | `PASS` |
| App label seed | `./applabel.txt` | `PASS` |
| Changelog | `./changelog.txt` | `PASS` |
| Magisk installer metadata | `./META-INF/com/google/android/update-binary`, `./META-INF/com/google/android/updater-script` | `PASS` |

## US1 Pass/Fail Criteria

| Check | PASS | FAIL |
| --- | --- | --- |
| Self-use scope | Package name, evidence note, or package metadata identifies OnePlus 13 Android 16 self-use target and makes no public support claim. | Package name or docs imply public/multi-ROM support. |
| Module metadata | `module.prop` exists and defines `id`, `name`, `version`, `versionCode`, and author metadata. | Missing `module.prop` or missing required metadata fields. |
| No system overlay flag | `skip_mount` exists because this module does not ship a `system/` overlay. | Magisk may attempt to mount a non-existent or unintended overlay. |
| Service script | `service.sh` exists and starts only after boot/user-unlock checks and honors disable/remove flags. | Service starts before storage/user unlock or ignores disable/remove flags. |
| Manager APK | Exactly one manager APK is packaged as `freezeit.apk` and install handling exists in `customize.sh`. | APK is absent, ambiguous, or not handled by install script. |
| Native binary | ARM64 native binary exists as `freezeitARM64`; `customize.sh` maps it to `freezeit` for `ARCH=arm64`. | ARM64 binary absent or install script cannot select it. |
| ROM baseline | `rom_baseline.prop` exists with target ROM identity fields. | Baseline metadata absent. |
| Config seeds/preservation | Seed text files exist and upgrade preservation for `appcfg.txt`, `applabel.txt`, and `settings.db` is documented. | Seed files absent or upgrade preservation absent. |
| Zip readability | `bsdtar -tf` can list the package contents without error. | Archive cannot be read. |

## Notes

- `settings.db` is not present as a seed file in the current Magisk source tree.
  Upgrade preservation for an existing installed `settings.db` is handled by
  `customize.sh` and recorded in `config-preservation.md`.
- The package name includes the self-use target scope:
  `oneplus13_android16_selfuse`.
- The package does not claim support for other ROMs, devices, Android versions,
  or public release channels.
- Latest package was rebuilt after ROM baseline warning, hook readiness gate,
  protected-state diagnostics, call/audio-capture/screen-projection skip gates,
  Magisk `skip_mount`, LSPosed metadata packaging, LSPosed libxposed API 102
  Java entry/backend adaptation, and changelog updates.
