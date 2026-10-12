
import java.lang.reflect.Proxy;

import org.aopalliance.intercept.MethodInterceptor;
import org.aopalliance.intercept.MethodInvocation;
import org.springframework.aop.framework.ProxyFactory;

/**
 * spring-aop pilot C2：ProxyFactory 的 JDK 接口代理面（切片 2/4）。
 * CGLIB 类代理不在本例（接口代理是 rava 已解锁的动态代理面）。
 */
public class SpringProxyJdkMain {

    public interface UserService {
        String greet(String who);

        default String farewell(String who) {
            return "bye " + who;
        }
    }

    public static class SimpleUserService implements UserService {
        @Override
        public String greet(String who) {
            return "hello " + who;
        }
    }

    static void check(String name, Object expected, Object actual) {
        boolean ok = String.valueOf(expected).equals(String.valueOf(actual));
        System.out.println((ok ? "PASS " : "FAIL ") + name
                + (ok ? "" : " :: expected=<" + expected + "> actual=<" + actual + ">"));
    }

    public static void main(String[] args) {
        ProxyFactory factory = new ProxyFactory(new SimpleUserService());
        var calls = new java.util.ArrayList<String>();
        factory.addAdvice((MethodInterceptor) invocation -> {
            calls.add(invocation.getMethod().getName()
                    + "(" + java.util.Arrays.toString(invocation.getArguments()) + ")");
            Object result = invocation.proceed();
            calls.add("-> " + result);
            return "[" + result + "]";
        });
        UserService proxy = (UserService) factory.getProxy();

        check("advised-greet", "[hello java]", proxy.greet("java"));
        check("advised-default-method", "[bye kotlin]", proxy.farewell("kotlin"));
        check("interceptor-calls",
                "[greet([java]), -> hello java, farewell([kotlin]), -> bye kotlin]",
                calls.toString());

        // 代理类型：JDK 动态代理（接口面）
        check("is-jdk-proxy", true, Proxy.isProxyClass(proxy.getClass()));
        check("target-type", SimpleUserService.class,
                org.springframework.aop.support.AopUtils.getTargetClass(proxy));
        check("is-aop-proxy", true, org.springframework.aop.support.AopUtils.isAopProxy(proxy));

        // InvocationHandler 侧：方法名直通
        var handler = Proxy.getInvocationHandler(proxy);
        check("handler-not-null", true, handler != null);

        // 多通知链：两个拦截器按序包裹
        ProxyFactory f2 = new ProxyFactory(new SimpleUserService());
        var order = new java.util.ArrayList<String>();
        f2.addAdvice((MethodInterceptor) inv -> {
            order.add("A-open");
            Object r = inv.proceed();
            order.add("A-close");
            return r;
        });
        f2.addAdvice((MethodInterceptor) inv -> {
            order.add("B-open");
            Object r = inv.proceed();
            order.add("B-close");
            return r;
        });
        UserService chained = (UserService) f2.getProxy();
        chained.greet("chain");
        check("chain-order", "[A-open, B-open, B-close, A-close]", order.toString());
        System.out.println("DONE SpringProxyJdkMain");
    }
}
