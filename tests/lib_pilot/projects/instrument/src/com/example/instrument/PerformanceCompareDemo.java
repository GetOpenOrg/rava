package com.example.instrument;

import java.lang.reflect.Method;

import org.springframework.cglib.proxy.Enhancer;
import org.springframework.cglib.proxy.MethodInterceptor;
import org.springframework.cglib.proxy.MethodProxy;

/**
 * 性能对比（文章第七节）：直接调用 vs CGLib 代理 vs 反射，各 100 万次。
 * 典型量级：直接调用 << CGLib 代理 < 反射（MethodProxy 基于 FastClass 索引，避免反射）。
 */
public class PerformanceCompareDemo {

    interface Calc {
        int compute(int x);
    }

    static class CalcImpl implements Calc {
        @Override
        public int compute(int x) {
            return x + 1;
        }
    }

    private static final int TIMES = 1_000_000;

    public static void main(String[] args) throws Exception {
        Calc direct = new CalcImpl();

        // CGLib 代理（预先创建好，衡量的是调用开销而非生成开销）
        Enhancer enhancer = new Enhancer();
        enhancer.setSuperclass(CalcImpl.class);
        enhancer.setCallback((MethodInterceptor) (obj, method, mArgs, proxy) ->
                proxy.invokeSuper(obj, mArgs));
        Calc cglibProxy = (Calc) enhancer.create();

        // 反射 Method（预先获取）
        Method method = CalcImpl.class.getMethod("compute", int.class);

        int sink = 0;

        for (int i = 0; i < TIMES; i++) {
            sink += direct.compute(i);
        }
        for (int i = 0; i < TIMES; i++) {
            sink += cglibProxy.compute(i);
        }
        for (int i = 0; i < TIMES; i++) {
            sink += (int) method.invoke(direct, i);
        }

        System.out.println("三类调用各完成 " + TIMES + " 次");
        System.out.println("量级结论（文章典型数据）：直接调用 << CGLib 代理 < 反射调用");
        System.out.println("(防JIT消除校验: " + (sink == Integer.MIN_VALUE) + ")");
    }
}
