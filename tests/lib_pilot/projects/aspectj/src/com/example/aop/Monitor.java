package com.example.aop;

import java.lang.annotation.ElementType;
import java.lang.annotation.Retention;
import java.lang.annotation.RetentionPolicy;
import java.lang.annotation.Target;

/**
 * 自定义监控注解：标注在方法上，由 PerformanceAspect 环绕织入统计耗时。
 */
@Target(ElementType.METHOD)
@Retention(RetentionPolicy.RUNTIME)
public @interface Monitor {

    /** 链路描述，用于日志输出。 */
    String value() default "";
}
