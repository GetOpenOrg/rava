package com.example.cglib;

import java.lang.reflect.Method;

import org.springframework.cglib.proxy.MethodInterceptor;
import org.springframework.cglib.proxy.MethodProxy;

/**
 * 方法拦截器：在目标方法前后插入日志。
 * MethodProxy 基于 FastClass 索引调用，避免反射开销。
 */
public class LogInterceptor implements MethodInterceptor {

    @Override
    public Object intercept(Object obj, Method method, Object[] args,
                            MethodProxy proxy) throws Throwable {
        System.out.println("【前置日志】调用" + method.getName());
        Object result = proxy.invokeSuper(obj, args); // 调用父类（目标类）方法
        System.out.println("【后置日志】调用完成");
        return result;
    }
}
