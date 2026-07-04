# US4 Recovery Evidence

Date: 2026-07-04

Scope: T074 fallback, hook-missing, config-corrupt, restart,
network-unavailable, wake-lock-unavailable, and screen-state-unavailable
validation.

Status: partial. T074 remains unchecked.

## Read-only Baseline

Command:

```text
rtk sh -lc 'sh -n scripts/validate-degraded-state.sh &&
  sh scripts/validate-degraded-state.sh 3B1F4LE5MS142WJY'
```

Result:

```text
sys.boot_completed: 1
root uid: 0
freezeit pid: 3501
daemon socket: LISTEN 127.0.0.1:60613
package inventory: pm list packages -U returned package/uid rows
policy file: policy-ready
cgroup freezer: cgroup-freezer-present
binder: binder-present
power dump: mWakefulness=Dozing
connectivity dump: NetworkProviders present
```

The helper was corrected during this validation to check
`/sys/fs/cgroup/apps` and `/sys/fs/cgroup/system` for `cgroup.freeze`, because
the COS16 app freezer paths are not rooted at `/sys/fs/cgroup/cgroup.freeze`.

## Runtime Diagnostic Baseline

Manager protocol after final T052 package install and empty config cleanup:

```text
health={"status":"active","daemonReady":true,"hookHealth":"active"}
self_check={"controlAllowed":true}
xp_log={"status":"active","system_server_ready":true,"config_ready":true,"screen_ready":true,"wakelock_ready":true,"network_ready":true}
GetAppCfgLen=0
operation={"operations":[]}
```

## Restart Validation

Command shape:

```text
old=$(adb -s 3B1F4LE5MS142WJY shell "su -c 'pidof freezeit'")
adb -s 3B1F4LE5MS142WJY shell \
  "su -c 'kill $old; sleep 1; nohup /data/adb/modules/freezeit/freezeit >/dev/null 2>&1 &'"
```

Observed:

```text
old_pid=3501
LISTEN 127.0.0.1:60613 users:(("freezeit",pid=20314,fd=3))
GetAppCfgLen=0
health={"status":"active","daemonReady":true,"hookHealth":"active"}
self_check={"controlAllowed":true}
xp_log={"status":"active","system_server_ready":true,"config_ready":true,"screen_ready":true,"wakelock_ready":true,"network_ready":true}
operation={"operations":[]}
```

Restart recovery passes for the no-controlled-app baseline: the daemon returns
to an active state, no stale app config is retained, and no spurious control
operation is logged.

## Related Evidence

Fallback and skip logging:

- `specs/002-modern-freezer-rewrite/evidence/us2-device-freeze.md` records the
  final cgroup backend logs and the earlier signal fallback diagnosis that led
  to app cgroup discovery.
- The final cleanup check recorded protected-policy `skip` entries with
  package identity, UID, action, result, backend, and reason.

Hook missing:

- `specs/002-modern-freezer-rewrite/evidence/us1-install-boot.md` records the
  LSPosed scope removal/restore validation. With system scope removed, daemon
  health reported degraded/hook missing and `RunSelfCheck` returned
  `controlAllowed:false`; after DB restore and reboot, active state returned.

Host-side degraded behavior:

- `specs/002-modern-freezer-rewrite/evidence/us4-implementation.md` records
  contract/integration coverage for operation log JSON, restart recovery,
  config recovery, and degraded reasons for package inventory, freezer,
  network, wake-lock, and screen-state failures.

## Open Gaps

T074 is not complete yet:

- Config-corrupt recovery has host coverage, but the current live daemon startup
  path does not expose a target-device diagnostic that proves corrupted
  `appcfg.txt` recovery on boot.
- Network-unavailable, wake-lock-unavailable, and screen-state-unavailable are
  modeled by host contracts, while the target device currently reports all
  three as ready. Deliberately breaking those system services on a daily-use
  phone was not performed.
- A complete T074 pass still needs target-observable diagnostics for these
  unavailable paths, or an owner-approved safe fault-injection method.

## Scoped Review And Convergence

Scoped Brooks review result for this evidence: no code-level finding is closed
by the partial validation. The main finding is a completion risk: T074 cannot be
marked complete from the current target evidence because several requested fault
classes remain host-only or unsafe to inject.

Scoped Speckit convergence result: not converged for T074. No task checkbox was
changed.
