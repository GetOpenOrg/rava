import java.io.Serializable;
import java.lang.reflect.Modifier;
import java.util.Arrays;
import java.util.function.Function;
import java.util.function.Supplier;

// lambda 类身份边界：每个调用点一个隐藏类（超类 Object、直接超接口为函数式接口 + 标记接口 /
// Serializable、FINAL|SYNTHETIC、isHidden），同站点实例共享类、不同站点不同类。
// 只断言确定性性质，不打印 `$$Lambda/0x…` 名字。
public class TestLambdaHiddenClass {
    interface Marker {
        default String tag() {
            return "marker-default";
        }
    }

    static Runnable fromSite() {
        return () -> {};
    }

    static Supplier<String> capture(String s) {
        return () -> s + "!";
    }

    static void describe(String label, Object o) {
        Class<?> c = o.getClass();
        System.out.println("== " + label);
        System.out.println("super=" + c.getSuperclass().getName());
        System.out.println("interfaces=" + Arrays.toString(c.getInterfaces()));
        System.out.println("hidden=" + c.isHidden() + " synthetic=" + c.isSynthetic()
                + " interface=" + c.isInterface() + " array=" + c.isArray());
        int m = c.getModifiers();
        System.out.println("modifiers=0x" + Integer.toHexString(m)
                + " final=" + Modifier.isFinal(m) + " public=" + Modifier.isPublic(m));
        System.out.println("anonymous=" + c.isAnonymousClass() + " local=" + c.isLocalClass()
                + " member=" + c.isMemberClass());
        System.out.println("enclosing=" + c.getEnclosingClass() + " declaring=" + c.getDeclaringClass()
                + " canonical=" + c.getCanonicalName());
        System.out.println("nestHost=" + (c.getNestHost() == TestLambdaHiddenClass.class)
                + " package=[" + c.getPackageName() + "]");
        System.out.println("isInstance=" + c.isInstance(o) + " notObjectClass=" + (c != Object.class));
        try {
            Class.forName(c.getName());
            System.out.println("forName=found");
        } catch (ClassNotFoundException e) {
            System.out.println("forName=CNFE");
        }
    }

    public static void main(String[] args) {
        Runnable r1 = () -> {};
        Runnable r2 = () -> System.out.println("other");
        describe("runnable", r1);
        System.out.println("runnable!=Runnable: " + (r1.getClass() != Runnable.class));
        System.out.println("Runnable.isAssignableFrom: " + Runnable.class.isAssignableFrom(r1.getClass()));
        System.out.println("Object.isAssignableFrom: " + Object.class.isAssignableFrom(r1.getClass()));
        System.out.println("different sites differ: " + (r1.getClass() != r2.getClass()));

        // 同一调用点（循环 / 方法多次调用）共享隐藏类
        Class<?> first = null;
        boolean same = true;
        for (int i = 0; i < 3; i++) {
            Runnable r = () -> {};
            if (first == null) first = r.getClass();
            else same &= first == r.getClass();
        }
        System.out.println("loop site shares class: " + same);
        System.out.println("method site shares class: " + (fromSite().getClass() == fromSite().getClass()));
        System.out.println("method site vs loop site differ: " + (fromSite().getClass() != first));

        Supplier<String> a = capture("a");
        Supplier<String> b = capture("b");
        describe("capturing supplier", a);
        System.out.println("capture values: " + a.get() + " " + b.get());
        System.out.println("capture site shares class: " + (a.getClass() == b.getClass()));

        Function<String, Integer> len = String::length;
        describe("method ref", len);
        System.out.println("method ref apply: " + len.apply("hello"));
        System.out.println("Function.isInstance: " + Function.class.isInstance(len)
                + " Supplier.isInstance: " + Supplier.class.isInstance(len));

        Runnable marked = (Runnable & Marker) () -> {};
        describe("marker intersection", marked);
        System.out.println("Marker.isAssignableFrom: " + Marker.class.isAssignableFrom(marked.getClass()));
        System.out.println("Marker.isInstance: " + Marker.class.isInstance(marked)
                + " instanceof: " + (marked instanceof Marker));
        Marker mk = (Marker) marked;
        System.out.println("marker default: " + mk.tag());
        System.out.println("marker Object methods: " + mk.equals(marked) + " " + (mk.hashCode() == marked.hashCode())
                + " " + mk.toString().equals(marked.toString()) + " " + (mk.getClass() == marked.getClass()));
        System.out.println("plain lambda instanceof Marker: " + (r1 instanceof Marker));
        try {
            Marker bad = (Marker) (Object) r1;
            System.out.println("plain lambda cast to Marker: ok " + (bad != null));
        } catch (ClassCastException e) {
            System.out.println("plain lambda cast to Marker: CCE");
        }

        Runnable ser = (Runnable & Serializable) () -> {};
        describe("serializable intersection", ser);
        System.out.println("Serializable.isAssignableFrom: "
                + Serializable.class.isAssignableFrom(ser.getClass()));
        System.out.println("plain lambda Serializable.isAssignableFrom: "
                + Serializable.class.isAssignableFrom(r1.getClass()));
        System.out.println("Serializable instanceof: serializable=" + (ser instanceof Serializable)
                + " plain=" + (((Object) r1) instanceof Serializable)
                + " supplier=" + (((Object) a) instanceof Serializable));
        try {
            Serializable bad = (Serializable) (Object) r1;
            System.out.println("plain lambda cast to Serializable: ok " + (bad != null));
        } catch (ClassCastException e) {
            System.out.println("plain lambda cast to Serializable: CCE");
        }
    }
}
