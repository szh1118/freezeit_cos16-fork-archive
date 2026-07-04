# Magisk And LSPosed API Review

Captured: 2026-07-03

## Sources Checked

- Magisk module guide:
  `https://github.com/topjohnwu/Magisk/blob/master/docs/guides.md`
- LSPosed modern API guide:
  `https://github.com/LSPosed/LSPosed/wiki/Develop-Xposed-Modules-Using-Modern-Xposed-API`
- libxposed API README:
  `https://github.com/libxposed/api`
- GitHub API for `LSPosed/LSPosed`: `archived=true`,
  `pushed_at=2025-03-04T21:28:36Z`, default branch `master`.

## Magisk Module Format Check

| Requirement | Current package evidence | Result |
| --- | --- | --- |
| `module.prop` strict metadata | `id`, `name`, `version`, `versionCode`, `author`, `description`, and optional `updateJson` are present. | `PASS` |
| `service.sh` late-start script | Present; starts after storage/user-unlock checks and disable/remove checks. | `PASS` |
| `customize.sh` installer customization | Present; installs native binary and manager APK, preserving existing config. | `PASS` |
| `uninstall.sh` removal hook | Present. | `PASS` |
| No system overlay | Added `skip_mount`, because this module does not ship a `system/` overlay. | `PASS` |
| Recovery flash metadata | `META-INF/com/google/android/update-binary` and `updater-script` are packaged. | `PASS` |

## LSPosed Compatibility Check

| Area | Current evidence | Result |
| --- | --- | --- |
| Legacy Xposed entry | `assets/xposed_init` points to `io.github.jark006.freezeit.hook.Hook`; `Hook` implements `IXposedHookLoadPackage`. | `PASS` for legacy LSPosed/Xposed runtime. |
| Legacy scope metadata | `AndroidManifest.xml` declares `xposedmodule`, `xposeddescription`, `xposedminversion=82`, and `xposedscope`. | `PASS` for legacy LSPosed manager metadata. |
| Modern metadata | Added `META-INF/xposed/module.prop` with `minApiVersion=100`, `targetApiVersion=102`, `staticScope=true`. | `PASS` for modern metadata packaging. |
| Modern static scope | Added `META-INF/xposed/scope.list` with `android`, `io.github.jark006.freezeit`, and `com.miui.powerkeeper`, matching implemented hook targets. | `PASS` for metadata packaging. |
| Modern Java entry | Added `META-INF/xposed/java_init.list` pointing to `io.github.jark006.freezeit.hook.ModernHook`; `ModernHook` extends `io.github.libxposed.api.XposedModule`. | `PASS` for modern entry packaging/build. |
| Modern hook API usage | Added `io.github.libxposed:api:102.0.0` as `compileOnly`; added libxposed README R8 rules for annotations and `java_init.list`; `ModernHook` uses `onPackageReady` for app scopes and `onSystemServerStarting` for system server; `ModernXposedBackend` uses `XposedInterface.hook(...).intercept(...)`. Shared hook logic now calls a Freezeit `MethodHook` abstraction instead of directly depending on `XC_MethodHook`. | `PASS` for API 102 code adaptation and release build. |
| Legacy compatibility | Legacy entry remains in `assets/xposed_init`; legacy calls are isolated in `LegacyXposedBackend`. | `PASS` for retaining old runtime compatibility without using legacy APIs from the modern entry. |

## Decision

The produced APK now has both supported entry paths:

- Legacy LSPosed/Xposed: `assets/xposed_init` -> `Hook` ->
  `LegacyXposedBackend`.
- Modern LSPosed/libxposed API 102: `META-INF/xposed/java_init.list` ->
  `ModernHook` -> `ModernXposedBackend`.

The modern path no longer delegates into `de.robv.android.xposed` APIs. It uses
libxposed API 102 hooks and a local callback abstraction that is shared with the
legacy backend.

This is still not runtime-proven on the target phone because there is no
attached Magisk/LSPosed OnePlus 13 device in this environment. The local status
is: source adapted, release APK build passed, package inspection passed, and AVD
manager smoke test passed.
