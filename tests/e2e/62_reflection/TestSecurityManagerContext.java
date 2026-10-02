import java.lang.reflect.Method;
import java.util.Arrays;

/**
 * SecurityManager.getClassContext（JVM_GetClassContext）：自调用者起逐帧的声明类，
 * 跳过反射调用帧（Method.invoke 链）与 native 帧；lambda 代理类帧在输出侧过滤。
 */
public class TestSecurityManagerContext {
    @SuppressWarnings("removal")
    static class Probe extends SecurityManager {
        String context() {
            // 运行期生成的 lambda 代理类名含地址（不确定），过滤掉
            return Arrays.stream(getClassContext()).map(Class::getName)
                    .filter(n -> !n.contains("$$Lambda")).toList().toString();
        }
    }

    static class Outer {
        static String viaOuter(Probe p) { return Inner.viaInner(p); }
    }

    static class Inner {
        static String viaInner(Probe p) { return p.context(); }
    }

    public static String reflective(Probe p) { return p.context(); }

    public static void main(String[] args) throws Exception {
        Probe p = new Probe();
        System.out.println("direct = " + p.context());
        System.out.println("nested = " + Outer.viaOuter(p));
        Method m = TestSecurityManagerContext.class.getMethod("reflective", Probe.class);
        System.out.println("reflect = " + m.invoke(null, p));
        Runnable r = () -> System.out.println("lambda = " + p.context());
        r.run();
    }
}
