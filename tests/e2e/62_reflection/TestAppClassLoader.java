import java.io.IOException;
import java.net.URL;
import java.util.Collections;
import java.util.Enumeration;
import java.util.ServiceLoader;

/**
 * 内建加载器层级与系统类加载器（FS-C2）：类镜像的定义加载器（用户类 / 嵌套类 / 数组类 / 基本类型 /
 * 引导类 / 平台类）、getSystemClassLoader 与 getParent 链、Class.forName 按加载器查找、断言状态按
 * 加载器求值，以及 initPhase3 设定的主线程上下文类加载器（新线程继承、ServiceLoader 按其查找）。
 */
public class TestAppClassLoader {
    interface Service {}
    static class Nested {}

    static class Early {
        static boolean asserts() {
            try {
                assert false : "early";
                return false;
            } catch (AssertionError e) {
                return true;
            }
        }
    }

    static class Late {
        static boolean asserts() {
            try {
                assert false : "late";
                return false;
            } catch (AssertionError e) {
                return true;
            }
        }
    }

    /** 记录 ServiceLoader 经上下文加载器发起的资源查询 */
    static class Probe extends ClassLoader {
        Probe() { super("probe", ClassLoader.getSystemClassLoader()); }

        @Override
        public Enumeration<URL> getResources(String name) throws IOException {
            System.out.println("  probe.getResources " + name);
            return Collections.emptyEnumeration();
        }
    }

    static String name(ClassLoader l) {
        return l == null ? "null" : l.getName();
    }

    static String chain(ClassLoader l) {
        StringBuilder sb = new StringBuilder();
        for (; l != null; l = l.getParent()) sb.append(l.getName()).append(" -> ");
        return sb.append("null").toString();
    }

    static ClassLoader childContext() throws InterruptedException {
        ClassLoader[] seen = new ClassLoader[1];
        Thread t = new Thread(() -> seen[0] = Thread.currentThread().getContextClassLoader());
        t.start();
        t.join();
        return seen[0];
    }

    public static void main(String[] args) throws Exception {
        ClassLoader scl = ClassLoader.getSystemClassLoader();
        System.out.println("scl = " + name(scl));
        System.out.println("scl again same = " + (ClassLoader.getSystemClassLoader() == scl));
        System.out.println("chain = " + chain(scl));
        System.out.println("platform same = " + (ClassLoader.getPlatformClassLoader() == scl.getParent()));

        System.out.println("user == scl: " + (TestAppClassLoader.class.getClassLoader() == scl));
        System.out.println("nested == scl: " + (Nested.class.getClassLoader() == scl));
        System.out.println("Nested[][] == scl: " + (Nested[][].class.getClassLoader() == scl));
        System.out.println("int = " + name(int.class.getClassLoader()));
        System.out.println("void = " + name(void.class.getClassLoader()));
        System.out.println("int[] = " + name(int[].class.getClassLoader()));
        System.out.println("String = " + name(String.class.getClassLoader()));
        System.out.println("String[] = " + name(String[].class.getClassLoader()));
        System.out.println("java.sql.Time = " + name(java.sql.Time.class.getClassLoader()));
        System.out.println("java.sql.Time[] = " + name(java.sql.Time[].class.getClassLoader()));

        Class<?> found = Class.forName("TestAppClassLoader$Nested", false, scl);
        System.out.println("forName(app) same = " + (found == Nested.class));
        try {
            Class.forName("TestAppClassLoader$Missing", false, scl);
            System.out.println("forName(missing) found");
        } catch (ClassNotFoundException e) {
            System.out.println("forName(missing) CNFE " + e.getMessage());
        }

        System.out.println("assert(Early) desired = " + Early.class.desiredAssertionStatus());
        System.out.println("assert(Early) runs = " + Early.asserts());
        scl.setClassAssertionStatus("TestAppClassLoader$Late", true);
        // javac 让嵌套类按最外层类取断言状态：最外层类开启后，此后初始化的嵌套类执行 assert
        scl.setClassAssertionStatus("TestAppClassLoader", true);
        System.out.println("assert(Late) desired = " + Late.class.desiredAssertionStatus());
        System.out.println("assert(Late) runs = " + Late.asserts());
        System.out.println("assert(Early) after = " + Early.asserts());
        System.out.println("assert(String) = " + String.class.desiredAssertionStatus());
        scl.clearAssertionStatus();

        Thread main = Thread.currentThread();
        System.out.println("main ctx == scl: " + (main.getContextClassLoader() == scl));
        System.out.println("child ctx == scl: " + (childContext() == scl));

        Probe probe = new Probe();
        main.setContextClassLoader(probe);
        System.out.println("child ctx after set = " + name(childContext()));
        System.out.println("ServiceLoader via probe:");
        System.out.println("  hasNext = " + ServiceLoader.load(Service.class).iterator().hasNext());
        System.out.println("ServiceLoader explicit scl:");
        System.out.println("  hasNext = " + ServiceLoader.load(Service.class, scl).iterator().hasNext());

        main.setContextClassLoader(null);
        System.out.println("child ctx null = " + name(childContext()));
        System.out.println("ServiceLoader with null ctx:");
        System.out.println("  hasNext = " + ServiceLoader.load(Service.class).iterator().hasNext());

        main.setContextClassLoader(scl);
        System.out.println("main ctx restored = " + (main.getContextClassLoader() == scl));
    }
}
