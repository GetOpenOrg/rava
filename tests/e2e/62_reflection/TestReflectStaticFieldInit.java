import java.lang.invoke.MethodHandle;
import java.lang.invoke.MethodHandles;
import java.lang.invoke.VarHandle;
import java.lang.reflect.Field;

// 边界用例：反射 / 方法句柄读静态字段只初始化字段的声明类，不初始化字段类型；基本类型类镜像
public class TestReflectStaticFieldInit {
    static class Payload {
        static { System.out.println("Payload init"); }
    }

    static class ByField {
        static { System.out.println("ByField init"); }
        static int count = 7;
        static Payload payload;
    }

    static class ByHandle {
        static { System.out.println("ByHandle init"); }
        static long total = 42L;
        static Payload payload;
    }

    static class ByVarHandle {
        static { System.out.println("ByVarHandle init"); }
        static double ratio = 1.5;
    }

    static class Untouched {
        static { System.out.println("Untouched init"); }
        static int n = 1;
    }

    public static void main(String[] args) throws Throwable {
        Field count = ByField.class.getDeclaredField("count");
        Field payload = ByField.class.getDeclaredField("payload");
        System.out.println("declaring " + count.getDeclaringClass().getSimpleName() + " type " + count.getType());
        System.out.println("payload type " + payload.getType().getSimpleName());
        System.out.println("count = " + count.getInt(null));
        System.out.println("payload = " + payload.get(null));

        Field total = ByHandle.class.getDeclaredField("total");
        MethodHandle g = MethodHandles.lookup().unreflectGetter(total);
        System.out.println("handle ready");
        System.out.println("total = " + (long) g.invokeExact());
        MethodHandle p = MethodHandles.lookup().findStaticGetter(ByHandle.class, "payload", Payload.class);
        System.out.println("payload via handle = " + (Payload) p.invokeExact());

        VarHandle v = MethodHandles.lookup().findStaticVarHandle(ByVarHandle.class, "ratio", double.class);
        System.out.println("varhandle ready");
        System.out.println("ratio = " + (double) v.get());

        Field n = Untouched.class.getDeclaredField("n");
        System.out.println("untouched type " + n.getType() + " primitive " + n.getType().isPrimitive());

        System.out.println("int.class == Integer.TYPE: " + (int.class == Integer.TYPE));
        System.out.println("long.class == total type: " + (long.class == total.getType()));
        System.out.println("void.class: " + void.class + " " + Void.TYPE.isPrimitive());
        System.out.println("double.class super: " + double.class.getSuperclass());
    }
}
