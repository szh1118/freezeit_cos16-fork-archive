# freezeit_cos16

Freezeit / 冻它 的 COS16 自用适配版。

本仓库由 @szh1118 继续维护，基于原作者 JARK006 的 Freezeit 项目改版，保留原项目的 Magisk 模块、管理器 APK、native 服务和 LSPosed/Xposed hook 结构。

## 适配环境

- 设备：OnePlus 13
- 实测机型：CPH2653 / CPH2653EEA
- 系统：ColorOS 16 / Android 16
- SDK：API 36
- ROM 基线：BP2A.250605.015 / V.R4T3.1338e95_e24685_de185d
- Root：Magisk / Zygisk 环境
- Xposed：LSPosed IT v2.1.0-it，Modern Xposed API 102
- 模块版本：3.1.0Alpha / versionCode 301000

## 本版改动

- 适配 Android 16 / COS16 上的 system scope：LSPosed 作用域使用 `system`，同时包含管理器本包。
- 保留 legacy Xposed 入口，并新增 libxposed API 102 modern entry/backend。
- 增加 Android 16 / OnePlus 13 自用 ROM 基线记录，ROM 差异只作为 warning 记录，不阻止启动。
- 增加 hook readiness gate，避免 Xposed 侧尚未就绪时过早执行冻结控制。
- 增加 Linux ARM64 native 构建脚本和 Magisk 打包脚本。

## 目录

- `freezeitVS/`：native 服务、Magisk 模块脚本、打包脚本。
- `freezeitApp/`：管理器 APK 和 Xposed hook 代码。
- `freezeitRelease/`：当前打包好的 Magisk zip。

## 安装

1. 在 Magisk 中刷入 `freezeitRelease/freezeit_oneplus13_android16_selfuse_v3.1.0Alpha_301000.zip`。
2. 在 LSPosed 中启用冻它模块。
3. 作用域至少选择：
   - 系统框架 / `system`
   - 冻它管理器 / `io.github.jark006.freezeit`
4. 重启手机。

## 构建

管理器 APK：

```bash
cd freezeitApp
ANDROID_HOME=/home/admin/Android/Sdk \
ANDROID_SDK_ROOT=/home/admin/Android/Sdk \
JAVA_HOME=/usr/lib/jvm/java-17-openjdk \
PATH=/usr/lib/jvm/java-17-openjdk/bin:$PATH \
bash ./gradlew assembleRelease
```

Magisk 模块：

```bash
bash freezeitVS/freezeitVS/build_pack_linux.sh
```

## 注意

这是 @szh1118 针对自己 COS16 / OnePlus 13 环境维护的适配版，不承诺兼容其他 ROM、机型或 Android 版本。

原项目：

- https://github.com/jark006/freezeitVS
- https://github.com/jark006/freezeitApp
- https://github.com/jark006/freezeitRelease
