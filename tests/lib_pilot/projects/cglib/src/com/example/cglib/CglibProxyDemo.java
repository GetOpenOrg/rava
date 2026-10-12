package com.example.cglib;

import java.lang.reflect.InvocationHandler;
import java.lang.reflect.Method;
import java.lang.reflect.Proxy;

import org.springframework.cglib.core.DebuggingClassWriter;
import org.springframework.cglib.proxy.Enhancer;

/**
 * CGLib 第一个代理程序 + JDK 动态代理对比。
 * - CGLib：生成目标类子类，普通类即可代理
 * - JDK 代理：只能代理接口（Proxy.newProxyInstance + InvocationHandler）
 * 同时开启 DebuggingClassWriter，把生成的代理类 .class 落盘到 ./cglib 目录。
 */
public class CglibProxyDemo {

    /** 供 JDK 代理使用的接口。 */
    public interface UserInterface {
        void save();
    }

    /** 供 JDK 代理使用的实现。 */
    public static class UserImpl implements UserInterface {
        @Override
        public void save() {
            System.out.println("保存用户");
        }
    }

    public static void main(String[] args) {
        // 输出 CGLib 动态生成的代理类字节码（类名形如 OrderService$$EnhancerByCGLIB$$xxx）
        System.setProperty(DebuggingClassWriter.DEBUG_LOCATION_PROPERTY, "./cglib-dump");

        // ---------- 1. CGLib 代理普通类 ----------
        Enhancer enhancer = new Enhancer();
        enhancer.setSuperclass(OrderService.class);
        enhancer.setCallback(new LogInterceptor());

        OrderService proxy = (OrderService) enhancer.create();
        proxy.createOrder("ORDER_001");
        proxy.pay(); // final 方法：拦截器会被调用，但 invokeSuper 执行的是原逻辑

        System.out.println();

        // ---------- 2. JDK 动态代理（必须基于接口） ----------
        UserInterface jdkProxy = (UserInterface) Proxy.newProxyInstance(
                CglibProxyDemo.class.getClassLoader(),
                new Class[]{UserInterface.class},
                new InvocationHandler() {
                    private final UserImpl target = new UserImpl();

                    @Override
                    public Object invoke(Object proxy, Method method, Object[] mArgs)
                            throws Throwable {
                        System.out.println("【前置日志】调用" + method.getName());
                        Object result = method.invoke(target, mArgs);
                        System.out.println("【后置日志】调用完成");
                        return result;
                    }
                });
        jdkProxy.save();

        System.out.println();

        // ---------- 3. 代理类结构验证 ----------
        // 不打印生成的代理类名（其中的 $$ 十六进制后缀由 CGLib 生成，跨运行不保证一致）
        System.out.println("CGLib 代理类是 OrderService 的子类: " + (proxy instanceof OrderService));
        System.out.println("JDK  代理类是 UserInterface 的实现: " + (jdkProxy instanceof UserInterface));
    }
}
