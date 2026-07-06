# Research: ColorOS Conflict Mitigation

## Decision: Target `com.oplus.athena` first

**Rationale**: The reverse-engineering report shows Athena owns package-level background policy, while `com.oplus.battery` mainly drives UI/config and `poweropt-service` is a native power-hint bridge. Hooking Athena is the shortest path to reduce duplicate cleanup and refreeze work.

**Alternatives considered**:

- Hook `com.oplus.battery`: deferred because Battery delegates GuardElf decisions to Athena and is not the main cleanup executor.
- Hook native `poweropt-service`: rejected because the IDA notes show it is not the package whitelist/kill policy layer.
- Only read OPPO allowlists in the UI: useful for diagnostics, but it does not stop the cleanup fight.

## Decision: Short-circuit both strategy entries and utility exits

**Rationale**: External cleanup strategies can enter through force-stop, kill-pid, kill-uid, and force-stop-or-kill classes. The shared utility methods still matter because other Athena paths can call them directly. Covering both levels gives defense in depth while preserving non-Athena system-server hooks.

**Alternatives considered**:

- Hook only shared utility methods: lower maintenance, but loses visibility into strategy classes and may miss alternate paths.
- Hook only strategy classes: misses direct calls to shared kill/force-stop helpers.
- Hook `Process.killProcess` globally: rejected because it is too broad for a vendor-specific mitigation.

## Decision: Log GuardElf policy changes without blocking them

**Rationale**: GuardElf list edits are valuable evidence of ColorOS policy changes, but blocking them can make the Battery UI misleading. The MVP should stop expensive cleanup while leaving user-visible policy changes alone.

**Alternatives considered**:

- Block every GuardElf policy write: rejected because it changes Battery UI semantics.
- Ignore GuardElf entirely: rejected because users need a way to diagnose whether ColorOS is still changing protection policy.

## Decision: Use a source smoke test

**Rationale**: The hook targets vendor classes that only exist on the target phone. A repository-local test can still prevent accidental removal of package scope, dispatch, and key hook registrations.

**Alternatives considered**:

- Full JVM unit test: not useful without the Android/Xposed runtime and vendor classes.
- Device-only validation: useful later, but too slow and fragile as the only regression guard.
