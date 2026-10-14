
import java.lang.reflect.Method;

import org.springframework.util.ClassUtils;
import org.springframework.util.ReflectionUtils;

/**
 * spring-core pilot C1：ReflectionUtils / ClassUtils 工具面（切片 1/4）。
 */
public class SpringReflectUtilsMain {

    public static class Demo {
        private final String secret = "hidden";
        private int hits;

        public String serve(String in) {
            hits++;
            return "[" + in + "]";
        }

        private String reveal() {
            return secret;
        }

        public static String statik() {
            return "S";
        }
    }

    static void check(String name, Object expected, Object actual) {
        boolean ok = String.valueOf(expected).equals(String.valueOf(actual));
        System.out.println((ok ? "PASS " : "FAIL ") + name
                + (ok ? "" : " :: expected=<" + expected + "> actual=<" + actual + ">"));
    }

    public static void main(String[] args) {
        Demo demo = new Demo();

        // ReflectionUtils：方法解析与调用
        Method serve = ReflectionUtils.findMethod(Demo.class, "serve", String.class);
        check("find-method", "serve", serve == null ? null : serve.getName());
        check("invoke", "[x]", ReflectionUtils.invokeMethod(serve, demo, "x"));

        Method reveal = ReflectionUtils.findMethod(Demo.class, "reveal");
        ReflectionUtils.makeAccessible(reveal);
        check("invoke-private", "hidden", ReflectionUtils.invokeMethod(reveal, demo));

        // 字段读写
        var field = ReflectionUtils.findField(Demo.class, "secret");
        ReflectionUtils.makeAccessible(field);
        check("field-read", "hidden", ReflectionUtils.getField(field, demo));
        ReflectionUtils.setField(field, demo, "rewritten");
        check("field-write", "rewritten", ReflectionUtils.getField(field, demo));

        // 方法唯一性与直方
        var methods = new java.util.TreeSet<String>();
        ReflectionUtils.doWithMethods(Demo.class, m -> methods.add(m.getName()));
        check("do-with-methods", true, methods.containsAll(java.util.List.of(
                "serve", "reveal", "statik", "equals", "hashCode", "toString")));
        System.out.println("  declared+inherited count=" + methods.size());

        // ClassUtils
        check("short-name", "SpringReflectUtilsMain.Demo",
                ClassUtils.getShortName(Demo.class));
        check("qualified", Demo.class.getName(), ClassUtils.getQualifiedName(Demo.class));
        check("is-primitive-wrap", true,
                ClassUtils.isPrimitiveWrapper(Integer.class));
        check("resolve-primitive-name", int.class,
                ClassUtils.resolvePrimitiveClassName("int"));
        check("is-primitive-or-wrapper", true,
                ClassUtils.isPrimitiveOrWrapper(Integer.class));
        check("classpath-resource", true,
                Demo.class.getClassLoader() == null
                        ? ClassUtils.getDefaultClassLoader() != null
                        : ClassUtils.getDefaultClassLoader() != null);
        System.out.println("DONE SpringReflectUtilsMain");
    }
}
