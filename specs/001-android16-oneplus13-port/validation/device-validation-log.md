# Device Validation Log

## Target Device Prerequisites

| Field | Value |
| --- | --- |
| Device model | OnePlus 13 / `UNVERIFIED` exact device output |
| Installed ROM build | `UNVERIFIED` |
| Root/module manager | `UNVERIFIED` |
| LSPosed/Xposed framework | `UNVERIFIED` |
| Freezeit enabled for Android/System Framework scope | `UNVERIFIED` |
| Recovery path available | Root/module manager disable or uninstall |

Prerequisite status remains `UNVERIFIED` until checked on the target phone.

## Validation Status Summary

### PASS

- Local package artifact exists and is inspectable; see
  `package-inspection.md`.
- Local startup script and native build checks pass; see `build-log.md`.
- ROM baseline metadata is packaged; see `rom_baseline.prop` and
  `compatibility-note.md`.

### DEGRADED

- Media playback protection is currently broad/global through `/dev/snd`
  playback detection, not per-UID attribution.
- Display power hook compatibility is statically evidenced but exact runtime
  hook binding is not yet proven on the phone.

### FAIL

- No target-phone runtime failure has been observed because no target phone is
  connected.

### UNVERIFIED

- Target device prerequisites.
- Module install through root/module manager.
- Three reboot attempts and bootloop absence.
- First unlock service startup and manager status within 60 seconds.
- Installed phone build comparison against packaged ROM baseline.
- Freeze/restore behavior for three selected apps.
- Protected-state behavior for system apps, foreground app, media playback,
  calls, audio recording, and screen recording.
- Manager/native protocol exercises on device.
- Diagnostics for observed runtime failures.
- Root/module manager disable/uninstall recovery.

## Install

| Field | Value |
| --- | --- |
| Package path | `UNVERIFIED` |
| Root/module manager | `UNVERIFIED` |
| Install result | `UNVERIFIED` |
| Evidence | `adb devices -l` on 2026-07-03 showed no attached devices. |

Required evidence:

- Package path installed through the root/module manager.
- Root/module manager name and version if visible.
- Install success/failure text or screenshot reference.
- Any install-time warning, especially architecture, SDK, kernel, LSPosed, or
  conflicting tombstone module warnings.

## Boot

| Attempt | Result | Launcher reachable | Bootloop absent | Recovery intervention absent | Evidence |
| --- | --- | --- | --- | --- | --- |
| 1 | `UNVERIFIED` | `UNVERIFIED` | `UNVERIFIED` | `UNVERIFIED` | `UNVERIFIED` |
| 2 | `UNVERIFIED` | `UNVERIFIED` | `UNVERIFIED` | `UNVERIFIED` | `UNVERIFIED` |
| 3 | `UNVERIFIED` | `UNVERIFIED` | `UNVERIFIED` | `UNVERIFIED` | `UNVERIFIED` |

Required evidence:

- Device reaches launcher after each reboot.
- No bootloop, forced recovery, or manual module removal is needed.
- Any module disable/remove intervention is recorded as `FAIL` unless explicitly
  part of the recovery validation.

## Service Startup

| Field | Value |
| --- | --- |
| First unlock time | `UNVERIFIED` |
| Native service availability | `UNVERIFIED` |
| Hook readiness | `UNVERIFIED` |
| Manager status visible within 60 seconds | `UNVERIFIED` |
| Manager version visible | `UNVERIFIED` |
| Manager logs visible | `UNVERIFIED` |

Required evidence:

- First post-install unlock timestamp.
- `boot.log` or manager-visible status showing service startup.
- Native service reachable through manager status within 60 seconds.
- Hook readiness visible through `getXpLog` or manager log diagnostics.
- Version/status/log screen opens without manager crash.

## Build Match Check

| Field | Value |
| --- | --- |
| ROM baseline build | `UNVERIFIED` |
| Installed phone build | `UNVERIFIED` |
| Mismatch warning logged | `UNVERIFIED` |
| Startup/control blocked by mismatch | `UNVERIFIED` |

Required evidence:

- ROM baseline fingerprint or incremental from `rom_baseline.prop`.
- Installed phone `getprop ro.build.fingerprint`,
  `ro.build.version.incremental`, and `ro.build.version.security_patch`.
- If mismatch exists, warning-only log evidence and proof that startup/control
  is not blocked by the mismatch.

## Freeze/Unfreeze

| App | Package | Freeze within 30s | Restore within 5s | Result | Evidence |
| --- | --- | --- | --- | --- | --- |
| App 1 | `UNVERIFIED` | `UNVERIFIED` | `UNVERIFIED` | `UNVERIFIED` | `UNVERIFIED` |
| App 2 | `UNVERIFIED` | `UNVERIFIED` | `UNVERIFIED` | `UNVERIFIED` | `UNVERIFIED` |
| App 3 | `UNVERIFIED` | `UNVERIFIED` | `UNVERIFIED` | `UNVERIFIED` | `UNVERIFIED` |

## Protected States

| State | Expected behavior | Result | Evidence |
| --- | --- | --- | --- |
| Unselected system app | Not frozen | `UNVERIFIED` | `UNVERIFIED` |
| Current foreground app | Not frozen | `UNVERIFIED` | `UNVERIFIED` |
| Media playback | Not frozen | `UNVERIFIED` | `UNVERIFIED` |
| Call | Not frozen | `UNVERIFIED` | `UNVERIFIED` |
| Audio recording | Not frozen | `UNVERIFIED` | `UNVERIFIED` |
| Screen recording | Not frozen | `UNVERIFIED` | `UNVERIFIED` |

## Diagnostics

| Event | Diagnostic source | Result | Evidence |
| --- | --- | --- | --- |
| Install/startup warnings | Manager log or file log | `UNVERIFIED` | `UNVERIFIED` |
| Hook readiness | Manager status/log | `UNVERIFIED` | `UNVERIFIED` |
| Build mismatch | Manager log or file log | `UNVERIFIED` | `UNVERIFIED` |
| Control failures | Manager log or file log | `UNVERIFIED` | `UNVERIFIED` |

## Recovery

| Field | Value |
| --- | --- |
| Disable/uninstall path | Root/module manager |
| Recovery duration | `UNVERIFIED` |
| Normal operation restored | `UNVERIFIED` |
| Data loss observed | `UNVERIFIED` |
| Persistent frozen state absent | `UNVERIFIED` |
