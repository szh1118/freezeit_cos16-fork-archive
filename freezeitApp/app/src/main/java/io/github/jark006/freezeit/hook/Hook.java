package io.github.jark006.freezeit.hook;

import de.robv.android.xposed.IXposedHookLoadPackage;
import de.robv.android.xposed.callbacks.XC_LoadPackage.LoadPackageParam;

public class Hook implements IXposedHookLoadPackage {

    @Override
    public void handleLoadPackage(LoadPackageParam lpParam) {
        XpUtils.setHookBackend(new LegacyXposedBackend());
        FreezeitHookEntry.handlePackage(lpParam.packageName, lpParam.classLoader);
    }
}
