import java.io.IOException;
import java.lang.reflect.InvocationHandler;
import java.lang.reflect.Proxy;
import java.lang.reflect.UndeclaredThrowableException;
import java.util.ArrayList;
import java.util.Arrays;
import java.util.Comparator;
import java.util.List;

/**
 * FS-R R4a：动态代理（Proxy.newProxyInstance）。多接口、default 方法、基本类型实参 / 返回、
 * equals / hashCode / toString 转发、instanceof、isProxyClass / getInvocationHandler、
 * 异常透传（声明的 checked / unchecked）与 UndeclaredThrowableException、
 * 基本类型返回 null → NPE、错型返回 → CCE、代理作为 JDK 接口（Comparator）被库代码调用。
 */
public class TestDynamicProxy {
    interface Greeter {
        String greet(String name);
        default String twice(String n) { return greet(n) + greet(n); }
    }

    interface Counter {
        int add(int a, int b);
        long total();
        void reset();
        boolean flag(char c, double d);
    }

    interface Risky {
        void checked() throws IOException;
        void unchecked();
        void undeclared();
    }

    public static void main(String[] args) throws Exception {
        List<String> log = new ArrayList<>();
        InvocationHandler h = (proxy, m, a) -> {
            log.add(m.getDeclaringClass().getSimpleName() + "." + m.getName()
                    + (a == null ? "[]" : Arrays.toString(a)));
            switch (m.getName()) {
                case "greet": return "hi " + a[0] + ";";
                case "twice": return "twice(" + a[0] + ")";
                case "add": return (Integer) a[0] + (Integer) a[1];
                case "total": return 42L;
                case "reset": return null;
                case "flag": return ((Character) a[0]) == 'y' && ((Double) a[1]) > 1.5;
                case "hashCode": return 7;
                case "equals": return proxy == a[0];
                case "toString": return "ProxyObj";
                case "checked": throw new IOException("io");
                case "unchecked": throw new IllegalStateException("ise");
                case "undeclared": throw new Exception("checked-undeclared");
                default: return null;
            }
        };
        ClassLoader cl = TestDynamicProxy.class.getClassLoader();
        Object p = Proxy.newProxyInstance(cl, new Class<?>[] {Greeter.class, Counter.class, Risky.class}, h);

        Greeter g = (Greeter) p;
        System.out.println(g.greet("bob"));
        System.out.println(g.twice("al"));
        Counter c = (Counter) p;
        System.out.println(c.add(2, 3));
        System.out.println(c.total());
        c.reset();
        System.out.println(c.flag('y', 2.5) + " " + c.flag('n', 2.5));
        System.out.println(p.hashCode());
        System.out.println(p.equals(p) + " " + p.equals("x"));
        System.out.println(p.toString());
        System.out.println("instanceof Greeter=" + (p instanceof Greeter)
                + " Counter=" + (p instanceof Counter) + " Runnable=" + (p instanceof Runnable));
        System.out.println("isProxyClass=" + Proxy.isProxyClass(p.getClass())
                + " plain=" + Proxy.isProxyClass(String.class));
        System.out.println("handler same=" + (Proxy.getInvocationHandler(p) == h));

        Risky r = (Risky) p;
        try {
            r.checked();
        } catch (IOException e) {
            System.out.println("IOException " + e.getMessage());
        }
        try {
            r.unchecked();
        } catch (IllegalStateException e) {
            System.out.println("ISE " + e.getMessage());
        }
        try {
            r.undeclared();
        } catch (UndeclaredThrowableException e) {
            System.out.println("UTE " + e.getCause().getMessage());
        }

        Counter nul = (Counter) Proxy.newProxyInstance(cl, new Class<?>[] {Counter.class}, (px, m, a) -> null);
        try {
            nul.add(1, 2);
        } catch (NullPointerException e) {
            System.out.println("NPE for null primitive return");
        }
        nul.reset();
        System.out.println("void null ok");
        Counter wrong = (Counter) Proxy.newProxyInstance(cl, new Class<?>[] {Counter.class}, (px, m, a) -> "str");
        try {
            wrong.total();
        } catch (ClassCastException e) {
            System.out.println("CCE for wrong return type");
        }

        @SuppressWarnings("unchecked")
        Comparator<String> rev = (Comparator<String>) Proxy.newProxyInstance(cl, new Class<?>[] {Comparator.class},
                (px, m, a) -> m.getName().equals("compare") ? ((String) a[1]).compareTo((String) a[0]) : null);
        List<String> items = new ArrayList<>(List.of("b", "a", "c", "d"));
        items.sort(rev);
        System.out.println("sorted=" + items);

        for (String s : log) {
            System.out.println(s);
        }
    }
}
