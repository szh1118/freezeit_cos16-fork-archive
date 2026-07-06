# Data Model: ColorOS Conflict Mitigation

## Athena Hook Scope

Fields:

- `packageName`: The package that Xposed is loading.
- `frameworkScope`: The scope source that allows the hook to load.

Validation:

- `packageName` must equal `com.oplus.athena` for the ColorOS hook.
- Both legacy and modern scope declarations must include the package.

## Cleanup Strategy Hook

Fields:

- `className`: Athena external clear strategy class.
- `methodName`: Strategy entry method.
- `returnBehavior`: Empty cleanup result.

Validation:

- Strategy hooks cover force-stop, kill-pid, kill-uid, and force-stop-or-kill classes.
- Missing classes are logged but do not stop module loading.

## Cleanup Utility Hook

Fields:

- `className`: Athena shared cleanup utility class.
- `methodName`: Kill or force-stop method.
- `returnBehavior`: `null` for void methods, `false` for boolean kill methods.

Validation:

- Force-stop helper methods return before calling the vendor implementation.
- Kill helper methods report failure before process termination.

## GuardElf Policy Log

Fields:

- `packageName`: Package whose ColorOS power policy changed.
- `policy`: Policy value supplied by Athena/Battery.
- `switchState`: GuardElf switch value when available.

Validation:

- Logging does not alter the original method result.
- Missing GuardElf binder implementation is non-fatal.
