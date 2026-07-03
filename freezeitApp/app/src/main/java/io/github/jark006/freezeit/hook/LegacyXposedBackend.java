package io.github.jark006.freezeit.hook;

import java.lang.reflect.Constructor;
import java.lang.reflect.Method;

import de.robv.android.xposed.XC_MethodHook;
import de.robv.android.xposed.XposedBridge;
import de.robv.android.xposed.XposedHelpers;

public class LegacyXposedBackend implements XpUtils.HookBackend {
    @Override
    public boolean hookMethod(String TAG, ClassLoader classLoader, XpUtils.MethodHook callback,
                              String className, String methodName, Object... parameterTypes) {
        Class<?> clazz = XposedHelpers.findClassIfExists(className, classLoader);
        if (clazz == null) {
            XpUtils.log(TAG, "Cannot hookMethod: " + methodName + ", cannot find " + className);
            return false;
        }
        Method method = XposedHelpers.findMethodExactIfExists(clazz, methodName, parameterTypes);
        if (method == null) {
            XpUtils.log(TAG, "Cannot hookMethod: " + methodName);
            return false;
        }
        XposedBridge.hookMethod(method, adapt(callback));
        XpUtils.log(TAG, "Success hookMethod: " + methodName);
        return true;
    }

    @Override
    public void hookConstructor(String TAG, ClassLoader classLoader, XpUtils.MethodHook callback,
                                String className, Object... parameterTypes) {
        Class<?> clazz = XposedHelpers.findClassIfExists(className, classLoader);
        if (clazz == null) {
            XpUtils.log(TAG, "Cannot hookConstructor, cannot find " + className);
            return;
        }
        Constructor<?> constructor = XposedHelpers.findConstructorExact(clazz, parameterTypes);
        if (constructor == null) {
            XpUtils.log(TAG, "Cannot hookConstructor: " + className);
            return;
        }
        XposedBridge.hookMethod(constructor, adapt(callback));
        XpUtils.log(TAG, "Success hookConstructor: " + className);
    }

    private XC_MethodHook adapt(final XpUtils.MethodHook callback) {
        return new XC_MethodHook() {
            @Override
            protected void beforeHookedMethod(MethodHookParam param) throws Throwable {
                XpUtils.MethodHookParam freezeitParam =
                        new XpUtils.MethodHookParam(param.thisObject, param.args);
                callback.beforeHookedMethod(freezeitParam);
                if (freezeitParam.hasThrowable()) {
                    param.setThrowable(freezeitParam.getThrowable());
                } else if (freezeitParam.isReturnEarly()) {
                    param.setResult(freezeitParam.getResult());
                }
            }

            @Override
            protected void afterHookedMethod(MethodHookParam param) throws Throwable {
                XpUtils.MethodHookParam freezeitParam =
                        new XpUtils.MethodHookParam(param.thisObject, param.args);
                if (param.hasThrowable()) {
                    freezeitParam.setThrowable(param.getThrowable());
                } else {
                    freezeitParam.setProceedResult(param.getResult());
                }
                callback.afterHookedMethod(freezeitParam);
                if (freezeitParam.hasThrowable()) {
                    param.setThrowable(freezeitParam.getThrowable());
                } else if (freezeitParam.isReturnEarly()) {
                    param.setResult(freezeitParam.getResult());
                }
            }
        };
    }
}
