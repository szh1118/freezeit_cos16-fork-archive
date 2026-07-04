# Protected-State Matrix

## Validation Scope

Confirm Freezeit does not freeze or disrupt protected states on the target
OnePlus 13 Android 16 ROM.

## Matrix

| Protected state | Test app/source | Expected behavior | Required evidence | Result |
| --- | --- | --- | --- | --- |
| Unselected system app | `UNVERIFIED` | Remains unselected and is not frozen. | App classification/config, native log, process state. | `UNVERIFIED` |
| Current foreground app | `UNVERIFIED` | Foreground UID is skipped. | Foreground app visible, manager/native foreground diagnostics, no freeze. | `UNVERIFIED` |
| Media playback | `UNVERIFIED` | Playing app is skipped or restored without playback disruption. | Playback active, native audio/media diagnostic, no freeze. | `UNVERIFIED` |
| Call | `UNVERIFIED` | Call app/UID is skipped and call remains active. | Call active indicator, detector/hook evidence, no freeze. | `UNVERIFIED` |
| Audio recording | `UNVERIFIED` | Recording app/UID is skipped and recording continues. | Recording indicator/source app, detector/hook evidence, no freeze. | `UNVERIFIED` |
| Screen recording | `UNVERIFIED` | Screen-recording app/UID is skipped and recording continues. | Screen recording active, detector/hook evidence, no freeze. | `UNVERIFIED` |

## Required Result Semantics

- `PASS`: protected state remains usable and logs show why control was skipped
  or not applied.
- `DEGRADED`: protected state remains usable but attribution is broad or
  diagnostic detail is incomplete.
- `FAIL`: protected state is frozen, disrupted, or left unusable.
- `UNVERIFIED`: check has not been run on the target phone.

## Static System-App Classification Check

- Source: `freezeitVS/include/managedApp.hpp`
- Result: `PASS` for default classification.
- Evidence:
  - `updateAppList()` reads all packages and third-party packages, then sets
    `freezeMode = WHITELIST` when a UID is not in the third-party set.
  - `applyCfgTemp()` also protects known system/vendor package prefixes with
    `WHITELIST`.
  - Trusted packages, forced whitelist packages, input methods, and the launcher
    are set to `WHITEFORCE`.
- Caveat: explicit existing/user config can override a UID after default
  classification; target-device validation must confirm unselected system apps
  remain unselected and not frozen.

## Static Media Playback Check

- Source: `freezeitVS/include/systemTools.hpp`
- Result: `DEGRADED` until target-phone validation.
- Evidence:
  - `SystemTools::sndThreadFunc()` watches `/dev/snd` for `pcm*...p`
    playback device open/close events.
  - `SystemTools::isAudioPlaying` becomes true while playback devices are open.
  - `Freezer::getFreezeSkipReason()` skips freeze operations while
    `isAudioPlaying` is true and writes a manager-visible diagnostic.
- Caveat: this is broad playback protection. It avoids freezing during active
  playback, but it does not attribute playback to a specific UID.

## Static Call, Recording, And Screen-Recording Check

- Source: `freezeitVS/include/systemTools.hpp`,
  `freezeitVS/include/freezer.hpp`
- Result: `DEGRADED` until target-phone validation.
- Evidence:
  - `SystemTools::protectedStateThreadFunc()` polls `dumpsys telecom` and
    `dumpsys media_projection` for active call/projection markers.
  - `SystemTools::sndThreadFunc()` tracks `/dev/snd` capture devices ending in
    `c` as active audio recording/capture.
  - `Freezer::getFreezeSkipReason()` skips freeze operations while call,
    capture, or screen-recording/projection state is active.
  - State transitions and skip reasons are written to the native log.
- Caveat: these are broad global protections, not per-UID attribution. The
  target phone must confirm the ROM emits the expected `dumpsys` and `/dev/snd`
  signals during actual call, recording, and screen-recording scenarios.

## Static Protected-State Skip Check

- Source: `freezeitVS/include/freezer.hpp`
- Result: `PASS` for local skip-gate wiring; runtime rows remain
  `UNVERIFIED` until target-device execution.
- Evidence:
  - `Freezer::handleProcess()` calls `getFreezeSkipReason()` before freeze work.
  - Whitelisted/unselected apps, including unselected system apps, are skipped.
  - Current foreground UID is skipped.
  - Active audio playback, audio capture, call state, and screen
    recording/projection globally skip freeze.
  - Skip reasons are written to the manager-visible native log.
- Remaining `UNVERIFIED` states: all matrix rows still need target-device
  validation.
