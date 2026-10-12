package com.example.aop;

import org.aspectj.lang.JoinPoint;
import org.aspectj.lang.ProceedingJoinPoint;
import org.aspectj.lang.annotation.After;
import org.aspectj.lang.annotation.AfterReturning;
import org.aspectj.lang.annotation.AfterThrowing;
import org.aspectj.lang.annotation.Around;
import org.aspectj.lang.annotation.Aspect;
import org.aspectj.lang.annotation.Before;

/**
 * 日志切面：演示五种 Advice 与构造器拦截。
 * AspectJ 编译期织入（CTW）——由 ajc 在编译时把切面代码直接插入目标方法字节码中，
 * 运行时零反射、零代理。
 */
@Aspect
public class LogAspect {

    // ---------- 1. @Before：前置通知 ----------
    @Before("execution(* com.example.aop.UserService.addUser(..)) && args(name, ..)")
    public void logBefore(JoinPoint jp, String name) {
        System.out.println("[AOP-@Before] " + jp.getSignature().getName()
                + " 参数: " + name);
    }

    // ---------- 2. @AfterReturning：返回后通知（可拿到返回值） ----------
    @AfterReturning(
            pointcut = "execution(* com.example.aop.UserService.getUser(..))",
            returning = "result")
    public void logAfterReturning(JoinPoint jp, Object result) {
        System.out.println("[AOP-@AfterReturning] " + jp.getSignature().getName()
                + " 返回值: " + result);
    }

    // ---------- 3. @AfterThrowing：异常通知 ----------
    @AfterThrowing(
            pointcut = "execution(* com.example.aop.UserService.deleteUser(..))",
            throwing = "ex")
    public void logAfterThrowing(JoinPoint jp, Throwable ex) {
        System.out.println("[AOP-@AfterThrowing] " + jp.getSignature().getName()
                + " 异常: " + ex.getMessage());
    }

    // ---------- 4. @After：最终通知（类似 finally，无论是否异常） ----------
    @After("execution(* com.example.aop.UserService.updateUser(..))")
    public void logAfter(JoinPoint jp) {
        System.out.println("[AOP-@After] " + jp.getSignature().getName() + " 结束");
    }

    // ---------- 5. @Around：环绕通知（最强大，可控制目标是否执行） ----------
    @Around("execution(* com.example.aop.UserService.updateUser(..))")
    public Object logAround(ProceedingJoinPoint pjp) throws Throwable {
        long start = System.nanoTime();
        Object result = pjp.proceed(); // 执行目标方法
        long costMs = (System.nanoTime() - start) / 1_000_000;
        // 不打印具体毫秒数（每次运行不同），打印固定的档位结论
        System.out.println("[AOP-@Around] " + pjp.getSignature().getName()
                + (costMs < 10 ? " 执行耗时在 10ms 以内" : " 执行耗时超过 10ms"));
        return result;
    }

    // ---------- 特色：拦截构造方法（Spring AOP / JDK 代理做不到） ----------
    @Before("execution(com.example.aop.UserService.new(..))")
    public void logConstructor(JoinPoint jp) {
        System.out.println("[AOP-构造器拦截] " + jp.getSignature().getName());
    }
}
