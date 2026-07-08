# Freezeit self-use release workspace

This directory stores self-use Magisk release artifacts for the verified
CPH2653 Android 16 target.

The current `3.2.xSelfUse` zips package the legacy native daemon from
`freezeitVS/magisk/freezeitARM64` through `freezeitVS/build_pack_linux.sh`.
`freezeitDaemon/` remains a modern rewrite workspace and is not the default
phone payload unless a candidate zip explicitly contains `freezeitRustARM64`
and target-device validation says so.

Validate any candidate zip with `scripts/validate-release-zip.sh`.

## Current Self-Use Release Artifact

- Release candidate zip:
  `freezeitRelease/freezeit_oneplus13_android16_selfuse_v3.2.8SelfUse_302008.zip`
- Build path: `freezeitVS/build_pack_linux.sh`
- Validation path: `scripts/validate-release-zip.sh`

## Self-Use Threat Boundary

`3.2.8SelfUse` is a background runtime-control build for the verified CPH2653
Android 16 baseline. It can reduce selected third-party app background execution
after hook/root/freezer readiness, foreground eligibility, configured delay, and
idle checks pass.

It is not a malware scanner, sandbox, exploit mitigator, or root/system trust
boundary. It does not prevent behavior before freeze, foreground behavior while
the app is being used, or activity from privileged ROM/root/system components.

# ❌ 本项目已停止维护 ❌

---

# FreezeitRelease 冻它模块发布页

**[面具模块]** 实现部分墓碑机制，自动暂停后台进程的运行。

**[MagiskModule]** Implement a partial tombstone mechanism to automatically suspend background processes.

### 相关链接

1. [当前自用维护地址](https://github.com/szh1118/freezeit_cos16)

1. [管理器源码目录](https://github.com/szh1118/freezeit_cos16/tree/main/freezeitApp)

1. [模块包发布目录](https://github.com/szh1118/freezeit_cos16/tree/main/freezeitRelease)

### 其他链接

[发布说明](https://github.com/szh1118/freezeit_cos16/blob/main/README.md) |
[问题反馈](https://github.com/szh1118/freezeit_cos16/issues) |
[更新元数据](https://github.com/szh1118/freezeit_cos16/blob/main/freezeitRelease/update.json)
