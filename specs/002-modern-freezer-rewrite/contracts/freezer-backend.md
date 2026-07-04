# Contract: Freezer Backend

## Purpose

Define the safe interface between policy/scheduler logic and low-level Android process control.

## Backend Interface

The daemon core calls a safe backend interface with these operations:

```text
discover_capabilities() -> ControlCapability[]
discover_processes(package_name, uid) -> RuntimeProcess[]
can_freeze(processes, policy, hook_state) -> Decision
freeze(processes, reason) -> ControlOperation
unfreeze(processes, reason) -> ControlOperation
recover(package_name, uid) -> ControlOperation
```

The implementation may use unsafe Rust internally only inside `sys/*`.

## Required Pre-Checks

Before freeze:

- Daemon health is active.
- Package identity and UID are reconciled.
- PID still exists.
- PID UID matches package UID.
- Process is not foreground or user-visible under the selected foreground strategy.
- Protected package rules pass.
- Required freezer capability is available.
- Hook state is fresh when the decision depends on system_server fields.

Before unfreeze:

- PID exists or stale state is reconciled.
- If the process already exited, operation records recovery instead of failure.
- User-visible/foreground launch always has priority over background freeze timers.

## Backend Order

1. `SystemAwareCgroupBinderBackend`
   - Uses cgroup v2 `cgroup.freeze` paths discovered under `/sys/fs/cgroup/apps` and `/sys/fs/cgroup/system`.
   - Uses binder freezer capability where available.
   - Uses LSPosed/system_server bridge as safety evidence.

2. `SystemServerBridgeBackend`
   - Candidate only after implementation tests prove framework calls keep ActivityManager state coherent.
   - If promoted, record evidence and any behavior tradeoff before release.

3. `SignalBackend`
   - Uses `SIGSTOP`/`SIGCONT`.
   - Degraded fallback only; never default for protected/system apps.

4. `TerminateBackend`
   - Uses termination behavior only for explicit terminate policy or accepted fallback.
   - Must never run as an automatic replacement for freeze.

5. `Skip/Postpone`
   - Required fallback when safety is uncertain.

## State Integrity Rules

- A partial freeze must be logged as partial and immediately reconciled.
- If binder freeze succeeds but cgroup freeze fails, operation is partial and must attempt safe rollback or mark recovery required.
- If cgroup freeze succeeds but post-check sees unsafe foreground/system state, unfreeze immediately and log fallback.
- Daemon restart must read current process and cgroup state before issuing new operations.

## Verification

- Unit tests cover capability selection and fallback decisions.
- Contract tests simulate PID exit, UID mismatch, foreground transition, hook missing, cgroup missing, and binder rejection.
- Device tests validate freeze/unfreeze on at least three third-party apps and one multi-process app.
