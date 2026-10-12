package com.example.cglib;

import java.lang.annotation.ElementType;
import java.lang.annotation.Retention;
import java.lang.annotation.RetentionPolicy;
import java.lang.annotation.Target;
import java.lang.reflect.Method;

import org.springframework.cglib.proxy.Enhancer;
import org.springframework.cglib.proxy.MethodInterceptor;
import org.springframework.cglib.proxy.MethodProxy;

/**
 * 实战：用 CGLib 实现自定义 @Log 注解（文章第八节）。
 * 只有标注 @Log 的方法才打印耗时档位，其余方法直接放行。
 *
 * 输出顺序固定说明：不打印具体纳秒耗时（每次运行不同），打印固定的档位结论。
 */
public class AnnotationLogDemo {

    /** 自定义注解。 */
    @Target(ElementType.METHOD)
    @Retention(RetentionPolicy.RUNTIME)
    public @interface Log {
    }

    /** 被 @Log 标注的方法才织入耗时统计。 */
    static class LogMethodInterceptor implements MethodInterceptor {
        @Override
        public Object intercept(Object obj, Method method, Object[] args,
                                MethodProxy proxy) throws Throwable {
            if (method.isAnnotationPresent(Log.class)) {
                long start = System.nanoTime();
                Object result = proxy.invokeSuper(obj, args);
                long costMs = (System.nanoTime() - start) / 1_000_000;
                System.out.println(method.getName()
                        + (costMs >= 100 ? " 耗时超过100ms" : " 耗时在100ms以内"));
                return result;
            }
            return proxy.invokeSuper(obj, args);
        }
    }

    static class PaymentService {
        @Log
        public String pay(String orderId) {
            try {
                Thread.sleep(150); // 模拟支付处理（>100ms，命中慢档位）
            } catch (InterruptedException e) {
                Thread.currentThread().interrupt();
            }
            return "PAY_OK_" + orderId;
        }

        /** 未标注 @Log，不打印耗时。 */
        public String query(String orderId) {
            return "QUERY_OK_" + orderId;
        }
    }

    public static void main(String[] args) {
        Enhancer enhancer = new Enhancer();
        enhancer.setSuperclass(PaymentService.class);
        enhancer.setCallback(new LogMethodInterceptor());

        PaymentService proxy = (PaymentService) enhancer.create();
        System.out.println(proxy.pay("ORDER_001"));
        System.out.println(proxy.query("ORDER_001"));
    }
}
