# Contract: Manager-Daemon Protocol

## Transport

- Host: `127.0.0.1`
- Port: `60613`
- Scope: local device only.
- Compatibility: Rust daemon must support the existing v1 manager protocol before manager UI changes are required.

## V1 Frame

Request header is 6 bytes:

```text
bytes 0..3: little-endian uint32 payload_len
byte 4: command_id
byte 5: xor checksum of payload bytes, or 0 for empty payload
```

Payload follows immediately and is `payload_len` bytes.

Response uses the same 6-byte header shape followed by response payload.

## Existing Required Commands

The Rust daemon must implement these current `MANAGER_CMD` values:

| Command | ID | Request | Response |
|---------|----|---------|----------|
| `getPropInfo` | 2 | empty | `ID\nName\nVersion\nVersionCode\nAuthor\nclusterNum` |
| `getChangelog` | 3 | empty | text changelog |
| `getLog` | 4 | empty | text log |
| `getAppCfg` | 5 | empty | current policy text compatible with existing manager parsing |
| `getRealTimeInfo` | 6 | empty | existing raw bitmap plus metrics format, or compatible degraded response |
| `getSettings` | 8 | empty | 256-byte settings block |
| `getUidTime` | 9 | empty | `uid last_user_time last_sys_time user_time sys_time` rows |
| `getXpLog` | 10 | empty | hook log text |
| `setAppCfg` | 21 | policy payload | success/failure response compatible with manager |
| `setAppLabel` | 22 | label payload | success/failure response compatible with manager |
| `setSettingsVar` | 23 | two bytes: index/value | success/failure response compatible with manager |
| `clearLog` | 61 | empty | cleared log text |
| `getProcState` | 62 | empty | current process/control state text |

## V2 Diagnostic Commands

New commands must be additive and must not break existing manager calls.

| Command | Proposed ID | Response Format | Purpose |
|---------|-------------|-----------------|---------|
| `getHealthReport` | 71 | UTF-8 JSON | Full `ModuleHealth` and degraded reasons |
| `getCapabilityReport` | 72 | UTF-8 JSON | `ControlCapability` list with evidence |
| `getCompatibilityBaseline` | 73 | UTF-8 JSON | Current `CompatibilityBaseline` |
| `getOperationLogJson` | 74 | UTF-8 JSON lines | Structured recent `ControlOperation` entries |
| `runSelfCheck` | 75 | UTF-8 JSON | Non-destructive readiness probe result |

## Error Handling

- Invalid header length: close connection and log protocol error.
- Payload length above daemon limit: close connection and log protocol error.
- Checksum mismatch: close connection and log protocol error.
- Unknown command: return a structured error in v2 format when possible; v1 callers receive a failure-compatible response.
- Daemon degraded: read-only commands continue; unsafe mutation/control commands return failure with degraded reason.

## Compatibility Tests

- Existing manager request fixtures must round-trip through the Rust daemon parser.
- v1 responses for `getAppCfg`, `getSettings`, and `setSettingsVar` must remain parseable by current `freezeitApp`.
- v2 JSON must validate against the data model fields in `data-model.md`.
