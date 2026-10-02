import java.lang.reflect.AccessFlag;
import java.util.Arrays;

/**
 * Class 上的 VM native：getDeclaredClasses0 / getNestMembers0 / getClassAccessFlagsRaw0 /
 * getSigners / setSigners，以及 ClassLoader.retrieveDirectives（断言状态初始化）。
 */
public class TestClassNestNatives {
    public static class Inner {
        class Deep {}
    }
    private interface Hidden {}
    enum Color { RED }
    @interface Marker {}
    static abstract class Shape {}

    static String names(Class<?>[] cs) {
        String[] out = new String[cs.length];
        for (int i = 0; i < cs.length; i++) out[i] = cs[i].getName();
        Arrays.sort(out);
        return Arrays.toString(out);
    }

    static class Loader extends ClassLoader {
        Loader() { super(TestClassNestNatives.class.getClassLoader()); }
        void sign(Class<?> c, Object[] s) { setSigners(c, s); }
    }

    public static void main(String[] args) {
        System.out.println("declared(Test) = " + names(TestClassNestNatives.class.getDeclaredClasses()));
        System.out.println("declared(Inner) = " + names(Inner.class.getDeclaredClasses()));
        System.out.println("declared(Deep) = " + names(Inner.Deep.class.getDeclaredClasses()));
        System.out.println("declared(int[]) = " + names(int[].class.getDeclaredClasses()));
        System.out.println("declared(long) = " + names(long.class.getDeclaredClasses()));

        Class<?>[] nest = Inner.Deep.class.getNestMembers();
        System.out.println("nest host first = " + nest[0].getName());
        System.out.println("nest(Deep) = " + names(nest));
        System.out.println("nest(String[]) = " + names(String[].class.getNestMembers()));
        System.out.println("nest(char) = " + names(char.class.getNestMembers()));
        System.out.println("nestmate(Hidden, Color) = " + Hidden.class.isNestmateOf(Color.class));
        System.out.println("nestmate(Inner, String) = " + Inner.class.isNestmateOf(String.class));

        Class<?>[] flagged = {TestClassNestNatives.class, String.class, Runnable.class, Override.class,
                int.class, int[].class, Inner.class, Hidden.class, Color.class, Marker.class, Shape.class};
        for (Class<?> c : flagged) {
            System.out.println("flags " + c.getName() + " = " + c.accessFlags());
        }

        Loader loader = new Loader();
        System.out.println("signers before = " + Arrays.toString(Shape.class.getSigners()));
        loader.sign(Shape.class, new Object[] {"alice", 42});
        Object[] got = Shape.class.getSigners();
        System.out.println("signers after = " + Arrays.toString(got));
        got[0] = "mutated";
        System.out.println("signers copy = " + Arrays.toString(Shape.class.getSigners()));
        loader.sign(int.class, new Object[] {"ignored"});
        System.out.println("signers(int) = " + Arrays.toString(int.class.getSigners()));

        // 应用类加载器首次设置断言状态时经 retrieveDirectives 取 VM 指令（无 -ea：全空、缺省关）
        ClassLoader app = TestClassNestNatives.class.getClassLoader();
        System.out.println("assert(Inner) before = " + Inner.class.desiredAssertionStatus());
        app.setPackageAssertionStatus("java.util", true);
        System.out.println("assert(Inner) after maps = " + Inner.class.desiredAssertionStatus());
        app.setClassAssertionStatus(Shape.class.getName(), true);
        System.out.println("assert(Shape) class on = " + Shape.class.desiredAssertionStatus());
        app.setDefaultAssertionStatus(true);
        System.out.println("assert(Inner) default on = " + Inner.class.desiredAssertionStatus());
        app.clearAssertionStatus();
        System.out.println("assert(Shape) cleared = " + Shape.class.desiredAssertionStatus());
        System.out.println("assert(String) = " + String.class.desiredAssertionStatus());
    }
}
