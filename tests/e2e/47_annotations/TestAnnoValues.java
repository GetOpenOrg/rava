import java.lang.annotation.ElementType;
import java.lang.annotation.Inherited;
import java.lang.annotation.Retention;
import java.lang.annotation.RetentionPolicy;
import java.lang.annotation.Target;
import java.lang.reflect.Method;
import java.util.Arrays;

/**
 * FS-R R4b：注解回到 JDK 字节码（原始注解字节 + 稀疏常量池 → AnnotationParser → 动态代理）。
 * 覆盖：枚举 / 数组 / Class / 嵌套注解 / 全部基本类型元素、默认值、toString / equals /
 * hashCode / annotationType、@Inherited 继承、getAnnotations 顺序、参数注解。
 */
public class TestAnnoValues {
    enum Color { RED, GREEN, BLUE }

    @Retention(RetentionPolicy.RUNTIME)
    @interface Inner {
        String tag() default "in";
    }

    @Retention(RetentionPolicy.RUNTIME)
    @Target({ElementType.TYPE, ElementType.METHOD})
    @Inherited
    @interface Rich {
        int i() default 1;
        long l() default 2L;
        double d() default 1.5;
        float f() default 2.5f;
        char c() default 'x';
        byte b() default 7;
        short s() default 9;
        boolean z() default true;
        String str() default "def";
        Color color() default Color.GREEN;
        Class<?> type() default Object.class;
        int[] ints() default {};
        String[] names() default {"a", "b"};
        Color[] colors() default {};
        Inner inner() default @Inner;
    }

    @Retention(RetentionPolicy.RUNTIME)
    @interface Mark {
    }

    @Retention(RetentionPolicy.RUNTIME)
    @Target(ElementType.PARAMETER)
    @interface Param {
        String value();
    }

    @Rich(i = 42, l = 1L << 40, d = -0.25, c = 'q', str = "hello", color = Color.BLUE,
          type = String.class, ints = {3, 1, 2}, names = {"x"}, colors = {Color.RED, Color.BLUE},
          inner = @Inner(tag = "deep"))
    @Mark
    static class Base {
        @Rich
        public void plain(@Param("first") int a, int b, @Param("third") String c) {}
    }

    static class Child extends Base {
    }

    public static void main(String[] args) throws Exception {
        Rich r = Base.class.getAnnotation(Rich.class);
        System.out.println("i=" + r.i() + " l=" + r.l() + " d=" + r.d() + " f=" + r.f());
        System.out.println("c=" + r.c() + " b=" + r.b() + " s=" + r.s() + " z=" + r.z());
        System.out.println("str=" + r.str() + " color=" + r.color() + " type=" + r.type().getSimpleName());
        System.out.println("ints=" + Arrays.toString(r.ints()) + " names=" + Arrays.toString(r.names())
                + " colors=" + Arrays.toString(r.colors()));
        System.out.println("inner=" + r.inner().tag() + " annotationType=" + r.annotationType().getSimpleName());
        System.out.println("toString=" + r);

        Method m = Base.class.getMethod("plain", int.class, int.class, String.class);
        Rich dr = m.getAnnotation(Rich.class);
        System.out.println("defaults: i=" + dr.i() + " str=" + dr.str() + " color=" + dr.color()
                + " type=" + dr.type().getSimpleName() + " names=" + Arrays.toString(dr.names())
                + " inner=" + dr.inner().tag());
        System.out.println("default toString=" + dr);

        Rich again = Base.class.getAnnotation(Rich.class);
        System.out.println("equals self=" + r.equals(again) + " equals other=" + r.equals(dr)
                + " hash stable=" + (r.hashCode() == again.hashCode()));

        System.out.println("child inherited Rich=" + (Child.class.getAnnotation(Rich.class) != null)
                + " child Mark=" + (Child.class.getAnnotation(Mark.class) != null)
                + " declared on child=" + Child.class.getDeclaredAnnotations().length);
        System.out.println("base annotations=" + Base.class.getAnnotations().length
                + " isPresent Mark=" + Base.class.isAnnotationPresent(Mark.class));

        java.lang.annotation.Annotation[][] pa = m.getParameterAnnotations();
        for (int i = 0; i < pa.length; i++) {
            StringBuilder sb = new StringBuilder("param" + i + ":");
            for (java.lang.annotation.Annotation a : pa[i]) {
                sb.append(' ').append(((Param) a).value());
            }
            System.out.println(sb);
        }
    }
}
