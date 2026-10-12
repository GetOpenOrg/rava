package com.example.aop;

import org.aspectj.lang.ProceedingJoinPoint;
import org.aspectj.lang.annotation.Around;
import org.aspectj.lang.annotation.Aspect;

/**
 * 性能监控切面（文章第九节“高阶实战”）：
 * 只拦截标注了 @Monitor 的方法，统计耗时，超过 100ms 打印慢调用告警。
 */
@Aspect
public class PerformanceAspect {

    @Around("@annotation(monitor)")
    public Object monitor(ProceedingJoinPoint pjp, Monitor monitor) throws Throwable {
        String className = pjp.getTarget().getClass().getSimpleName();
        String methodName = pjp.getSignature().getName();
        long start = System.nanoTime();

        try {
            return pjp.proceed();
        } finally {
            long costMs = (System.nanoTime() - start) / 1_000_000;
            String tag = monitor.value().isEmpty() ? "" : "[" + monitor.value() + "] ";
            // 不打印具体耗时数值（每次运行不同），打印固定的档位结论
            if (costMs > 100) { // 超过 100ms 视为慢方法
                System.out.println("[监控-慢调用] " + tag + className + "." + methodName
                        + " 耗时超过 100ms");
            } else {
                System.out.println("[监控-正常] " + tag + className + "." + methodName
                        + " 耗时在正常范围");
            }
        }
    }
}
