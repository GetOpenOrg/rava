import java.lang.reflect.Field;
import java.lang.reflect.InvocationTargetException;
import java.lang.reflect.Method;

/**
 * FS-R R2a：Field.get / set / getInt / getLong 与 Method.invoke 回到 JDK 字节码
 * （ReflectionFactory → MethodHandleAccessorFactory → 字段 / 方法句柄）。
 */
public class TestReflectFieldMethod {
    static class Base {
        public int n = 3;
        private long big = 1L << 40;
        protected String label = "base";
        static int counter = 7;
        final int fixed = 11;

        public String describe(int k) { return label + ":" + (n * k); }
        public static int twice(int x) { return 2 * x; }
        private String secret() { return "hidden-" + n; }
        public void boom() { throw new IllegalStateException("kaboom"); }
    }

    static class Derived extends Base {
        @Override
        public String describe(int k) { return "derived/" + super.describe(k); }
    }

    public static void main(String[] args) throws Exception {
        Base b = new Base();
        Field n = Base.class.getDeclaredField("n");
        System.out.println("get n=" + n.get(b) + " getInt=" + n.getInt(b) + " getLong=" + n.getLong(b));
        n.set(b, 5);
        System.out.println("after set n=" + b.n);
        n.setInt(b, 6);
        System.out.println("after setInt n=" + b.n);

        Field big = Base.class.getDeclaredField("big");
        big.setAccessible(true);
        System.out.println("big=" + big.getLong(b));
        big.setLong(b, 42L);
        System.out.println("big after=" + big.get(b));

        Field label = Base.class.getDeclaredField("label");
        label.set(b, "changed");
        System.out.println("label=" + label.get(b) + " direct=" + b.label);

        Field counter = Base.class.getDeclaredField("counter");
        System.out.println("static counter=" + counter.getInt(null));
        counter.set(null, 9);
        System.out.println("static after=" + Base.counter);

        Field fixed = Base.class.getDeclaredField("fixed");
        try {
            fixed.set(b, 1);
        } catch (IllegalAccessException e) {
            System.out.println("final set IAE");
        }
        try {
            n.set(b, "notInt");
        } catch (IllegalArgumentException e) {
            System.out.println("type IAE");
        }
        try {
            n.get(null);
        } catch (NullPointerException e) {
            System.out.println("null receiver NPE");
        }

        Method twice = Base.class.getDeclaredMethod("twice", int.class);
        System.out.println("static invoke=" + twice.invoke(null, 21));
        Method describe = Base.class.getDeclaredMethod("describe", int.class);
        System.out.println("instance invoke=" + describe.invoke(b, 2));
        System.out.println("virtual invoke=" + describe.invoke(new Derived(), 2));
        Method secret = Base.class.getDeclaredMethod("secret");
        secret.setAccessible(true);
        System.out.println("private invoke=" + secret.invoke(b));
        Method boom = Base.class.getDeclaredMethod("boom");
        try {
            boom.invoke(b);
        } catch (InvocationTargetException e) {
            System.out.println("ITE cause=" + e.getCause());
        }
        try {
            describe.invoke(b, "x");
        } catch (IllegalArgumentException e) {
            System.out.println("arg IAE");
        }
    }
}
