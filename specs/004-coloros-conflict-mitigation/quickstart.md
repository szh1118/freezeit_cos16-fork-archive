# Quickstart: ColorOS Conflict Mitigation

## Automated Validation

From the repository root:

```sh
rtk sh scripts/test-coloros-athena-hook.sh
```

Expected outcome:

- The test confirms Athena is present in both Xposed scope files.
- The test confirms legacy and modern hook dispatch include Athena.
- The test confirms the ColorOS hook contains strategy, utility, and GuardElf diagnostic targets.

## Android Build

From the repository root:

```sh
rtk sh -c 'cd freezeitApp && ./gradlew assembleDebug'
```

Expected outcome:

- The Android app compiles with the new hook module and scope configuration.

## Manual Device Scenario

1. Enable the Freezeit Xposed module for `android`, `io.github.jark006.freezeit`, and `com.oplus.athena`.
2. Reboot or restart the Xposed environment.
3. Open Freezeit Xposed logs and confirm `Freezeit[OplusAthena]` hook attempts are logged.
4. Trigger a ColorOS battery/background cleanup scenario for a managed app.
5. Confirm Athena cleanup hooks report success and Freezeit no longer repeatedly refreezes the same app immediately after Athena cleanup.
