# Manager/Native Protocol Check

Captured: 2026-07-03

## Source Files

- Manager command constants:
  `freezeitApp/app/src/main/java/io/github/jark006/freezeit/ManagerCmd.java`
- Native command enum: `freezeitVS/include/utils.hpp`
- Native command handling: `freezeitVS/include/server.hpp`

## Command Mapping

| Manager constant | Java ID | Native enum | Native handler | Status |
| --- | ---: | --- | --- | --- |
| `getPropInfo` | 2 | `MANAGER_CMD::getPropInfo` | Returns module metadata, CPU cluster, module env, freezer mode, Android version, kernel version, memory size | `PASS` |
| `getChangelog` | 3 | `MANAGER_CMD::getChangelog` | Returns changelog buffer | `PASS` |
| `getLog` | 4 | `MANAGER_CMD::getLog` | Returns native log buffer | `PASS` |
| `getAppCfg` | 5 | `MANAGER_CMD::getAppCfg` | Returns repeated `uid`, `freezeMode`, `isPermissive` int tuples | `PASS` |
| `getRealTimeInfo` | 6 | `MANAGER_CMD::getRealTimeInfo` | Validates 12-byte payload, returns chart and runtime text | `PASS` |
| `getSettings` | 8 | `MANAGER_CMD::getSettings` | Returns settings bytes | `PASS` |
| `getUidTime` | 9 | `MANAGER_CMD::getUidTime` | Returns repeated UID time tuples | `PASS` |
| `getXpLog` | 10 | `MANAGER_CMD::getXpLog` | Requests Xposed log and logs LSPosed diagnostic if empty | `PASS` |
| `setAppCfg` | 21 | `MANAGER_CMD::setAppCfg` | Validates 12-byte tuple payload, updates managed app config, saves config, syncs Xposed | `PASS` |
| `setAppLabel` | 22 | `MANAGER_CMD::setAppLabel` | Updates labels, saves labels, syncs Xposed | `PASS` |
| `setSettingsVar` | 23 | `MANAGER_CMD::setSettingsVar` | Validates 2-byte payload, updates settings | `PASS` |
| `clearLog` | 61 | `MANAGER_CMD::clearLog` | Clears and returns native log | `PASS` |
| `printFreezerProc` | 62 | `MANAGER_CMD::getProcState` | Native enum uses a different name but the same ID and handler prints freezer process state | `PASS` |

## Compatibility Notes

- Existing command IDs are stable between Java constants and native enum values.
- `printFreezerProc` is Java-side naming; native code calls the same ID
  `getProcState`.
- Invalid or unsupported command IDs return `非法命令`; malformed payloads for
  `getRealTimeInfo`, `setAppCfg`, and `setSettingsVar` return diagnostic text
  instead of crashing in the checked handler paths.
- Future additions to `getPropInfo` must preserve the existing field order:
  `id`, `name`, `version`, `versionCode`, `author`, `cpuCluster`,
  `moduleEnv`, `workMode`, `androidVersion`, `kernelVersion`,
  `extMemorySize`.

## Pending Device Exercises

| Exercise | Command path | Expected result | Evidence | Status |
| --- | --- | --- | --- | --- |
| `getAppCfg` | Manager app app-list/config load | Native returns repeated `uid`, `freezeMode`, `isPermissive` int tuples. | App list visible and config values match seed or existing upgrade config. | `UNVERIFIED` |
| `setAppCfg` | Manager app changes selected apps/freeze mode | Native returns `success`, saves config, syncs Xposed. | Manager setting persists after app restart/reboot and native log records config change. | `UNVERIFIED` |
| `getLog` | Manager log screen | Native log text is visible, including startup and mismatch warnings if present. | Screenshot/log excerpt. | `UNVERIFIED` |
| `getXpLog` | Manager Xposed log/status path | Xposed log text is visible, or actionable LSPosed diagnostic appears if empty. | Screenshot/log excerpt. | `UNVERIFIED` |
| `printFreezerProc` | Manager diagnostic action | Native process/freezer state is appended to visible log. | Before/after log excerpt around selected app freeze. | `UNVERIFIED` |

No target phone is currently connected over ADB; `adb devices -l` returned an
empty device list on 2026-07-03.
