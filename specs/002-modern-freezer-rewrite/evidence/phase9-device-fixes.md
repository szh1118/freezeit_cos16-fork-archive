# Phase 9 Device Fix Evidence

Date: 2026-07-04

Scope: T092, T093, T094, T098, plus the owner-reported regressions for
manager log responsiveness, real-time graph/RAM data, battery power, temperature,
settings writes, Android/kernel version display, and UID CPU time.

## Root Causes Addressed

- Manager log and operation-log JSON timed out because `GetLog` and
  `GetOperationLogJson` synchronously ran a live control pass before replying.
  The daemon also held `state` and `control_state` locks across the control pass.
- Control passes scanned `/proc` once per controlled UID. On the target device,
  `/data/adb/modules/freezeit/appcfg.txt` contains 380 records, including 141
  control-mode records, so the old per-UID scan made manager reads wait 9-15 s.
- Real-time temperature returned 0 because the reader short-circuited on
  `thermal_zone0`, which is `pm8550-bcl-lvl0 temp=0` on this COS16 baseline.
- Hook config was loaded locally but needed synchronization to the hook after
  daemon startup before the control loop could safely run.

## Code Changes

- `freezeitDaemon/src/sys/socket.rs`
  - Manager requests no longer run live control passes before responding.
  - Manager request handling no longer locks `RuntimeControlState`.
  - Background control loop clones a read-only state snapshot and releases the
    manager state lock before running control work.
  - Control loop uses batched procfs discovery.
- `freezeitDaemon/src/sys/procfs.rs`
  - Added `discover_managed_uid_processes*` to scan `/proc` once for all managed
    UIDs in a pass.
- `freezeitDaemon/src/protocol/manager_v1.rs`
  - Real-time chart uses persistent `/proc/stat` delta history.
  - Temperature selection ignores zero/invalid zones and prefers CPU/cpuss
    thermal zones over BCL zones.
  - Battery power uses battery or USB readings so discharge/USB states do not
    collapse to 0 when one source reports 0.
- `freezeitDaemon/src/app/controller.rs`
  - Loaded manager config is synchronized to the hook at startup.
- `freezeitApp/app/src/main/java/io/github/jark006/freezeit/ManagerCmd.java`
  - v2 diagnostic command IDs match the Rust daemon and protocol contract.

## Host Verification

```text
rtk cargo test --manifest-path freezeitDaemon/Cargo.toml --test procfs_runtime_discovery discovers_multiple_managed_uid_processes_with_one_procfs_scan
result: pass
observed: 1 passed, 3 filtered
```

```text
rtk cargo test --manifest-path freezeitDaemon/Cargo.toml realtime_temperature_ignores_zero_bcl_and_prefers_cpu_zone
result: pass
observed: 1 passed, 78 filtered
```

```text
rtk sh freezeitDaemon/scripts/test-host.sh
result: pass
observed: 79 Rust unit/contract/integration tests passed, 0 failed
```

```text
rtk sh -lc '. "$HOME/.cargo/env" && sh freezeitDaemon/scripts/build-android.sh'
result: pass
observed: release daemon built for aarch64-linux-android
```

```text
rtk sh scripts/package-release.sh
result: pass
observed:
- release zip integrity: pass
- packaged release: `freezeitRelease/freezeit_3.2.0SelfUse.zip`
- final candidate sha256:
  a01bd6cde0de11cfe3a0e4daa69deeeb7c5004dec1546dd5cc4fbc60b0a7d73d
```

## Target Device Verification

Device: `3B1F4LE5MS142WJY` / OnePlus CPH2653 / Android 16 / SDK 36 /
kernel `6.6.89-android15-8-g096cdb6ecefc-ab14358676-4k`.

Install and boot:

```text
rtk adb -s 3B1F4LE5MS142WJY shell 'su -c "magisk --install-module /data/local/tmp/freezeit_3.2.0SelfUse.zip"'
result: pass
observed: Magisk module install completed and requested reboot

rtk adb -s 3B1F4LE5MS142WJY reboot
rtk sh -lc 'adb -s 3B1F4LE5MS142WJY wait-for-device; ... getprop sys.boot_completed ...'
result: pass
observed: boot_completed=1

post-unlock daemon state:
pid=3574
listener=127.0.0.1:60613
module daemon binary=/data/adb/modules/freezeit/freezeit, size=834456
```

Manager protocol results after final install:

```text
getPropInfo cmd=2: ok, elapsed=0.006 s
sample:
freezeit / 3.2.0SelfUse / Android 16 /
kernel 6.6.89-android15-8-g096cdb6ecefc-ab14358676-4k /
daemon active / hook active

getXpLog cmd=10: ok, elapsed=0.006 s
sample:
{"status":"active","system_server_ready":true,"config_ready":true,
 "screen_ready":true,"wakelock_ready":true,"network_ready":true}

getHealthReport cmd=71: ok, elapsed=0.007 s
sample: {"status":"active","daemonReady":true,"hookHealth":"active"}

getCompatibilityBaseline cmd=73: ok, elapsed=0.004 s
sample contains:
deviceModel=CPH2653, androidVersion=16, sdk=36,
rootReady=true, hookReady=true, freezerReady=true

runSelfCheck cmd=75: ok, elapsed=0.003 s
sample: {"controlAllowed":true}

old diagnostic cmd=70: no response, elapsed=0.006 s
observed: short_header:0
```

Log and operation diagnostics:

```text
getOperationLogJson cmd=74: ok, len=6637, elapsed=0.004 s
sample includes:
packageName="com.facebook.services", uid=10120, pidList=[12743],
action="freeze", backend="cgroup.freeze",
reason="cgroup and binder freezer available", result="success"

getLog cmd=4: ok, len=6031, elapsed=0.003 s
sample includes:
daemon active: apps=380 settings=256 android=16 kernel=...
hook config synced: managed_apps=380 settings=256
operationId=1 ... package=com.facebook.services uid=10120 ...
```

Settings writes:

```text
getSettings cmd=8: ok, len=256
setSettingsVar cmd=23:
- idx=10 value=1 -> success, elapsed=0.006 s
- idx=13 value=1 -> success, elapsed=0.005 s
- idx=16 value=0 -> success, elapsed=0.004 s
- idx=17 value=1 -> success, elapsed=0.004 s
- idx=19 value=1 -> success, elapsed=0.004 s
```

UID CPU time:

```text
getUidTime cmd=9: ok, len=324, elapsed=0.038 s
```

Real-time chart and metrics, using command 6 with request
`height=80,width=160,available=0`:

```text
call 0: ok, elapsed=0.092 s, image_sha16=4f6dd4de1aa8a596,
        cpu_summary=9, temp_mC=42300, power_mW=1105, mem_avail=5868
call 1: ok, elapsed=0.099 s, image_sha16=b04714aea415f9f3,
        cpu_summary=5, temp_mC=41500, power_mW=839, mem_avail=5868
call 2: ok, elapsed=0.101 s, image_sha16=b5d82d32e202c817,
        cpu_summary=7, temp_mC=41500, power_mW=1017, mem_avail=5862
call 3: ok, elapsed=0.109 s, image_sha16=286476c0c3e3b73c,
        cpu_summary=8, temp_mC=41500, power_mW=1088, mem_avail=5879
```

Interpretation: the real-time payload size matches legacy image plus 23 i32
metrics (`80*160*4 + 92 = 51292`), consecutive image hashes differ, RAM is
non-zero, CPU usage updates, temperature is non-zero, and power is non-zero.

## Scoped Brooks Review

Mode: PR Review

Scope: `freezeitDaemon/src/sys/socket.rs`,
`freezeitDaemon/src/sys/procfs.rs`,
`freezeitDaemon/src/protocol/manager_v1.rs`,
`freezeitDaemon/src/app/controller.rs`,
`freezeitDaemon/src/protocol/manager_v2.rs`,
`freezeitApp/app/src/main/java/io/github/jark006/freezeit/ManagerCmd.java`,
and changed contract/integration tests.

Health Score: 99/100.

Findings:

Suggestion - R3 Knowledge Duplication

Symptom: `procfs.rs` now has similar PID/status/cmdline-to-`RuntimeProcess`
construction in both the legacy single-UID discovery path and the new batched
managed-UID discovery path.

Source: The Pragmatic Programmer - DRY; Fowler - Duplicate Code.

Consequence: future changes to process identity or cgroup path construction must
remember both scan paths.

Remedy: extract a shared private helper for "read this proc entry into an
optional `RuntimeProcess`" when the next procfs change touches this area. This is
not a blocker for Phase 9 because both paths are covered by contract tests and
the batch path is required to resolve the device timeout.

No Critical or Warning findings.

## Scoped Speckit Convergence

Checked against FR-001, FR-009, FR-010, FR-011, FR-014, SC-005,
`manager-daemon-protocol.md`, T092, T093, T094, T098, and the constitution.

Result:

- T092 satisfied for the target device: `getRealTimeInfo` command 6 returns
  changing chart images, RAM data, CPU deltas, non-zero temperature, and non-zero
  power.
- T093 reopened after source-level legacy log review: the earlier `getLog`
  sample was responsive but showed Rust `operationId=...` text rather than the
  original C++ emoji manager log surface. See the correction section below.
- T094 satisfied for the target device: hook health, xp log, self-check, startup
  hook config sync, and control-loop gating agree on active/allowed state.
- T098 satisfied for the target device: v2 IDs 71-75 work as published; old 70
  returns no response.

Remaining open work is still tracked by existing unchecked tasks:

- T074/T095: degraded/fault validation for config corruption, network,
  wake-lock, and screen-state substitutes.
- T084/T096: 24-hour self-use soak.
- T086/T097: full quickstart validation after T074 and T084 close.
- T089: final aggregate Brooks review.
- T090: final aggregate Speckit convergence.
- T099: command-by-command C++ parity matrix.

## Correction: Legacy Emoji Log Surface

Date: 2026-07-04

The earlier T093 evidence in this file was too broad. The recorded `getLog`
sample showed `operationId=... action=...`, which is the Rust structured
operation-log text fallback, not the original C++ manager log surface.

Legacy source re-check:

- `freezeitVS/include/freezeit.hpp`: `Freezeit::log/logFmt` prepend
  `[HH:MM:SS]  ` and append text to the in-memory manager log buffer.
- `freezeitVS/include/server.hpp`: manager `getLog` command 4 returns
  `freezeit.getLogPtr()`/`getLoglen()`; `clearLog` returns a one-byte newline
  buffer; `getProcState` appends freezer process state into the same log buffer.
- `freezeitVS/include/freezer.hpp`: user-visible operation lines use
  emoji text such as `☀️解冻`, `❄️冻结`, `🧊冻结`, `😭关闭`, and delay/freezer
  state lines.

Host code changes:

- `freezeitDaemon/src/app/operation_log.rs`
  - `operation_to_legacy_text` now emits `[HH:MM:SS]  ` legacy-style emoji lines
    for freeze, unfreeze, terminate, postpone, fallback, skip, and recovery.
  - v2 JSON remains unchanged and continues to expose structured diagnostic
    fields.
- `freezeitDaemon/src/protocol/manager_v1.rs`
  - `clearLog` now returns `\n` and leaves `state.log` as `\n`, matching the C++
    buffer behavior.
  - `getProcState` no longer appends the English placeholder
    `process state: ...`; it appends a legacy-style Chinese process-state log
    block.

Host verification:

```text
rtk cargo test --manifest-path freezeitDaemon/Cargo.toml --test operation_log_json operation_log_legacy_text_formats -- --nocapture
result: pass
observed: 2 passed, 4 filtered
```

```text
rtk cargo test --manifest-path freezeitDaemon/Cargo.toml --test manager_health_v1 remaining_legacy_commands_return_compatibility_payloads -- --nocapture
result: pass
observed: 1 passed, 23 filtered
```

```text
rtk cargo test --manifest-path freezeitDaemon/Cargo.toml --test manager_health_v1 get_log_includes_original_emoji_operation_entries -- --nocapture
result: pass
observed: 1 passed, 23 filtered
```

```text
rtk cargo fmt --manifest-path freezeitDaemon/Cargo.toml --check
result: pass
```

```text
rtk sh freezeitDaemon/scripts/test-host.sh
result: pass
observed: 81 Rust unit/contract/integration tests passed, 0 failed
```

Build environment correction:

```text
rtk sh freezeitDaemon/scripts/build-android.sh
result before environment repair: fail
observed:
error[E0463]: can't find crate for `core`
note: the `aarch64-linux-android` target may not be installed
```

Root cause: non-login shell invocations picked `/usr/bin/cargo`, which did not
have the rustup-managed Android target. The rustup toolchain and NDK were
already present under:

- `/home/admin/.cargo/bin`
- `/home/admin/.rustup`
- `/home/admin/Android/Sdk/ndk/28.2.13676358`

Environment fix:

- `freezeitDaemon/scripts/build-android.sh` now prepends `~/.cargo/bin` to
  `PATH`.
- It initializes `ANDROID_HOME`, `ANDROID_SDK_ROOT`, and `ANDROID_NDK_HOME` from
  `~/Android/Sdk` when those variables are not already set.
- It ensures the rustup `aarch64-linux-android` target is installed before
  building.
- `cargo-ndk v4.1.2` was installed with the rustup cargo into
  `/home/admin/.cargo/bin`.

Post-fix Android build and package verification:

```text
rtk sh freezeitDaemon/scripts/build-android.sh
result: pass
observed:
Building arm64-v8a (aarch64-linux-android)
Finished release profile
```

```text
rtk sha256sum freezeitDaemon/target/aarch64-linux-android/release/freezeit
result: pass
observed:
0f737877f65c25d1a1c2cb0536209fc5c6f579f166de5734b724b4cff33ea0de
```

```text
rtk sh scripts/package-release.sh
result: pass
observed:
release zip integrity: pass
packaged release: `freezeitRelease/freezeit_3.2.0SelfUse.zip`
```

```text
rtk sha256sum freezeitRelease/freezeit_3.2.0SelfUse.zip
result: pass
observed:
f1001975615f422d19f703515ae46cd79d43acff795c41db2a7a7419d2faa8db
```

Binary text search confirms both the rebuilt daemon and packaged
`freezeitRustARM64` contain the new `❄️冻结`, `☀️解冻`, and `进程冻结状态`
strings, and no longer contain the old legacy-log `operationId=` or
`process state:` strings.

Therefore T093 is still not closed by this correction. It has a passing host fix
and a rebuilt Android package, but still needs target install/reboot and target
`cmd=4` validation showing emoji legacy log entries rather than
`operationId=...`.
