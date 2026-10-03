import java.lang.invoke.MethodHandles;
import java.lang.reflect.Method;
import java.util.function.Function;
import java.util.function.Supplier;

// @CallerSensitive 方法经方法引用 / lambda 调用时的调用者类：方法引用的调用者是 lambda 隐藏类
// （宿主类的巢成员），lambda 体内调用的调用者是宿主类
class Outsider {
    private static String priv() { return "outsider"; }
}

public class TestCallerSensitiveMethodRef {
    interface Invoker { Object call(Object target, Object[] args) throws Exception; }
    interface Loader { Class<?> load(String name) throws Exception; }

    static class Other {
        static Supplier<MethodHandles.Lookup> ref() { return MethodHandles::lookup; }
        static Supplier<MethodHandles.Lookup> lam() { return () -> MethodHandles.lookup(); }
        private static String hidden() { return "hidden"; }
        static Function<Method, Object> invoker() {
            return mm -> { try { return mm.invoke(null); } catch (Exception e) { return e.getClass().getSimpleName(); } };
        }
    }

    private static String secret() { return "secret"; }

    static String name(Class<?> c) { return c.getName().replaceAll("\\$\\$Lambda.*", "\\$\\$Lambda"); }

    static String call(Invoker inv) {
        try { return String.valueOf(inv.call(null, new Object[0])); }
        catch (Exception e) { return e.getClass().getSimpleName() + ": " + String.valueOf(e.getMessage()).replaceAll("/0x[0-9a-f]+", "/0x?"); }
    }

    static Object callIt(Method m) { try { return m.invoke(null); } catch (Exception e) { return e.getClass().getSimpleName(); } }

    public static void main(String[] args) throws Exception {
        Supplier<MethodHandles.Lookup> r1 = MethodHandles::lookup;
        System.out.println("ref: " + name(r1.get().lookupClass()));
        Supplier<MethodHandles.Lookup> l1 = () -> MethodHandles.lookup();
        System.out.println("lambda: " + name(l1.get().lookupClass()));
        System.out.println("other ref: " + name(Other.ref().get().lookupClass()));
        System.out.println("other lambda: " + name(Other.lam().get().lookupClass()));

        Method m = TestCallerSensitiveMethodRef.class.getDeclaredMethod("secret");
        Method om = Other.class.getDeclaredMethod("hidden");
        Method out = Outsider.class.getDeclaredMethod("priv");
        Function<Object, Object> inv = o -> { try { return m.invoke(o); } catch (Exception e) { return e.getClass().getSimpleName(); } };
        System.out.println("invoke via lambda: " + inv.apply(null));
        Function<Method, Object> fromOther = Other.invoker();
        System.out.println("other invoker on own private: " + fromOther.apply(om));
        System.out.println("other invoker on host private: " + fromOther.apply(m));
        System.out.println("other invoker on outsider: " + fromOther.apply(out));
        Function<Method, Object> invRef = TestCallerSensitiveMethodRef::callIt;
        System.out.println("helper ref: " + invRef.apply(m));

        System.out.println("invoke ref on host private: " + call(m::invoke));
        System.out.println("invoke ref on nested private: " + call(om::invoke));
        System.out.println("invoke ref on outsider: " + call(out::invoke));

        Loader byRef = Class::forName;
        System.out.println("forName ref: " + byRef.load("TestCallerSensitiveMethodRef$Other").getSimpleName());
        Function<String, Class<?>> fn = s -> { try { return Class.forName(s); } catch (ClassNotFoundException e) { return null; } };
        System.out.println("forName lambda: " + fn.apply("TestCallerSensitiveMethodRef$Other").getSimpleName());
    }
}
