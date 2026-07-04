# Contract: Manager Native Protocol

This contract documents the existing manager app to native service protocol that must be preserved unless implementation proves a change is required.

## Transport

- Manager app uses the existing `Utils.freezeitTask(...)` path.
- Native module service handles manager commands in `freezeitVS/include/server.hpp`.
- Replies are length-prefixed by the native service and read into the existing manager response buffer.

## Command Surface

| Command | ID | Direction | Payload | Reply | Required For |
|---------|----|-----------|---------|-------|--------------|
| `getPropInfo` | 2 | App to native | none | newline-separated module and runtime fields | Manager status, version, runtime diagnostics |
| `getChangelog` | 3 | App to native | none | changelog text | App information |
| `getLog` | 4 | App to native | none | native log text | Diagnostics |
| `getAppCfg` | 5 | App to native | none | repeated 3x int32 tuples: `uid`, `freezeMode`, `isPermissive` | Config preservation and app list display |
| `getRealTimeInfo` | 6 | App to native | 3x int32: chart height, width, available MiB | chart bytes plus text fields | Existing dashboard |
| `getSettings` | 8 | App to native | none | settings bytes | Existing settings UI |
| `getUidTime` | 9 | App to native | none | repeated 3x int32 tuples: `uid`, delta time, total time | Existing app time UI |
| `getXpLog` | 10 | App to native | none | Xposed log text or diagnostic fallback | Hook readiness diagnostics |
| `setAppCfg` | 21 | App to native | repeated 3x int32 tuples: `uid`, `freezeMode`, `isPermissive` | `success` or error text | User-selected freeze config |
| `setAppLabel` | 22 | App to native | newline-separated `uid` plus label strings | `success` or error text | App labels |
| `setSettingsVar` | 23 | App to native | 2 bytes: setting index and value | status text | Existing settings |
| `clearLog` | 61 | App to native | none | log text | Diagnostics |
| `printFreezerProc` | 62 | App to native | none | log text | Freeze state diagnostics |

## Compatibility Requirements

- Existing command IDs must remain stable for the manager app and native service.
- Any added diagnostic field in `getPropInfo` must preserve the existing field order so older parsing remains valid.
- Build mismatch warning must be visible in `getLog` output or another manager-accessible diagnostic log.
- Hook readiness failure must be visible through manager status/logs.
- `getAppCfg` and `setAppCfg` must preserve existing configuration semantics for upgrade installs.

## Validation

- Manager app opens and displays module status within 60 seconds after first post-install unlock.
- `getPropInfo`, `getLog`, `getAppCfg`, `setAppCfg`, `getXpLog`, and `printFreezerProc` must be exercised during validation.
- Invalid or unsupported command payloads must return diagnostic text rather than crashing the native service.
