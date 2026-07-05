# Quickstart: Download Freeze Deferral

## Automated Validation

From the repository root:

```sh
rtk sh scripts/test-download-freeze-deferral.sh
rtk sh scripts/test-legacy-refreeze-schedule.sh
```

Expected outcome:

- The download deferral test compiles a small helper test binary and verifies package matching, first-sample wait, high-speed deferral, threshold boundary, and non-candidate behavior.
- The legacy refreeze test remains green.

## Native Build

From the repository root:

```sh
rtk sh freezeitVS/build_arm64_linux.sh
```

Expected outcome:

- The legacy native daemon compiles for ARM64 with the new download deferral helper included by `freezer.hpp`.

## Manual Device Scenario

1. Start a download in a matching cloud drive app.
2. Send the app to the background and wait for its normal freeze timeout.
3. Open the manager log page.
4. Confirm the log reports a download-speed deferral while the app is receiving more than 5 MiB/s.
5. Stop or throttle the download, wait for the retry interval, and confirm the app can freeze normally.
