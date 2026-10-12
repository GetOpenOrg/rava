package com.example.instrument;

import java.lang.reflect.Method;
import java.util.concurrent.TimeUnit;

import org.springframework.cglib.proxy.MethodInterceptor;
import org.springframework.cglib.proxy.MethodProxy;

/**
 * 性能监控拦截器（文章第四节）：
 * 只拦截标注 @Monitor 的方法，超过 100ms 打慢调用告警，否则打正常日志。
 */
public class PerformanceInterceptor implements MethodInterceptor {

    @Override
    public Object intercept(Object obj, Method method, Object[] args,
                            MethodProxy proxy) throws Throwable {
        if (!method.isAnnotationPresent(Monitor.class)) {
            return proxy.invokeSuper(obj, args); // 未标注：直接放行
        }

        String methodName = method.getDeclaringClass().getSimpleName()
                + "." + method.getName();
        long start = System.nanoTime();

        try {
            return proxy.invokeSuper(obj, args);
        } finally {
            long ms = TimeUnit.NANOSECONDS.toMillis(System.nanoTime() - start);
            // 不打印具体毫秒数（每次运行不同），打印固定的档位结论
            if (ms > 100) {
                System.out.println("【慢查询】" + methodName + " 耗时超过 100ms");
            } else {
                System.out.println("【正常】" + methodName + " 耗时在正常范围");
            }
        }
    }
}
