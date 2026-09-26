import java.lang.reflect.Field;
import java.lang.reflect.Method;
import java.util.concurrent.atomic.AtomicInteger;

/**
 * 反射访问检查：同一程序内的用户类成员（private / package-private / protected，含嵌套类
 * nestmate）无需 setAccessible 即可 invoke / get / set（JDK 按调用方判定同类 / 同包 / nestmate
 * 可达）；JDK 类的非 public 成员无 setAccessible → IllegalAccessException，setAccessible 后仍
 * 不可达（java.base 封装，InaccessibleObjectException）的情形不在此测。
 */
public class TestReflectAccessCheck {
    private int secret = 7;
    int pkg = 8;
    protected int prot = 9;

    private String hidden(int x) { return "hidden" + x; }
    static class Inner {
        private static String nest() { return "nestmate"; }
    }

    public static void main(String[] args) throws Exception {
        TestReflectAccessCheck t = new TestReflectAccessCheck();
        Field fs = TestReflectAccessCheck.class.getDeclaredField("secret");
        System.out.println("private field get=" + fs.get(t));
        fs.set(t, 70);
        System.out.println("private field set=" + t.secret);
        System.out.println("pkg=" + TestReflectAccessCheck.class.getDeclaredField("pkg").get(t)
                + " prot=" + TestReflectAccessCheck.class.getDeclaredField("prot").get(t));
        Method h = TestReflectAccessCheck.class.getDeclaredMethod("hidden", int.class);
        System.out.println("private method=" + h.invoke(t, 3));
        Method n = Inner.class.getDeclaredMethod("nest");
        System.out.println("nestmate static=" + n.invoke(null));

        AtomicInteger ai = new AtomicInteger(5);
        Field v = AtomicInteger.class.getDeclaredField("value");
        try {
            System.out.println("jdk private=" + v.get(ai));
        } catch (IllegalAccessException e) {
            System.out.println("jdk private -> IllegalAccessException");
        }
    }
}
