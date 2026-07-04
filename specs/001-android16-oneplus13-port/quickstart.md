# Quickstart: Android 16 OnePlus 13 Port Validation

This guide describes the validation flow for the self-use build after implementation tasks are complete.

## Prerequisites

- Target ROM archive exists at `/home/admin/code/Rom/oneplus13.zip`.
- MIO-KITCHEN-SOURCE dependencies are installed. In this headless shell, use the
  non-GUI payload extractor and bundled `extract.erofs` rather than `tool.py`.
- Android NDK exists at `/home/admin/Android/Sdk/ndk/28.2.13676358`.
- Gradle wrapper is run with project directory, Java 17, and Android SDK env
  from the repository root.
- `adevtool` is available at `/home/admin/.local/bin/adevtool`.
- Target phone has a working root/module manager and LSPosed/Xposed hook environment.
- Validation apps are non-critical third-party apps selected by the maintainer.

## 1. Establish ROM Baseline

From the repository root:

```bash
rtk test -f /home/admin/code/Rom/oneplus13.zip
```

Use the repaired MIO workflow to unpack the target ROM enough to collect build properties and framework artifacts. Record the exact command, output path, build identity, framework artifacts inspected, and unverified items in the compatibility note defined by [contracts/compatibility-evidence.md](./contracts/compatibility-evidence.md).

Commands used locally:

```bash
rtk unzip -p /home/admin/code/Rom/oneplus13.zip META-INF/com/android/metadata
rtk env PYTHONPATH=/home/admin/code/MIO-KITCHEN-SOURCE /home/admin/code/MIO-KITCHEN-SOURCE/.venv/bin/python -m src.core.payload_extract -t zip -i /home/admin/code/Rom/oneplus13.zip -o specs/001-android16-oneplus13-port/validation/rom-extract -X system
rtk /home/admin/code/MIO-KITCHEN-SOURCE/bin/Linux/x86_64/extract.erofs -x -f -i specs/001-android16-oneplus13-port/validation/rom-extract/system.img -o specs/001-android16-oneplus13-port/validation/rom-extract/system-root
```

Expected result: the compatibility note identifies the ROM build, Android version, device family, relevant framework/runtime observations, and any degraded or unverified areas.

## 2. Build Manager App

```bash
rtk env JAVA_HOME=/usr/lib/jvm/java-17-openjdk ANDROID_HOME=/home/admin/Android/Sdk ANDROID_SDK_ROOT=/home/admin/Android/Sdk PATH=/usr/lib/jvm/java-17-openjdk/bin:$PATH bash freezeitApp/gradlew -p freezeitApp :app:assembleRelease
```

Expected result: a release APK is produced under `freezeitApp/app/build/outputs/apk/release/`, with application ID `io.github.jark006.freezeit`.

Local result: `freezeitApp/app/build/outputs/apk/release/freezeit_v3.1.0Alpha_release.apk`.

## 3. Build Native Module And Package

Use the implementation's Linux packaging path for `freezeitVS`, backed by the local NDK. The produced package must include:

```bash
rtk bash freezeitVS/build_arm64_linux.sh
rtk bash freezeitVS/build_pack_linux.sh
```

- Native Freezeit binary for the target phone architecture
- Manager APK
- `module.prop`
- `service.sh`
- `customize.sh`
- `uninstall.sh`
- Existing config seed files and changelog files

Expected result: one installable Magisk/KernelSU zip intended only for the target OnePlus 13 ROM.

Local result:
`freezeitRelease/freezeit_oneplus13_android16_selfuse_v3.1.0Alpha_301000.zip`.

## 4. Inspect Package Before Device Install

Verify the zip contains the required module metadata, scripts, native binary, and APK. Confirm the package or accompanying note states this is a self-use target-ROM build and does not claim public support.

```bash
rtk bsdtar -tf freezeitRelease/freezeit_oneplus13_android16_selfuse_v3.1.0Alpha_301000.zip
rtk sha256sum freezeitRelease/freezeit_oneplus13_android16_selfuse_v3.1.0Alpha_301000.zip freezeitVS/magisk/freezeitARM64 freezeitApp/app/build/outputs/apk/release/freezeit_v3.1.0Alpha_release.apk
```

Expected result: artifact inspection passes before touching the device.

## 5. Install And Reboot Validate

Install the module through the target phone's root/module manager, reboot, unlock the device, and open the manager app.

Expected results:

- Phone reaches launcher after install.
- Manager shows status, version, and logs within 60 seconds after first unlock.
- App control does not start before first user unlock and service/hook readiness.
- Repeat reboot validation for 3 consecutive reboots.

## 6. Validate Build Mismatch Warning

Compare the ROM baseline build identity with the installed phone build. If they differ, confirm the mismatch is logged as a warning and does not block service startup or control operations.

Expected result: mismatch is diagnostic-only.

## 7. Validate Freeze And Restore

Configure at least 3 non-critical third-party apps for freezing. For each app:

1. Send the app to background.
2. Confirm control is applied within 30 seconds.
3. Bring the app back from launcher or recent tasks.
4. Confirm the app is usable within 5 seconds.

Expected result: all selected apps pass background control and foreground restore; at least 9 of 10 restore attempts complete within 5 seconds.

## 8. Validate Protected States

Confirm these states are not frozen:

- System apps that are not explicitly selected
- Current foreground app
- App playing media
- App involved in a call
- App recording audio
- App recording the screen

Expected result: protected apps remain usable and diagnostics show why control was skipped or not applied where relevant.

## 9. Validate Diagnostics

Collect manager-visible logs and any file logs for install, boot, service startup, hook readiness, build mismatch, freeze/unfreeze, protected-state checks, and failures.

Expected result: every observed failure or unsupported operation has actionable diagnostics.

## 10. Validate Recovery

Disable or uninstall the module through the root/module manager, then reboot if required by the manager.

Expected result: normal phone operation returns within 10 minutes without data loss and without persistent frozen state.

## Required Final Gates

Before completion is claimed:

```bash
/brooks-review
/speckit-converge
```

Both must pass or any remaining findings must be explicitly accepted by the human owner.
