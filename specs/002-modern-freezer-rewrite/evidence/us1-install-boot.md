# US1 Install Boot Evidence

Date: 2026-07-04

Scope: T038 target-device install, reboot, unlock, manager readiness, degraded
hook state validation.

Status: complete for T038. Active install/reboot/manager readiness passes, and
deliberate missing-LSPosed-system-scope validation proves fail-closed degraded
behavior with restoration back to active.

## Device

- Serial: `3B1F4LE5MS142WJY`
- Model: `CPH2653`
- Android release: `16`
- SDK: `36`
- Fingerprint:
  `OnePlus/CPH2653EEA/OP5D55L1:16/BP2A.250605.015/V.R4T3.1338e95_e24685_de185d:user/release-keys`
- Root: `su -c id` returned `uid=0(root)` with context `u:r:magisk:s0`

## Install And Reboot

```text
rtk adb -s 3B1F4LE5MS142WJY push freezeitRelease/freezeit_us1_test.zip /sdcard/Download/freezeit_us1_test.zip
result: pass
```

```text
rtk adb -s 3B1F4LE5MS142WJY shell 'su -c "magisk --install-module /sdcard/Download/freezeit_us1_test.zip"'
result: pass
observed:
- Extracting module files
- 冻它APP 安装成功
- 安装完毕, 重启生效
- Done
```

```text
rtk adb -s 3B1F4LE5MS142WJY reboot
rtk adb -s 3B1F4LE5MS142WJY wait-for-device
result: pass
```

```text
sys.boot_completed polling
result: pass
observed: boot_completed=1
```

## Module State After Reboot

```text
/data/adb/modules/freezeit/freezeit
owner: root
mode: executable
size: 457752 bytes
```

```text
ss -ltnp | grep 60613
result: pass
observed: LISTEN 127.0.0.1:60613 users:(("freezeit",pid=3467,fd=3))
```

```text
ps -A | grep -i freezeit
result: pass
observed:
root 3467 ... S freezeit
u0_a570 ... S io.github.jark006.freezeit
```

Boot log:

```text
[2026-07-04 00:30:36] 开始运行服务脚本
[2026-07-04 00:30:55] 进入桌面, 10秒后将启动冻它
[2026-07-04 00:31:05] 启动冻它
[2026-07-04 00:31:05] WARNING ROM fingerprint mismatch; continuing startup
  baseline=OnePlus/CPH2649IN/OP5D55L1:16/BP2A.250605.015/V.R4T3.1338e95_e24685_de185d:user/release-keys
  device=OnePlus/CPH2653EEA/OP5D55L1:16/BP2A.250605.015/V.R4T3.1338e95_e24685_de185d:user/release-keys
```

## Validator

The install/boot helper was pushed to `/data/local/tmp` and run under root.

```text
freezeit install boot validation
timestamp=2026-07-04 00:32:35
boot_completed=1
module_dir=/data/adb/modules/freezeit
daemon_binary=executable
daemon_socket=reachable
boot_log=present
```

## Manager Readiness

The device was initially on keyguard after reboot. A non-credential wake/swipe
allowed the manager to open.

UI hierarchy evidence from `uiautomator dump`:

```text
package="io.github.jark006.freezeit"
statusText="Daemon degraded / Hook unknown"
module_env="Magisk"
module_ver="0.1.0-rust (1)"
manager_ver="3.1.0Alpha (301000)"
```

Interpretation: manager, daemon process, and daemon socket are reachable after
install/reboot/unlock. Hook health is shown as degraded/unknown, which blocks
active readiness and therefore avoids unsafe app-control claims.

## Missing LSPosed System Scope Validation

Timestamp: `2026-07-04 01:45 Asia/Hong_Kong`

The LSPosed config DB was backed up before fault injection:

```text
/data/adb/lspd/config_backup/freezeit_scope_20260704_014526/
- modules_config.db
- modules_config.db-shm
- modules_config.db-wal
```

Host inspection of `/data/adb/lspd/config/modules_config.db` showed the active
Freezeit scope rows:

```text
io.github.jark006.freezeit|system|0
io.github.jark006.freezeit|io.github.jark006.freezeit|0
```

Fault injection removed only the `system` row while preserving the manager
package row:

```text
delete from scope
where module_pkg_name = 'io.github.jark006.freezeit'
  and app_pkg_name = 'system'
  and user_id = 0;

remaining rows:
io.github.jark006.freezeit|io.github.jark006.freezeit|0
```

After reboot with the missing system scope:

```text
daemon_socket=ready
grep FreezeitXposedServer /proc/net/unix
result: no socket
```

Daemon socket evidence:

```text
manager GetXpLog over 127.0.0.1:60613
hook bridge missing

manager GetHealthReport over 127.0.0.1:60613
{"status":"degraded","daemonReady":true,"hookHealth":"missing"}

manager RunSelfCheck over 127.0.0.1:60613
{"controlAllowed":false}
```

Manager UI hierarchy evidence:

```text
statusText="Daemon active / Hook missing\n{\"status\":\"degraded\",\"daemonReady\":true,\"hookHealth\":\"missing\"}"
manager_ver="3.2.0SelfUse (302000)"
```

Interpretation: with required `system` scope missing, LSPosed does not create
`@FreezeitXposedServer`; the daemon reports degraded hook health and blocks
unsafe control.

## Scope Restore Validation

The exact DB backup from
`/data/adb/lspd/config_backup/freezeit_scope_20260704_014526/` was restored and
the device was rebooted.

Post-restore socket evidence:

```text
manager SetAppCfg over 127.0.0.1:60613
result: success

manager GetXpLog over 127.0.0.1:60613
{"status":"active","system_server_ready":true,"config_ready":true,"screen_ready":true,"wakelock_ready":true,"network_ready":true}

manager GetHealthReport over 127.0.0.1:60613
{"status":"active","daemonReady":true,"hookHealth":"active"}

manager RunSelfCheck over 127.0.0.1:60613
{"controlAllowed":true}
```

Post-restore manager UI hierarchy evidence:

```text
statusText="Daemon active / Hook active\n{\"status\":\"active\",\"daemonReady\":true,\"hookHealth\":\"active\"}"
manager_ver="3.2.0SelfUse (302000)"
```

## Active Readiness Refresh

Timestamp: `2026-07-04 01:40:55 Asia/Hong_Kong`

After fixing the Rust daemon to query the LSPosed bridge, forward legacy manager
`SetAppCfg` writes as hook `SET_CONFIG`, translate binary manager app-config
records to the hook text payload, refresh hook health per manager request, and
render the manager headline from live daemon/hook health fields:

```text
rtk sh scripts/validate-install-boot.sh 3B1F4LE5MS142WJY
result: pass
observed:
- boot_completed=1
- daemon_binary=executable
- daemon_socket=reachable
- boot_log=present
```

```text
manager SetAppCfg over 127.0.0.1:60613
result: success
```

```text
manager GetXpLog over 127.0.0.1:60613
result: pass
observed:
{"status":"active","system_server_ready":true,"config_ready":true,"screen_ready":true,"wakelock_ready":true,"network_ready":true}
```

```text
manager GetHealthReport over 127.0.0.1:60613
result: pass
observed:
{"status":"active","daemonReady":true,"hookHealth":"active"}
```

Final manager UI hierarchy evidence:

```text
statusText="Daemon active / Hook active\n{\"status\":\"active\",\"daemonReady\":true,\"hookHealth\":\"active\"}"
manager_ver="3.2.0SelfUse (302000)"
```

## Scoped Brooks Review

Mode: PR Review

Scope: T038 target-device evidence and daemon residency fix.

Finding summary: no code finding remains for the observed daemon exit and stale
hook-health bugs after:

- `freezeitDaemon/src/app/controller.rs` initializes daemon state from the
  LSPosed bridge.
- `freezeitDaemon/src/sys/xposed_bridge.rs` connects to
  `@FreezeitXposedServer`, queries hook health, and forwards config writes.
- `freezeitDaemon/src/protocol/manager_v1.rs` translates legacy manager
  app-config payloads for hook `SET_CONFIG`.
- `freezeitDaemon/src/sys/socket.rs` accepts requests continuously and refreshes
  hook health per request.
- `freezeitApp/app/src/main/java/io/github/jark006/freezeit/fragment/Home.java`
  renders the live daemon/hook health fields.

Verification:

```text
rtk sh -lc '. "$HOME/.cargo/env" && cargo fmt --manifest-path freezeitDaemon/Cargo.toml && sh freezeitDaemon/scripts/test-host.sh'
result: pass, 50 tests
```

```text
ANDROID_HOME=/home/admin/Android/Sdk JAVA_HOME=/usr/lib/jvm/java-17-openjdk rtk sh ./gradlew :app:compileDebugJavaWithJavac :app:assembleRelease
result: pass
```

## Scoped Speckit Convergence

Scope: T038 checked against US1 acceptance scenarios and task text.

Convergence result: converged for T038. Install, reboot, daemon socket, manager
launch, active hook readiness, active manager UI, deliberate missing
LSPosed-system-scope degradation, fail-closed `controlAllowed:false`, and
restoration to active state are evidenced.
