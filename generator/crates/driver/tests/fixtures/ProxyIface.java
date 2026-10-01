import java.lang.reflect.Proxy;

// 运行期定义类（动态代理）实现的接口在闭包里没有静态实现类：代理对象经 VM 承载的类定义点（未建模来源）
// 进入值流，接口调用的属主须至少 L2，调用点不得判为接收者恒 null
public class ProxyIface {
    interface Greeter { String greet(String n); }

    public static void main(String[] args) {
        Greeter g = (Greeter) Proxy.newProxyInstance(ProxyIface.class.getClassLoader(),
                new Class<?>[] { Greeter.class }, (p, m, a) -> "hi " + a[0]);
        System.out.println(g.greet("bob"));
    }
}
