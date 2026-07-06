# Contract: Athena Hook Coverage

## Scope Contract

The release artifact must expose `com.oplus.athena` to both supported Xposed loading mechanisms:

- Legacy scope resource: `freezeitApp/app/src/main/res/values/arrays.xml`
- Modern scope list: `freezeitApp/app/src/main/resources/META-INF/xposed/scope.list`

## Dispatch Contract

`FreezeitHookEntry.handlePackage()` must dispatch:

- `android` to the existing Android system hooks.
- `com.miui.powerkeeper` to the existing MIUI hook.
- `com.oplus.athena` to the ColorOS Athena hook.

`ModernHook.onPackageReady()` must allow `com.oplus.athena` to reach `FreezeitHookEntry.handlePackage()`.

## Strategy Hook Contract

The ColorOS hook must attempt the external clear entry method for:

- `com.oplus.athena.systemservice.action.prockill.clear.externalclear.ForceStopStrategy`
- `com.oplus.athena.systemservice.action.prockill.clear.externalclear.KillPidStrategy`
- `com.oplus.athena.systemservice.action.prockill.clear.externalclear.KillUidStrategy`
- `com.oplus.athena.systemservice.action.prockill.clear.externalclear.c`

Each strategy entry returns an empty list before the vendor cleanup runs.

## Utility Hook Contract

The ColorOS hook must attempt shared cleanup utility methods in:

- `com.oplus.athena.systemservice.utils.s`
- `n1.m`

Force-stop methods return before execution. Boolean kill methods return `false`.

## Diagnostic Contract

The ColorOS hook must log GuardElf policy methods on:

- `com.oplus.athena.client.action.oplusguardelf.RemoteGuardElfService$1`

The diagnostic hooks must not short-circuit the original policy methods.
