package io.github.jark006.freezeit.hook.app;

import android.content.Context;

import java.util.ArrayList;
import java.util.List;
import java.util.concurrent.Callable;

import io.github.jark006.freezeit.hook.Enum;
import io.github.jark006.freezeit.hook.XpUtils;

// ColorOS/Athena package-level background cleanup mitigation.
public class OplusAthena {
    private static final String TAG = "Freezeit[OplusAthena]:";

    public static void Hook(ClassLoader classLoader) {
        hookExternalClearStrategy(classLoader, Enum.Class.OplusForceStopStrategy);
        hookExternalClearStrategy(classLoader, Enum.Class.OplusKillPidStrategy);
        hookExternalClearStrategy(classLoader, Enum.Class.OplusKillUidStrategy);
        hookExternalClearStrategy(classLoader, Enum.Class.OplusForceStopOrKillStrategy);

        XpUtils.hookMethod(TAG, classLoader, blockVoid("force-stop b"),
                Enum.Class.OplusClearUtils, Enum.Method.oplusForceStop,
                Context.class, String.class, int.class, int.class, int.class, String.class, String.class);
        XpUtils.hookMethod(TAG, classLoader, blockVoid("force-stop c"),
                Enum.Class.OplusClearUtils, Enum.Method.oplusForceStopWithFlag,
                Context.class, String.class, int.class, int.class, int.class, String.class, String.class, boolean.class);
        XpUtils.hookMethod(TAG, classLoader, blockBoolean("kill d"),
                Enum.Class.OplusClearUtils, Enum.Method.oplusKillSimple,
                int.class, int.class, String.class, int.class, int.class, int.class, String.class, String.class);
        XpUtils.hookMethod(TAG, classLoader, blockBoolean("kill e"),
                Enum.Class.OplusClearUtils, Enum.Method.oplusKill,
                int.class, int.class, String.class, int.class, int.class, int.class,
                String.class, String.class, Callable.class, Callable.class);
        XpUtils.hookMethod(TAG, classLoader, blockVoid("clear action kill h"),
                Enum.Class.OplusClearActionBase, Enum.Method.oplusClearActionKill,
                int.class, int.class, String.class, int.class, int.class, int.class,
                String.class, String.class, String.class);

        XpUtils.hookMethod(TAG, classLoader, logOnly("GuardElf policy"),
                Enum.Class.OplusRemoteGuardElfServiceStub, Enum.Method.onPowerProtectPolicyChange,
                String.class, int.class);
        XpUtils.hookMethod(TAG, classLoader, logOnly("GuardElf switch"),
                Enum.Class.OplusRemoteGuardElfServiceStub, Enum.Method.setGuardElfSwitch,
                boolean.class, String.class);
    }

    private static void hookExternalClearStrategy(ClassLoader classLoader, String className) {
        XpUtils.hookMethod(TAG, classLoader, blockList("external clear " + simpleName(className)),
                className, Enum.Method.oplusExternalClear,
                List.class,
                Enum.Class.OplusExternalClearW1,
                Enum.Class.OplusExternalClearO0,
                Enum.Class.OplusClearRecord,
                Enum.Class.OplusKeepRecord);
    }

    private static XpUtils.MethodHook blockList(final String target) {
        return new XpUtils.MethodHook() {
            @Override
            protected void beforeHookedMethod(XpUtils.MethodHookParam param) {
                XpUtils.log(TAG, "Blocked " + target + " " + describeArgs(param.args));
                param.setResult(new ArrayList<>());
            }
        };
    }

    private static XpUtils.MethodHook blockVoid(final String target) {
        return new XpUtils.MethodHook() {
            @Override
            protected void beforeHookedMethod(XpUtils.MethodHookParam param) {
                XpUtils.log(TAG, "Blocked " + target + " " + describeArgs(param.args));
                param.setResult(null);
            }
        };
    }

    private static XpUtils.MethodHook blockBoolean(final String target) {
        return new XpUtils.MethodHook() {
            @Override
            protected void beforeHookedMethod(XpUtils.MethodHookParam param) {
                XpUtils.log(TAG, "Blocked " + target + " " + describeArgs(param.args));
                param.setResult(false);
            }
        };
    }

    private static XpUtils.MethodHook logOnly(final String target) {
        return new XpUtils.MethodHook() {
            @Override
            protected void beforeHookedMethod(XpUtils.MethodHookParam param) {
                XpUtils.log(TAG, target + " " + describeArgs(param.args));
            }
        };
    }

    private static String describeArgs(Object[] args) {
        if (args == null || args.length == 0) return "";
        StringBuilder builder = new StringBuilder("[");
        for (int i = 0; i < args.length; i++) {
            if (i > 0) builder.append(", ");
            builder.append(describe(args[i]));
        }
        return builder.append(']').toString();
    }

    private static String describe(Object arg) {
        if (arg == null) return "null";
        String value = String.valueOf(arg);
        if (value.length() > 96) {
            value = value.substring(0, 96) + "...";
        }
        return value;
    }

    private static String simpleName(String className) {
        int index = className.lastIndexOf('.');
        return index >= 0 ? className.substring(index + 1) : className;
    }
}
