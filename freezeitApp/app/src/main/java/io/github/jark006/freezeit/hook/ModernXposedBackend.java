package io.github.jark006.freezeit.hook;

import android.util.Log;

import java.lang.reflect.Constructor;
import java.lang.reflect.Executable;
import java.lang.reflect.Method;

import io.github.libxposed.api.XposedInterface;

public class ModernXposedBackend implements XpUtils.HookBackend {
    private static final String LOG_TAG = "Freezeit";

    private final XposedInterface xposed;

    public ModernXposedBackend(XposedInterface xposed) {
        this.xposed = xposed;
    }

    @Override
    public boolean hookMethod(String TAG, ClassLoader classLoader, XpUtils.MethodHook callback,
                              String className, String methodName, Object... parameterTypes) {
        Class<?> clazz = XpUtils.findClassIfExists(className, classLoader);
        if (clazz == null) {
            XpUtils.log(TAG, "Cannot hookMethod: " + methodName + ", cannot find " + className);
            return false;
        }
        Method method = XpUtils.findMethodExactIfExists(clazz, classLoader, methodName, parameterTypes);
        if (method == null) {
            XpUtils.log(TAG, "Cannot hookMethod: " + methodName);
            return false;
        }
        hookExecutable(TAG, method, callback);
        XpUtils.log(TAG, "Success hookMethod: " + methodName);
        return true;
    }

    @Override
    public void hookConstructor(String TAG, ClassLoader classLoader, XpUtils.MethodHook callback,
                                String className, Object... parameterTypes) {
        Constructor<?> constructor = XpUtils.findConstructorExactIfExists(className, classLoader, parameterTypes);
        if (constructor == null) {
            XpUtils.log(TAG, "Cannot hookConstructor: " + className);
            return;
        }
        hookExecutable(TAG, constructor, callback);
        XpUtils.log(TAG, "Success hookConstructor: " + className);
    }

    private void hookExecutable(String TAG, Executable executable, XpUtils.MethodHook callback) {
        xposed.hook(executable)
                .setExceptionMode(XposedInterface.ExceptionMode.PROTECTIVE)
                .intercept(chain -> intercept(callback, chain));
    }

    private Object intercept(XpUtils.MethodHook callback, XposedInterface.Chain chain) throws Throwable {
        Object[] args = chain.getArgs().toArray(new Object[0]);
        XpUtils.MethodHookParam param = new XpUtils.MethodHookParam(chain.getThisObject(), args);

        callback.beforeHookedMethod(param);
        if (param.hasThrowable()) throw param.getThrowable();

        Object result = param.isReturnEarly() ? param.getResult() : chain.proceed(param.args);
        param.setProceedResult(result);

        callback.afterHookedMethod(param);
        if (param.hasThrowable()) throw param.getThrowable();
        return param.getResult();
    }

    public void logFramework(String message) {
        try {
            xposed.log(Log.INFO, LOG_TAG, message);
        } catch (Throwable ignored) {
        }
    }
}
