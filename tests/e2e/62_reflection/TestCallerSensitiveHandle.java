import java.lang.invoke.MethodHandle;
import java.lang.invoke.MethodHandles;
import java.lang.invoke.MethodType;
import java.lang.reflect.Field;
import java.lang.reflect.Method;

// @CallerSensitive 方法经方法句柄 / 反射调用时的调用者类：
// - 有 @CallerSensitiveAdapter 的（MethodHandles.lookup、Method.invoke、Class.forName）以 lookup 类为调用者；
// - 无适配器的（Field.get）经注入调用器隐藏类 Host$$InjectedInvoker/0x…（宿主的巢成员）调用；
// - 受限 lookup（publicLookup）查找 CS 方法抛 IllegalAccessException。
class Outsider {
    private static String priv = "outsider";
    private static String priv() { return "outsider()"; }
}

public class TestCallerSensitiveHandle {
    private static String secret = "secret";
    private static String secret() { return "secret()"; }

    static class Nested {
        private static String inner = "inner";
    }

    static class Other {
        static MethodHandle lookupHandle() throws Exception {
            return MethodHandles.lookup().findStatic(MethodHandles.class, "lookup", MethodType.methodType(MethodHandles.Lookup.class));
        }
        static MethodHandle fieldGetter() throws Exception {
            return MethodHandles.lookup().findVirtual(Field.class, "get", MethodType.methodType(Object.class, Object.class));
        }
    }

    static String norm(String s) { return s.replaceAll("/0x[0-9a-f]+", "/0x?"); }

    static String describe(Throwable t) {
        return t.getClass().getSimpleName() + ": " + norm(String.valueOf(t.getMessage()));
    }

    static String get(MethodHandle getter, Field f) {
        try { return String.valueOf(getter.invoke(f, (Object) null)); }
        catch (Throwable t) { return describe(t); }
    }

    static String reflectGet(Method get, Field f) {
        try { return String.valueOf(get.invoke(f, (Object) null)); }
        catch (java.lang.reflect.InvocationTargetException e) { return describe(e.getCause()); }
        catch (Throwable t) { return describe(t); }
    }

    public static void main(String[] args) throws Throwable {
        MethodHandles.Lookup self = MethodHandles.lookup();

        // 适配器路由：MethodHandles.lookup(Class)
        MethodHandle lk = self.findStatic(MethodHandles.class, "lookup", MethodType.methodType(MethodHandles.Lookup.class));
        System.out.println("mh lookup: " + ((MethodHandles.Lookup) lk.invokeExact()).lookupClass().getName());
        System.out.println("other mh lookup: " + ((MethodHandles.Lookup) Other.lookupHandle().invokeExact()).lookupClass().getName());

        // 适配器路由：Method.invoke(Object, Object[], Class)
        Method sm = TestCallerSensitiveHandle.class.getDeclaredMethod("secret");
        Method om = Outsider.class.getDeclaredMethod("priv");
        MethodHandle inv = self.findVirtual(Method.class, "invoke", MethodType.methodType(Object.class, Object.class, Object[].class));
        System.out.println("mh invoke own private: " + inv.invoke(sm, null, new Object[0]));
        try {
            System.out.println("mh invoke outsider: " + inv.invoke(om, null, new Object[0]));
        } catch (Throwable t) { System.out.println("mh invoke outsider: " + describe(t)); }

        // 适配器路由：Class.forName(String, Class)
        MethodHandle fn = self.findStatic(Class.class, "forName", MethodType.methodType(Class.class, String.class));
        System.out.println("mh forName: " + ((Class<?>) fn.invoke("TestCallerSensitiveHandle$Nested")).getName());

        // 注入调用器：Field.get 无适配器
        Field own = TestCallerSensitiveHandle.class.getDeclaredField("secret");
        Field nested = Nested.class.getDeclaredField("inner");
        Field outsider = Outsider.class.getDeclaredField("priv");
        MethodHandle getter = self.findVirtual(Field.class, "get", MethodType.methodType(Object.class, Object.class));
        System.out.println("mh get own private: " + get(getter, own));
        System.out.println("mh get nested private: " + get(getter, nested));
        System.out.println("mh get outsider: " + get(getter, outsider));
        System.out.println("other mh get host private: " + get(Other.fieldGetter(), own));
        System.out.println("other mh get outsider: " + get(Other.fieldGetter(), outsider));
        // 同一宿主复用同一注入类
        System.out.println("mh get outsider again: " + get(getter, outsider));

        // 反射调用无适配器的 CS 方法：Method.invoke(Field.get) 经 reflect_invoke_V
        Method fget = Field.class.getMethod("get", Object.class);
        System.out.println("reflect get own private: " + reflectGet(fget, own));
        System.out.println("reflect get outsider: " + reflectGet(fget, outsider));

        // 受限 lookup 查找 CS 方法
        try {
            MethodHandles.publicLookup().findVirtual(Field.class, "get", MethodType.methodType(Object.class, Object.class));
            System.out.println("public lookup Field.get: found");
        } catch (IllegalAccessException e) { System.out.println("public lookup Field.get: " + describe(e)); }
        try {
            MethodHandles.publicLookup().findStatic(MethodHandles.class, "lookup", MethodType.methodType(MethodHandles.Lookup.class));
            System.out.println("public lookup lookup(): found");
        } catch (IllegalAccessException e) { System.out.println("public lookup lookup(): " + describe(e)); }
        // 非 CS 方法不受影响
        MethodHandle len = MethodHandles.publicLookup().findVirtual(String.class, "length", MethodType.methodType(int.class));
        System.out.println("public lookup length: " + (int) len.invokeExact("abc"));
    }
}
