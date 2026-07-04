# Contract: LSPosed-Daemon Bridge

## Transport

- Existing bridge: Android `LocalServerSocket("FreezeitXposedServer")`.
- Client: daemon connects through abstract Unix socket name `\0FreezeitXposedServer`.
- Owner: LSPosed hook code in `freezeitApp`.
- Requirement: bridge is a readiness-gated source of system_server/package state. If unavailable, daemon enters degraded mode and does not run unsafe freeze actions.

## Existing Request Frame

The daemon sends:

```text
int32 command
int32 payload_len
payload bytes
```

The hook replies with command-specific integer or byte payloads.

## Existing Commands

Base code is CRC32 decimal value for `Freezeit`: `1359322925`.

| Command | Value | Purpose |
|---------|-------|---------|
| `GET_FOREGROUND` | base + 1 | Query foreground/currently visible UID state |
| `GET_SCREEN` | base + 2 | Query screen/interactive state |
| `GET_XP_LOG` | base + 3 | Read hook log |
| `SET_CONFIG` | base + 20 | Send current config to hook side |
| `SET_WAKEUP_LOCK` | base + 21 | Apply wake-lock handling |
| `BREAK_NETWORK` | base + 41 | Apply network break through system service |
| `UPDATE_PENDING` | base + 60 | Notify pending freeze app set |

## New Additive Commands

| Command | Proposed Value | Response | Purpose |
|---------|----------------|----------|---------|
| `GET_HOOK_HEALTH` | base + 70 | JSON | Report system_server hook readiness and package hook readiness |
| `GET_RUNTIME_APP_STATES` | base + 71 | JSON | Report selected AMS process fields such as foreground, cached, pending freeze, frozen |
| `GET_SYSTEM_FREEZER_HINTS` | base + 72 | JSON | Report whether system freezer state suggests postpone, skip, or safe control |

## LSPosed Modern API Requirements

- `META-INF/xposed/module.prop` keeps `minApiVersion=100`, `targetApiVersion=102`, and `staticScope=true`.
- `META-INF/xposed/java_init.list` points to `io.github.jark006.freezeit.hook.ModernHook`.
- `scope.list` includes `system` and the manager package.
- `ModernHook.onSystemServerStarting` remains the system_server entrypoint.
- `ModernHook.onPackageReady` remains the manager/package entrypoint.

## Failure Semantics

- Socket missing or connection refused: hook status is `missing`; daemon degrades.
- Hook responds but system_server fields are unavailable: hook status is `degraded`; daemon may read logs/config but must not use hook-dependent freeze decisions.
- Hook protocol version mismatch: daemon logs mismatch and uses root-side read-only capability checks only.
- Hook active but target package out of scope: manager reports scope issue and blocks unsafe control.

## Verification

- Manager must show hook readiness as active only after `GET_HOOK_HEALTH` succeeds.
- Disabling LSPosed scope must make daemon report degraded without freezing apps.
- Re-enabling scope and rebooting must restore active state within 30 seconds after first unlock.
