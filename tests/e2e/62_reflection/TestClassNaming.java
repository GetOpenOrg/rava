import java.util.function.Supplier;

/**
 * FS-R R1：Class 命名族与嵌套元数据（getName / getSimpleName / getPackageName / isMemberClass /
 * isLocalClass / isAnonymousClass / getDeclaringClass / getEnclosingClass / isRecord /
 * descriptorString / cast）——JDK 字节码经类级 native（InnerClasses / EnclosingMethod / Record）执行。
 */
public class TestClassNaming {
    static class Member {}
    class Inner {}
    record Point(int x, int y) {}
    interface Shape {}
    enum Color { RED }

    static void show(Class<?> c) {
        System.out.println(c.getName() + " | simple=" + c.getSimpleName()
                + " | pkg=" + c.getPackageName()
                + " | member=" + c.isMemberClass()
                + " local=" + c.isLocalClass()
                + " anon=" + c.isAnonymousClass()
                + " record=" + c.isRecord()
                + " | desc=" + c.descriptorString());
        Class<?> d = c.getDeclaringClass();
        Class<?> e = c.getEnclosingClass();
        System.out.println("  declaring=" + (d == null ? "null" : d.getName())
                + " enclosing=" + (e == null ? "null" : e.getName()));
    }

    static Supplier<Object> makeAnon() {
        return new Supplier<Object>() {
            public Object get() { return "anon"; }
        };
    }

    public static void main(String[] args) {
        class Local {}
        show(TestClassNaming.class);
        show(Member.class);
        show(Inner.class);
        show(Point.class);
        show(Shape.class);
        show(Color.class);
        show(Local.class);
        show(makeAnon().getClass());
        show(int.class);
        show(String[].class);
        show(int[][].class);
        show(Member[].class);
        show(String.class);
        show(java.util.Map.Entry.class);

        Object o = "text";
        System.out.println("cast=" + String.class.cast(o));
        try {
            Integer.class.cast(o);
        } catch (ClassCastException e) {
            System.out.println("CCE " + e.getMessage());
        }
        System.out.println("castNull=" + Integer.class.cast(null));
    }
}
