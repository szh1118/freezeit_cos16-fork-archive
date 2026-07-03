package io.github.jark006.freezeit.hook;

import static io.github.jark006.freezeit.hook.XpUtils.log;

import io.github.jark006.freezeit.BuildConfig;
import io.github.jark006.freezeit.hook.android.AlarmHook;
import io.github.jark006.freezeit.hook.android.AnrHook;
import io.github.jark006.freezeit.hook.android.BroadCastHook;
import io.github.jark006.freezeit.hook.android.FreezeitService;
import io.github.jark006.freezeit.hook.android.WakeLockHook;
import io.github.jark006.freezeit.hook.app.PowerKeeper;

public final class FreezeitHookEntry {
    private FreezeitHookEntry() {
    }

    public static void handlePackage(String packageName, ClassLoader classLoader) {
        switch (packageName) {
            case Enum.Package.self:
                XpUtils.hookMethod("Freezeit[manager]:", classLoader,
                        XpUtils.returnConstant(true),
                        Enum.Class.self, Enum.Method.isXposedActive);
                return;
            case Enum.Package.android:
                hookAndroid(classLoader);
                return;
            case Enum.Package.powerkeeper:
                PowerKeeper.Hook(classLoader);
                return;
            default:
        }
    }

    public static void hookAndroid(ClassLoader classLoader) {
        log("Freezeit[Xposed]", BuildConfig.VERSION_NAME);

        Config config = new Config();

        new FreezeitService(config, classLoader);
        new AlarmHook(config, classLoader);
        new AnrHook(config, classLoader);
        new BroadCastHook(config, classLoader);
        new WakeLockHook(config, classLoader); //FreezeitService 的 handleWakeLock 暂时不用
    }
}
