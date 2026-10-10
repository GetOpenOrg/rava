import java.lang.annotation.ElementType;
import java.lang.annotation.Retention;
import java.lang.annotation.RetentionPolicy;
import java.lang.annotation.Target;
import java.lang.reflect.Constructor;
import java.lang.reflect.Field;
import java.lang.reflect.InvocationTargetException;
import java.lang.reflect.Method;
import java.lang.reflect.Modifier;
import java.util.ArrayList;
import java.util.Comparator;
import java.util.LinkedHashMap;
import java.util.List;
import java.util.Map;

/**
 * 注解驱动反射入口（JUnit 式）：用户自定义注解标注方法与字段，一个与被测类无关的通用 runner 只经
 * 类镜像（经集合传递）→ getMethods / getDeclaredMethods / getDeclaredFields → 注解过滤（含元素值）→
 * Constructor.newInstance / Field.set / Method.invoke 发现并执行。被测方法在 main 中无任何直接调用，
 * 只能经反射入口到达；含继承的 @Case（getMethods 取父类公开方法）、子类覆盖、静态 @Once、
 * 期望异常元素（Class 值）、私有字段注入。方法次序按名排序（反射枚举序未规定）。
 */
public class TestAnnotationDrivenRunner {

    @Retention(RetentionPolicy.RUNTIME)
    @Target(ElementType.METHOD)
    @interface Case {
        Class<? extends Throwable> expected() default None.class;
        int weight() default 1;
    }

    @Retention(RetentionPolicy.RUNTIME)
    @Target(ElementType.METHOD)
    @interface Prepare {}

    @Retention(RetentionPolicy.RUNTIME)
    @Target(ElementType.METHOD)
    @interface Once {}

    @Retention(RetentionPolicy.RUNTIME)
    @Target(ElementType.FIELD)
    @interface Inject {
        String value();
    }

    static final class None extends Throwable {}

    static final List<String> LOG = new ArrayList<>();

    public static class BaseSuite {
        @Case(weight = 2)
        public void inherited() {
            LOG.add("base.inherited by " + getClass().getSimpleName());
        }

        @Case
        public void overridden() {
            LOG.add("base.overridden");
        }
    }

    public static class MathSuite extends BaseSuite {
        @Inject("calc")
        private String label;
        private int counter;

        @Once
        public static void boot() {
            LOG.add("once:boot");
        }

        @Prepare
        public void prepare() {
            counter = 10;
            LOG.add("prepare label=" + label);
        }

        @Case
        public void adds() {
            counter += 5;
            LOG.add("adds counter=" + counter);
        }

        @Case(expected = ArithmeticException.class, weight = 3)
        public void divides() {
            int zero = counter - counter;
            LOG.add("divides " + (counter / zero));
        }

        @Case(expected = IllegalStateException.class)
        public void wrongException() {
            throw new UnsupportedOperationException("boom");
        }

        @Override
        @Case
        public void overridden() {
            LOG.add("math.overridden counter=" + counter);
        }

        public void notACase() {
            LOG.add("never");
        }
    }

    public static class TextSuite {
        @Inject("txt")
        public String label;

        @Case
        public void upper() {
            LOG.add("upper " + label.toUpperCase());
        }

        @Case
        public void fails() {
            throw new IllegalArgumentException("bad " + label);
        }
    }

    /** 与被测类无关的通用 runner：只认注解。 */
    static final class Runner {
        final Map<String, Integer> score = new LinkedHashMap<>();

        void run(List<Class<?>> suites) throws Exception {
            for (Class<?> c : suites) {
                runSuite(c);
            }
        }

        private void runSuite(Class<?> c) throws Exception {
            String name = c.getSimpleName();
            for (Method m : sorted(c.getDeclaredMethods())) {
                if (m.isAnnotationPresent(Once.class) && Modifier.isStatic(m.getModifiers())) {
                    m.invoke(null);
                }
            }
            List<Method> cases = new ArrayList<>();
            Method prepare = null;
            for (Method m : sorted(c.getMethods())) {
                if (m.getAnnotation(Case.class) != null) {
                    cases.add(m);
                } else if (m.isAnnotationPresent(Prepare.class)) {
                    prepare = m;
                }
            }
            int total = 0;
            for (Method m : cases) {
                Constructor<?> ctor = c.getDeclaredConstructor();
                Object inst = ctor.newInstance();
                for (Field f : c.getDeclaredFields()) {
                    Inject inj = f.getAnnotation(Inject.class);
                    if (inj != null) {
                        f.setAccessible(true);
                        f.set(inst, inj.value() + "#" + m.getName());
                    }
                }
                if (prepare != null) {
                    prepare.invoke(inst);
                }
                Case meta = m.getAnnotation(Case.class);
                String verdict;
                try {
                    m.invoke(inst);
                    verdict = meta.expected() == None.class ? "ok" : "missing " + meta.expected().getSimpleName();
                } catch (InvocationTargetException e) {
                    Throwable t = e.getCause();
                    if (meta.expected().isInstance(t)) {
                        verdict = "ok(expected " + t.getClass().getSimpleName() + ")";
                    } else {
                        verdict = "fail " + t.getClass().getSimpleName() + ": " + t.getMessage();
                    }
                }
                if (verdict.startsWith("ok")) {
                    total += meta.weight();
                }
                LOG.add(name + "." + m.getName() + " -> " + verdict
                        + " [declared in " + m.getDeclaringClass().getSimpleName() + "]");
            }
            score.put(name, total);
        }

        private static List<Method> sorted(Method[] ms) {
            List<Method> out = new ArrayList<>(List.of(ms));
            out.sort(Comparator.comparing(Method::getName));
            return out;
        }
    }

    public static void main(String[] args) throws Exception {
        List<Class<?>> suites = new ArrayList<>();
        suites.add(MathSuite.class);
        suites.add(TextSuite.class);
        Runner r = new Runner();
        r.run(suites);
        for (String line : LOG) {
            System.out.println(line);
        }
        System.out.println("score=" + r.score);
    }
}
