
import java.lang.reflect.Method;

import org.apache.commons.lang3.reflect.FieldUtils;
import org.apache.commons.lang3.reflect.MethodUtils;

/**
 * commons-lang3 pilot A13：反射工具对固定类的调用（L3 反射分派的实测面，W1 #4）。
 */
public class Lang3ReflectMain {

    public static class Target {
        private String secret = "init";
        private int hits;

        public String greet(String who) {
            hits++;
            return "hello " + who;
        }

        private int secretCode() {
            return 42;
        }

        static String statik() {
            return "S";
        }
    }

    static void check(String name, Object expected, Object actual) {
        boolean ok = String.valueOf(expected).equals(String.valueOf(actual));
        System.out.println((ok ? "PASS " : "FAIL ") + name
                + (ok ? "" : " :: expected=<" + expected + "> actual=<" + actual + ">"));
    }

    public static void main(String[] args) throws Exception {
        Target target = new Target();

        // MethodUtils：公开/私有/静态，按名+参类型解析
        check("invoke-public", "hello java", MethodUtils.invokeMethod(target, "greet", "java"));
        check("invoke-private", 42, MethodUtils.invokeMethod(target, true, "secretCode"));
        check("invoke-static", "S", Target.class.getDeclaredMethod("statik").invoke(null));
        check("get-accessible", true,
                MethodUtils.getAccessibleMethod(Target.class, "greet", String.class) != null);

        // FieldUtils：读私有字段、写私有字段（字段子序号稳定）
        check("field-read", "init", FieldUtils.readField(target, "secret", true));
        FieldUtils.writeField(target, "secret", "changed", true);
        check("field-write", "changed", FieldUtils.readField(target, "secret", true));

        // 方法调用计数（验证调用真实生效）
        check("hits-after-greet", 1, FieldUtils.readField(target, "hits", true));

        // getMatchingAccessibleMethod：重载解析（greet(String) 唯一）
        Method m = MethodUtils.getMatchingAccessibleMethod(Target.class, "greet",
                new Class<?>[]{String.class});
        check("matching-method", "greet", m == null ? "null" : m.getName());

        // getAllFields 排序输出（FieldUtils 无排序保证）
        var names = new java.util.TreeSet<String>();
        for (var f : FieldUtils.getAllFields(Target.class)) {
            names.add(f.getName());
        }
        System.out.println("  fields sorted: " + names);
        System.out.println("DONE Lang3ReflectMain");
    }
}
