# Contract: Compatibility Evidence

The implementation must produce a compatibility note and validation log for the single target ROM.

## Target ROM Baseline Note

Required sections:

- `ROM Archive`: absolute path, file size, and capture timestamp
- `Build Identity`: Android version, device family/product, build ID, incremental, fingerprint, security patch if available
- `Unpacking Method`: MIO-KITCHEN-SOURCE command or fallback command used, plus output location
- `Framework Artifacts Inspected`: framework jars, boot classpath artifacts, services artifacts, or extracted source/signature evidence
- `Hook Compatibility Observations`: ActivityManager, broadcast, AppOps/wakelock, foreground UID, pending UID, display/screen state, media/call/recording/screen-recording detection observations
- `Runtime Assumptions`: root/module manager, LSPosed/Xposed enablement, post-unlock service startup
- `Known Degraded Or Unverified Areas`: explicit list
- `Scope Statement`: no support claim for other ROMs/devices/releases

## Device Validation Log

Required sections:

- `Install`: module package path, root/module manager used, install result
- `Boot`: three reboot attempts and result
- `Service Startup`: first unlock time, native service availability, hook readiness, manager status visibility
- `Build Match Check`: ROM build identity vs installed phone build, warning-only if mismatch
- `Freeze/Unfreeze`: at least three non-critical third-party apps, freeze timing, restore timing, result
- `Protected States`: system-app default unselected state, current foreground app, media playback, call, audio recording, and screen recording protection checks
- `Diagnostics`: failure/warning logs and where they were collected
- `Recovery`: disable or uninstall through root/module manager and recovery duration

## Status Values

- `PASS`: observed behavior meets the spec
- `DEGRADED`: usable but outside ideal behavior, with diagnostic detail
- `FAIL`: blocks a functional requirement or success criterion
- `UNVERIFIED`: not checked yet and must be listed before completion can be claimed

## Completion Rule

The feature cannot be called complete while any core install, boot, service startup, freeze/unfreeze, protected-state, diagnostics, or recovery check remains `FAIL` or `UNVERIFIED`, unless the human owner explicitly accepts the risk in writing.
