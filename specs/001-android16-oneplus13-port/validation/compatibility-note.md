# Compatibility Note

## ROM Archive

- Path: `/home/admin/code/Rom/oneplus13.zip`
- File size: `8090.2M`
- Capture timestamp: 2026-07-03
- Status: `PASS` for OTA metadata extraction.

## Build Identity

- Android version: `16`
- Device family/product: `CPH2649IN` / `OP5D55L1`
- Build ID: `BP2A.250605.015`
- Incremental: `V.R4T3.1338e95_e24685_de185d`
- Fingerprint:
  `OnePlus/CPH2649IN/OP5D55L1:16/BP2A.250605.015/V.R4T3.1338e95_e24685_de185d:user/release-keys`
- Security patch: `2025-12-01`
- OTA version: `CPH2649_11.F.83_2830_202512070007`
- Display version: `CPH2649_16.0.2.402(EX01)`

## Unpacking Method

- Primary workflow: `/home/admin/code/MIO-KITCHEN-SOURCE`
- Command:
  `unzip -p /home/admin/code/Rom/oneplus13.zip META-INF/com/android/metadata`
- Fallback command:
  `unzip -p /home/admin/code/Rom/oneplus13.zip payload_properties.txt`
- Payload extraction command:
  `PYTHONPATH=/home/admin/code/MIO-KITCHEN-SOURCE /home/admin/code/MIO-KITCHEN-SOURCE/.venv/bin/python -m src.core.payload_extract -t zip -i /home/admin/code/Rom/oneplus13.zip -o specs/001-android16-oneplus13-port/validation/rom-extract -X system`
- EROFS extraction command:
  `/home/admin/code/MIO-KITCHEN-SOURCE/bin/Linux/x86_64/extract.erofs -x -f -i specs/001-android16-oneplus13-port/validation/rom-extract/system.img -o specs/001-android16-oneplus13-port/validation/rom-extract/system-root`
- Output location:
  `specs/001-android16-oneplus13-port/validation/rom-extract/system-root`
- Status: `PASS` for OTA metadata, `system.img` extraction, and EROFS
  framework artifact extraction. MIO's Tk entrypoint remains unavailable in
  this headless shell, but its non-GUI payload extractor and bundled EROFS
  extractor were usable.

## Framework Artifacts Inspected

- ActivityManager: `services.jar`
- Broadcast handling: `services.jar`
- AppOps/wakelock: `services.jar`
- Display/screen state: `services.jar`, `framework.jar`
- Foreground UID: `services.jar`
- Pending UID: `services.jar`
- Media/call/recording/screen-recording detection:
  `services.jar`, `telephony-common.jar`, `telecom.jar`
- OPlus framework overlays/services:
  `oplus-framework.jar`, `oplus-services.jar`
- Extraction evidence:
  `specs/001-android16-oneplus13-port/validation/android16-hook-map.md`

## Hook Compatibility Observations

- ActivityManager: `PASS` for class/field-name presence; runtime constructor
  hook still needs target-phone Xposed log confirmation.
- Broadcast: `PASS` for Android 14+ class and method-name presence; overloads
  still need runtime confirmation.
- AppOps/wakelock: `PASS` for class/method-name presence; constructor overload
  still needs runtime confirmation.
- Foreground UID: `PASS` for class/field-name evidence; runtime state values
  need device validation.
- Pending UID: `PASS` for relevant pending-intent and broadcast artifacts.
- Display/screen state: `DEGRADED`; display power artifacts exist, but
  `DisplayPowerController2` was not proven by the string sweep.
- Media/call/recording/screen-recording: media and screen-recording source
  artifacts exist. Call, audio-recording, and screen-recording protection is
  implemented as broad global skip gates and remains `UNVERIFIED` for runtime
  target-device behavior.

## Runtime Assumptions

- Root/module manager: required on target phone; exact manager/version
  `UNVERIFIED` because no ADB target is attached.
- LSPosed/Xposed enabled for manager app and Android/System Framework scope:
  required; runtime status `UNVERIFIED`.
- Native service starts after first unlock: implemented in `service.sh` and
  runtime status `UNVERIFIED`.

## Known Degraded Or Unverified Areas

- Display power exact hook target needs runtime Xposed log confirmation.
- Call, audio-recording, and screen-recording protection need target-device
  detector evidence.
- Device install, reboot, freeze/restore, protected-state, diagnostics, and
  recovery validation have not yet been run.

## Scope Statement

This evidence and build target only the self-use OnePlus 13 Android 16 ROM
represented by `/home/admin/code/Rom/oneplus13.zip`. It does not claim support
for other ROMs, devices, Android versions, or public release channels.
