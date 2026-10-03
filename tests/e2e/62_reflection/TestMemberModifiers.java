import java.lang.reflect.Field;
import java.lang.reflect.Method;
import java.lang.reflect.Modifier;

/**
 * 成员级修饰符位与泛型签名（方法级实测：isPrivate 11 jar / isProtected 11 /
 * isTransient 9 / isAccessible 9 / Field.getGenericType 13 / toGenericString 6 /
 * getExceptionTypes 6，类级修饰符已有他例——本例补成员面）。
 */
public class TestMemberModifiers {

    static class Sample {
        private transient String secret;
        protected volatile int counter;
        public static final java.util.List<String> NAMES = java.util.Arrays.asList("a");

        public <T extends Exception> void risky(T input) throws Exception {
        }

        private void hidden() {
        }
    }

    public static void main(String[] args) throws Exception {
        Field secret = Sample.class.getDeclaredField("secret");
        int sm = secret.getModifiers();
        System.out.println("secret: private=" + Modifier.isPrivate(sm)
                + " transient=" + Modifier.isTransient(sm));

        Field counter = Sample.class.getDeclaredField("counter");
        int cm = counter.getModifiers();
        System.out.println("counter: protected=" + Modifier.isProtected(cm)
                + " volatile=" + Modifier.isVolatile(cm));

        Field names = Sample.class.getDeclaredField("NAMES");
        System.out.println("names: public=" + Modifier.isPublic(names.getModifiers())
                + " static=" + Modifier.isStatic(names.getModifiers())
                + " final=" + Modifier.isFinal(names.getModifiers()));
        System.out.println("generic-type=" + names.getGenericType().getTypeName());
        System.out.println("to-generic-string=" + names.toGenericString().contains("NAMES"));

        Method risky = Sample.class.getMethod("risky", Exception.class);
        System.out.println("risky-generic=" + risky.toGenericString().contains("<T"));
        Class<?>[] exs = risky.getExceptionTypes();
        System.out.println("throws-count=" + exs.length + " first=" + exs[0].getSimpleName());
        System.out.println("type-params=" + risky.getTypeParameters()[0].getName());

        // isAccessible / trySetAccessible 通道（私有方法）
        Method hidden = Sample.class.getDeclaredMethod("hidden");
        System.out.println("accessible-default=" + hidden.isAccessible());
        hidden.setAccessible(true);
        System.out.println("accessible-after=" + hidden.isAccessible()
                + " try=" + hidden.trySetAccessible());
        System.out.println("invoke-null=" + (hidden.invoke(new Sample()) == null));
    }
}
