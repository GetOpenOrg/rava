import java.lang.annotation.Annotation;
import java.lang.annotation.Retention;
import java.lang.annotation.RetentionPolicy;
import java.lang.reflect.Method;

// __name_assignable 的两条查表路径回归（二分化改动的行为见证，class_impl.rs）：
//   1) 层次表行：子类 → 父类 / 类 → 接口 / 枚举 / 数组协变 / 数组 → Serializable；
//   2) 接口块无层次行的传递查找：用户注解类型 → Annotation（沿直接超接口表）；
//   3) 反射实参校验（native_invoke → __name_assignable）：子类实参传父类形参、
//      接口形参收实现类、不匹配时 IllegalArgumentException 的形态。
public class TestIfaceAssignWalk {
    interface A { }
    interface B extends A { }
    interface C extends B { }
    static class Base implements C { }
    static class Derived extends Base { }

    @Retention(RetentionPolicy.RUNTIME)
    @interface Mark { }

    static String takeBase(Base b) { return "base:" + b.getClass().getSimpleName(); }
    static String takeA(A a) { return "a:" + a.getClass().getSimpleName(); }

    public static void main(String[] args) throws Exception {
        // 1) isAssignableFrom：层次表面
        Class<?>[] pairs = {
            Base.class, Derived.class,          // 子类 → 父类
            C.class, Derived.class,             // 实现 → 深接口
            A.class, B.class,                   // 接口 → 超接口
            Object.class, Derived.class,
            int[].class, int[].class,
            Object[].class, String[].class,     // 数组协变
            java.io.Serializable.class, String[].class,
            Cloneable.class, int[].class,
        };
        StringBuilder sb = new StringBuilder();
        for (int i = 0; i + 1 < pairs.length; i += 2) {
            sb.append(pairs[i].isAssignableFrom(pairs[i + 1]) ? 'Y' : 'N');
        }
        System.out.println("assign " + sb);

        // 2) 接口块路径：注解类型 → Annotation（层次表无行，沿直接超接口表）
        System.out.println("anno " + Annotation.class.isAssignableFrom(Mark.class));
        System.out.println("annoRev " + Mark.class.isAssignableFrom(Annotation.class));

        // 3) 反射实参校验（引用形参走 __name_assignable）
        Method mb = TestIfaceAssignWalk.class.getDeclaredMethod("takeBase", Base.class);
        mb.setAccessible(true);
        System.out.println("invokeSub " + mb.invoke(null, new Derived()));
        Method ma = TestIfaceAssignWalk.class.getDeclaredMethod("takeA", A.class);
        ma.setAccessible(true);
        System.out.println("invokeIface " + ma.invoke(null, new Derived()));
        try {
            mb.invoke(null, "not-a-base");
            System.out.println("invokeMismatch no-throw");
        } catch (IllegalArgumentException e) {
            System.out.println("invokeMismatch IAE");
        } catch (Exception e) {
            System.out.println("invokeMismatch " + e.getClass().getSimpleName());
        }
    }
}
