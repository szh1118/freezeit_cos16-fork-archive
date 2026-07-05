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
  `freezeitRelease/freezeit_oneplus13_android16_selfuse_v3.2.6SelfUse_302006.zip`
- Build path: `freezeitVS/build_pack_linux.sh`
- Validation path: `scripts/validate-release-zip.sh`

## Self-Use Threat Boundary

`3.2.6SelfUse` is a background runtime-control build for the verified CPH2653
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

1. [项目开源地址](https://github.com/jark006/freezeitVS)

1. [管理器开源地址](https://github.com/jark006/freezeitapp)

1. [模块包发布地址](https://github.com/jark006/freezeitRelease)

### 其他链接

[教程 Tutorials](https://jark006.github.io/FreezeitIntroduction/) |
[酷安 @JARK006](https://www.coolapk.com/u/1212220) |
[QQ频道 冻它模块](https://qun.qq.com/qqweb/qunpro/share?_wv=3&_wwv=128&appChannel=share&inviteCode=1W6opB7&appChannel=share&businessType=9&from=246610&biz=ka) |
[Telegram Group](https://t.me/+sjDX1oTk31ZmYjY1) |
[蓝奏云 密码: dy6i](https://jark006.lanzout.com/b017oz9if) 
