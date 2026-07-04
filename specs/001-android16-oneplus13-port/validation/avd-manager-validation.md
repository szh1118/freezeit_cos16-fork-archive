# AVD Manager Validation

Captured: 2026-07-03

## Environment

- AVD: `freezeit_api35`
- System image: `system-images;android-35;google_apis;x86_64`
- Emulator: installed through SDK manager during this validation run.
- Device serial: `emulator-5554`
- Android release/API: `15` / `35`
- APK:
  `freezeitApp/app/build/outputs/apk/release/freezeit_v3.1.0Alpha_release.apk`
- Latest validated APK SHA-256:
  `e14964b78f9e58faac547484f8b9bd1344949f8a95437c3d4ec9490aba96c523`

## Actions And Evidence

| Check | Evidence | Result |
| --- | --- | --- |
| Install manager APK | `adb install -r -g ...` returned `Success`. | `PASS` |
| Resolve launcher activity | `cmd package resolve-activity --brief io.github.jark006.freezeit` returned `io.github.jark006.freezeit/.activity.Main`. | `PASS` |
| Launch manager | `am start -n io.github.jark006.freezeit/.activity.Main`; activity became resumed. | `PASS` |
| Privacy first-run dialog | UI tree captured `Privacy`, `REJECT`, and `ACCEPT`. | `PASS` |
| Enter Home after accept | UI tree captured title `Home`, status warning text, and bottom navigation. | `PASS` |
| Config navigation | UI tree captured title `Config`, search/help actions, app RecyclerView, and floating action buttons. | `PASS` |
| Log navigation | UI tree captured title `Log`, update/switch/help actions, log view, and floating action buttons. | `PASS` |
| Crash check | Captured logcat had no `FATAL EXCEPTION` for `io.github.jark006.freezeit`. | `PASS` |
| Post-API-102-adaptation smoke test | Reinstalled the rebuilt APK, launched `io.github.jark006.freezeit/.activity.Main`, verified Home, Config, and Log pages on `freezeit_api35`. | `PASS` |

## Artifacts

- `validation/avd/freezeit-main.png`
- `validation/avd/freezeit-after-accept.png`
- `validation/avd/freezeit-config.png`
- `validation/avd/freezeit-log.png`
- `validation/avd/freezeit-ui.xml`
- `validation/avd/freezeit-ui-after-accept.xml`
- `validation/avd/freezeit-config-ui.xml`
- `validation/avd/freezeit-log-ui.xml`
- `validation/avd/freezeit-logcat.txt`
- `validation/avd/freezeit-after-accept-logcat.txt`
- `validation/avd/freezeit-navigation-logcat.txt`
- `validation/avd/final-freezeit-ui.xml`
- `validation/avd/final-freezeit-log-ui.xml`
- `validation/avd/final-freezeit-log.png`
- `validation/avd/final-freezeit-logcat.txt`
- `validation/avd/modern-log-ui.xml`
- `validation/avd/modern-log.png`
- `validation/avd/modern-logcat.txt`

## Scope Limits

This validates the APK manager UI on a generic AVD only. It does not validate
Magisk install, root/module manager behavior, LSPosed system-framework hooks,
native service startup, freeze/unfreeze, protected-state behavior, or OnePlus 13
ROM runtime compatibility.
