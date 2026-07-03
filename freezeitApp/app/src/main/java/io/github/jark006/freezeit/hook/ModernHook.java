package io.github.jark006.freezeit.hook;

import androidx.annotation.NonNull;

import io.github.libxposed.api.XposedModule;

public class ModernHook extends XposedModule {
    @Override
    public void onPackageReady(@NonNull PackageReadyParam param) {
        String packageName = param.getPackageName();
        if (!Enum.Package.self.equals(packageName) && !Enum.Package.powerkeeper.equals(packageName)) {
            return;
        }
        ModernXposedBackend backend = new ModernXposedBackend(this);
        XpUtils.setHookBackend(backend);
        backend.logFramework("Freezeit modern package hook: " + packageName);
        FreezeitHookEntry.handlePackage(packageName, param.getClassLoader());
    }

    @Override
    public void onSystemServerStarting(@NonNull SystemServerStartingParam param) {
        ModernXposedBackend backend = new ModernXposedBackend(this);
        XpUtils.setHookBackend(backend);
        backend.logFramework("Freezeit modern system_server hook");
        FreezeitHookEntry.hookAndroid(param.getClassLoader());
    }
}
