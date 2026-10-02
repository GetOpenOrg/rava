import java.lang.invoke.MethodHandle;
import java.lang.invoke.MethodHandles;
import java.lang.invoke.MethodType;
import java.lang.invoke.VarHandle;

/**
 * @CallerSensitive 调用者类（Reflection.getCallerClass）：MethodHandles.lookup() 的 lookupClass 是调用
 * lookup() 的那个调用点所在类——静态方法、嵌套类实例方法、接口 default 方法、静态初始化块内各不相同；
 * 经其他类转调时取直接调用点所在类。lookup 的类决定后续 findStaticVarHandle / findStatic 的访问检查与目标
 * 类初始化（VarHandle 静态字段首次访问、静态方法句柄首次调用触发 <clinit>）。
 */
public class TestCallerSensitiveLookup {
    static int counter = 7;

    interface Probe {
        default String where() {
            return MethodHandles.lookup().lookupClass().getName();
        }
    }

    static class Nested implements Probe {
        String mine() {
            return MethodHandles.lookup().lookupClass().getName();
        }
    }

    static class Relay {
        static MethodHandles.Lookup grab() {
            return MethodHandles.lookup();
        }
    }

    static class Holder {
        static long total;
        static final VarHandle TOTAL;
        static {
            System.out.println("Holder.<clinit>");
            try {
                TOTAL = MethodHandles.lookup().findStaticVarHandle(Holder.class, "total", long.class);
            } catch (ReflectiveOperationException e) {
                throw new ExceptionInInitializerError(e);
            }
            total = 40;
        }
        static long bump(long d) {
            return (long) TOTAL.getAndAdd(d) + d;
        }
    }

    static class Lazy {
        static {
            System.out.println("Lazy.<clinit>");
        }
        private static String hidden(int x) {
            return "hidden " + x;
        }
        static MethodHandle self() throws ReflectiveOperationException {
            return MethodHandles.lookup().findStatic(Lazy.class, "hidden",
                    MethodType.methodType(String.class, int.class));
        }
    }

    public static void main(String[] args) throws Throwable {
        System.out.println("main: " + MethodHandles.lookup().lookupClass().getName());
        Nested n = new Nested();
        System.out.println("nested: " + n.mine());
        System.out.println("default: " + n.where());
        System.out.println("relay: " + Relay.grab().lookupClass().getName());

        VarHandle c = MethodHandles.lookup().findStaticVarHandle(TestCallerSensitiveLookup.class, "counter", int.class);
        System.out.println("counter getAndAdd: " + (int) c.getAndAdd(3) + " -> " + counter);

        System.out.println("before Holder");
        System.out.println("bump: " + Holder.bump(2));
        System.out.println("bump: " + Holder.bump(5));

        System.out.println("before Lazy");
        MethodHandle h = Lazy.self();
        System.out.println("handle ready");
        System.out.println((String) h.invokeExact(42));

        try {
            MethodHandles.lookup().findStatic(Lazy.class, "hidden", MethodType.methodType(String.class, int.class));
            System.out.println("main saw private: allowed (nestmate)");
        } catch (IllegalAccessException e) {
            System.out.println("main saw private: IllegalAccessException");
        }
    }
}
