import java.lang.annotation.Annotation;
import java.lang.annotation.ElementType;
import java.lang.annotation.Inherited;
import java.lang.annotation.Repeatable;
import java.lang.annotation.Retention;
import java.lang.annotation.RetentionPolicy;
import java.lang.annotation.Target;
import java.lang.reflect.AnnotatedElement;

/**
 * 注解深水：默认值 getDefaultValue、@Inherited 继承可见性、可重复注解的容器形态、
 * 注解实例 toString/equals 与缓存同一性（注解是代理对象——框架打印与比对注解的物性）。
 */
public class TestAnnoDeepAccess {

    @Retention(RetentionPolicy.RUNTIME)
    @Target(ElementType.TYPE)
    @interface Conf {
        String name() default "anon";

        int level() default 3;

        String[] tags() default { "a", "b" };
    }

    @Retention(RetentionPolicy.RUNTIME)
    @Target(ElementType.TYPE)
    @Inherited
    @interface InheritedMarker {
    }

    @Retention(RetentionPolicy.RUNTIME)
    @Target(ElementType.TYPE)
    @Repeatable(Hist.class)
    @interface Entry {
        String value();
    }

    @Retention(RetentionPolicy.RUNTIME)
    @Target(ElementType.TYPE)
    @interface Hist {
        Entry[] value();
    }

    @Conf(name = "custom", tags = { "x" })
    @InheritedMarker
    @Entry("one")
    @Entry("two")
    static class Base {
    }

    static class Child extends Base {
    }

    public static void main(String[] args) throws Exception {
        // 默认值与非默认值
        System.out.println("default-name=" + Conf.class.getMethod("name").getDefaultValue());
        System.out.println("default-level=" + Conf.class.getMethod("level").getDefaultValue());
        System.out.println("default-tags="
                + java.util.Arrays.toString((String[]) Conf.class.getMethod("tags").getDefaultValue()));

        Conf c = Base.class.getAnnotation(Conf.class);
        System.out.println("name=" + c.name() + " level=" + c.level()
                + " tags=" + java.util.Arrays.toString(c.tags()));

        // 注解代理物性：toString 形态 / 实例缓存同一性 / equals
        System.out.println("to-string=" + c.toString());
        Conf again = Base.class.getAnnotation(Conf.class);
        System.out.println("cached-same=" + (c == again) + " eq=" + c.equals(again));

        // @Inherited：子类可见、getDeclaredAnnotations 不可见（继承 vs 直接声明）
        System.out.println("child-inherited=" + Child.class.isAnnotationPresent(InheritedMarker.class));
        System.out.println("child-declared="
                + (Child.class.getDeclaredAnnotation(InheritedMarker.class) == null));

        // 可重复注解：getAnnotationsByType 展开 vs 容器注解
        Entry[] entries = Base.class.getAnnotationsByType(Entry.class);
        System.out.println("repeat-count=" + entries.length
                + " first=" + entries[0].value() + " second=" + entries[1].value());
        System.out.println("container=" + (Base.class.getAnnotation(Hist.class) != null));
        System.out.println("container-not-entry="
                + (Base.class.getAnnotation(Entry.class) == null));

        // 数组默认值防御性拷贝物性（两次取值非同一实例但等值）
        Conf other = Base.class.getAnnotation(Conf.class);
        System.out.println("array-eq=" + java.util.Arrays.equals(c.tags(), other.tags()));
    }
}
