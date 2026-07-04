# Android 16 Hook Map

Captured: 2026-07-03

## ROM Artifact Extraction

- ROM archive: `/home/admin/code/Rom/oneplus13.zip`
- Payload manifest command:
  `PYTHONPATH=/home/admin/code/MIO-KITCHEN-SOURCE /home/admin/code/MIO-KITCHEN-SOURCE/.venv/bin/python -m src.core.payload_extract -t zip -i /home/admin/code/Rom/oneplus13.zip -o specs/001-android16-oneplus13-port/validation/rom-extract -X system`
- Extracted partition:
  `specs/001-android16-oneplus13-port/validation/rom-extract/system.img`
- Filesystem: EROFS
- EROFS extraction command:
  `/home/admin/code/MIO-KITCHEN-SOURCE/bin/Linux/x86_64/extract.erofs -x -f -i specs/001-android16-oneplus13-port/validation/rom-extract/system.img -o specs/001-android16-oneplus13-port/validation/rom-extract/system-root`
- Primary artifact:
  `specs/001-android16-oneplus13-port/validation/rom-extract/system-root/system/system/framework/services.jar`
- Supporting artifacts:
  `framework.jar`, `oplus-framework.jar`, `oplus-services.jar`, `telephony-common.jar`, `telecom.jar`, boot/framework vdex and oat files under the extracted `system/system/framework` tree.

## Existing Hook Expectations

- Class constants:
  `freezeitApp/app/src/main/java/io/github/jark006/freezeit/hook/Enum.java`
- Service/Xposed hook setup:
  `freezeitApp/app/src/main/java/io/github/jark006/freezeit/hook/android/FreezeitService.java`
- Broadcast hook setup:
  `freezeitApp/app/src/main/java/io/github/jark006/freezeit/hook/android/BroadCastHook.java`
- Native media/playback detector:
  `freezeitVS/include/systemTools.hpp`

## Compatibility Observations

| Area | ROM evidence | Existing implementation | Status |
| --- | --- | --- | --- |
| ActivityManager | `services.jar` dex strings include `com/android/server/am/ActivityManagerService`, `ActivityManagerService.java`, `mProcessList`, `mLruProcesses`, `UidObserverController`, and `registerUidObserver`. | `FreezeitService` hooks the `ActivityManagerService(Context, ActivityTaskManagerService)` constructor and reads `mProcessList.mLruProcesses`. | `PASS` for class/field-name presence; constructor signature must still be verified by Xposed log on device. |
| Broadcast | `services.jar` includes `BroadcastQueue`, `BroadcastQueueImpl`, `BroadcastQueueModernImplExt`, `BroadcastRecord`, `BroadcastFilter`, `deliverToRegisteredReceiverLocked`, `processCurBroadcastLocked`, and `skipReceiverLocked`. | `BroadCastHook` uses `BroadcastQueueImpl` on SDK 34+ with those method names. | `PASS` for class/method-name presence; exact overloads need runtime hook log confirmation. |
| AppOps/wakelock | `services.jar` includes `com/android/server/appop/AppOpsService`, `IAppOpsService`, `setUidMode`, and app-op callback classes. | `FreezeitService` finds `AppOpsService.setUidMode(int,int,int)` and hooks Android 14+ constructor shape. | `PASS` for class/method-name presence; constructor overload still needs runtime confirmation. |
| Display/screen state | `services.jar` strings include `DisplayPowerRequest`, `DisplayPowerController`, `DisplayPowerState`, `requestDisplayPower`, and `updateDisplayPowerStateLocked`. | `FreezeitService` hooks `DisplayPowerController.initialize(int)` and `DisplayPowerController2.initialize(int)` for SDK 34+. | `DEGRADED`: ROM evidence confirms display power classes, but `DisplayPowerController2` presence was not proven by the string sweep. Runtime `mPowerState` hook log is required. |
| Foreground UID | `services.jar` includes `ActivityManagerService`, `ProcessRecord`, `ProcessStateRecord`, `mCurProcState`, `UidObserver`, and `registerUidObserver`. | `FreezeitService.handleForeground` walks recent `mLruProcesses` and reads process state to derive foreground/permissive UIDs. | `PASS` for required class/field-name evidence; runtime state values need device validation. |
| Pending UID | `services.jar` includes `PendingIntentRecord`, `PendingIntentController`, `PendingStartActivityUids`, and pending broadcast strings. | Native sends `UPDATE_PENDING`; `FreezeitService.handlePendingApp` maintains `config.pendingUid`; `BroadCastHook` skips blocking pending UIDs. | `PASS` for relevant pending-intent and broadcast artifact presence. |
| Media playback | `services.jar` includes `AppMediaSessionTracker`, `MediaSessionRecord`, `MediaSessionManager`, and media session allowlist strings. | Native currently detects playback through `/dev/snd` activity in `SystemTools`. | `DEGRADED`: ROM has richer media-session sources, but the native detector remains device-behavior based and must be validated on target phone. |
| Call | `telephony-common.jar` and `telecom.jar` are present in the extracted framework tree. | `SystemTools` now polls `dumpsys telecom` for active-call markers and `Freezer::getFreezeSkipReason()` skips freeze globally while a call is active. | `DEGRADED`: implemented as broad active-call protection; target-device validation must confirm the ROM `dumpsys telecom` markers. |
| Audio recording | `services.jar` includes `AudioRecordingCallback`, `AudioRecordingDetector`, and `AudioRecordingConfiguration`. | `SystemTools::sndThreadFunc()` now tracks `/dev/snd` capture devices (`pcm*...c`) separately from playback and `Freezer::getFreezeSkipReason()` skips freeze globally while capture is active. | `DEGRADED`: implemented as broad capture protection; target-device validation must confirm `/dev/snd` capture events on this ROM. |
| Screen recording | `services.jar` includes `ScreenRecordingCallbackController`, `IScreenRecordingCallback`, `registerScreenRecordingCallback`, `onScreenRecordingStart`, and `visibleInScreenRecording`. | `SystemTools` now polls `dumpsys media_projection` for active projection markers and `Freezer::getFreezeSkipReason()` skips freeze globally while screen projection is active. | `DEGRADED`: implemented as broad projection protection; target-device validation must confirm the ROM `dumpsys media_projection` markers. |

## Summary

The ROM artifact inspection confirms the target Android 16 framework contains
the primary class and method names used by current Freezeit ActivityManager,
broadcast, AppOps, foreground UID, pending UID, media, and screen-state logic.
For US2, call, audio-recording, and screen-recording protection is implemented
as conservative global skip gates. Runtime target-phone validation is still
required before those protected-state rows can move out of `UNVERIFIED`.

## US2 Static Hook Verification

- `Enum.java`: no enum/class-name update is required for foreground UID or
  broadcast protection based on the extracted ROM artifacts. The existing
  `ActivityManagerService`, `ProcessRecord`, `ProcessStateRecord`,
  `BroadcastQueueImpl`, `BroadcastRecord`, and `BroadcastFilter` names are
  present in `services.jar`.
- `FreezeitService.java`: foreground UID tracking remains compatible at the
  class/field-name level because `ActivityManagerService`, `mProcessList`,
  `mLruProcesses`, process-state records, and UID observer strings are present.
  Runtime hook logs must still confirm constructor/field access on the phone.
- `BroadCastHook.java`: Android 14+ `BroadcastQueueImpl` path remains the
  correct static target for this ROM; `deliverToRegisteredReceiverLocked`,
  `processCurBroadcastLocked`, and `skipReceiverLocked` strings are present.
  Runtime hook logs must still confirm exact overload binding on the phone.
