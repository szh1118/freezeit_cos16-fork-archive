# Data Model: Android 16 OnePlus 13 Port

## Target ROM Baseline

Represents the single supported ROM source used for planning and validation.

**Fields**

- `archive_path`: absolute path to the ROM archive, fixed to `/home/admin/code/Rom/oneplus13.zip`
- `archive_size_bytes`: captured file size for traceability
- `device_family`: device family or product identity extracted from ROM properties
- `android_version`: Android version extracted from ROM properties
- `build_id`: build ID or incremental version extracted from ROM properties
- `build_fingerprint`: ROM fingerprint extracted from build props
- `framework_artifacts`: list of inspected framework files or images
- `compatibility_observations`: list of hook/runtime observations relevant to Freezeit
- `unverified_items`: list of artifacts or checks not completed

**Validation Rules**

- Must refer to exactly one ROM archive.
- Must record enough build identity to compare against the installed phone build.
- A mismatch with the installed phone build is a warning-only validation result.

## Self-Use Module Build

Represents the produced installable package for the target phone.

**Fields**

- `module_id`: expected `freezeit`
- `version_name`: version from `module.prop` and manager APK
- `version_code`: numeric version from `module.prop` and manager APK
- `package_path`: produced Magisk/KernelSU zip path
- `native_binaries`: included native binaries, expected to include ARM64 for OnePlus 13
- `manager_apk`: included `io.github.jark006.freezeit` APK
- `metadata_files`: `module.prop`, `service.sh`, `customize.sh`, `uninstall.sh`, changelog and config seed files
- `scope_note`: explicit statement that the build is only for this self-use target

**Validation Rules**

- Must include module service, manager app, and module metadata.
- Must preserve upgrade configuration files when present.
- Must not claim support for other devices, ROMs, Android versions, or public releases.

## Controlled App

Represents an app selected by the user for background control.

**Fields**

- `uid`: Android UID used by native control logic
- `package_name`: package identity
- `label`: display label
- `is_system_app`: whether the app is system/preinstalled according to app list classification
- `is_selected_for_freezing`: user opt-in state
- `freeze_mode`: existing Freezeit mode such as whitelist/freezer/signal/terminate
- `is_permissive`: existing mode-specific permissive flag
- `runtime_state`: current observed running/background/frozen/restored state
- `protected_state`: foreground, media playback, call, audio recording, or screen recording state
- `last_control_result`: most recent control result and diagnostic message

**Validation Rules**

- System apps are unselected by default.
- Unselected system apps must not be frozen.
- Apps in protected states must not be frozen.
- Failed or unsupported operations must leave the app usable and must produce diagnostics.

## Runtime Readiness

Represents whether app control operations may begin after boot.

**Fields**

- `boot_completed`: `sys.boot_completed` or equivalent boot state
- `first_user_unlock_complete`: storage/user unlock readiness
- `native_service_available`: native Freezeit service is reachable
- `hook_state_available`: Xposed/LSPosed hook state is available
- `manager_status_visible`: manager app can display status/version/logs
- `start_allowed`: derived readiness for app control operations

**Validation Rules**

- App control operations may start only after first user unlock.
- App control operations may start only after both native service and hook state are available.
- Manager status must become visible within 60 seconds after first post-install unlock.

## Validation Result

Represents evidence from ROM analysis, build inspection, and target-device checks.

**Fields**

- `check_name`: unique check name
- `category`: install, boot, service, hook, freeze, unfreeze, protected-state, diagnostics, recovery, or ROM baseline
- `status`: pass, degraded, failed, or unverified
- `evidence`: file path, log excerpt reference, command summary, or manual observation
- `timestamp`: time of evidence collection
- `affected_requirement_ids`: related FR or SC identifiers
- `notes`: short explanation of degraded or unverified behavior

**Validation Rules**

- Every failed compatibility or control operation observed during validation must have an actionable diagnostic.
- Reports must separate passing checks, degraded features, failed checks, and unverified areas.

## Recovery State

Represents the required recovery path if instability occurs.

**Fields**

- `root_manager_available`: whether the root/module manager can disable or uninstall the module
- `module_disabled`: whether the module is disabled through root/module manager
- `module_uninstalled`: whether the module is uninstalled through root/module manager
- `normal_operation_restored`: whether the phone returns to normal operation
- `recovery_duration_minutes`: time from recovery start to normal operation
- `data_loss_observed`: whether user data loss occurred

**Validation Rules**

- Disable or uninstall through the root/module manager must restore normal operation within 10 minutes.
- Reboot or module removal through the root/module manager must not leave persistent frozen state.
