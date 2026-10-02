import java.lang.annotation.Repeatable;
import java.lang.annotation.Retention;
import java.lang.annotation.RetentionPolicy;
import java.util.Arrays;

/**
 * 注解 toString 打印注解数组成员：数组元素是嵌套注解（代理对象）时逐个经其 toString 虚调用拼接。
 * 覆盖：单层 / 两层嵌套、空数组、单元素数组、字符串转义、负数、@Repeatable 容器，以及
 * 数组成员取值后的 equals / hashCode 一致性。各注解只有单个成员，toString 文本与成员迭代序无关。
 */
public class TestAnnoNestedArray {
    @Retention(RetentionPolicy.RUNTIME)
    @interface Inner { int value(); }

    @Retention(RetentionPolicy.RUNTIME)
    @interface Inners { Inner[] value(); }

    @Retention(RetentionPolicy.RUNTIME)
    @interface Leaf { String value(); }

    @Retention(RetentionPolicy.RUNTIME)
    @interface Branch { Leaf[] value(); }

    @Retention(RetentionPolicy.RUNTIME)
    @interface Tree { Branch[] value(); }

    @Retention(RetentionPolicy.RUNTIME)
    @interface Nothing { Inner[] value(); }

    @Retention(RetentionPolicy.RUNTIME)
    @interface Tags { Tag[] value(); }

    @Retention(RetentionPolicy.RUNTIME)
    @Repeatable(Tags.class)
    @interface Tag { String value(); }

    @Inners({@Inner(1), @Inner(-2)})
    @Tree({@Branch({@Leaf("a"), @Leaf("b\"c")}), @Branch({})})
    @Nothing({})
    @Tag("x")
    @Tag("y")
    static class Holder {
    }

    @Inners(@Inner(7))
    static class Single {
    }

    public static void main(String[] args) {
        Inners inners = Holder.class.getAnnotation(Inners.class);
        System.out.println(inners);
        System.out.println(Arrays.toString(inners.value()));
        System.out.println(Holder.class.getAnnotation(Tree.class));
        System.out.println(Holder.class.getAnnotation(Nothing.class));
        System.out.println(Holder.class.getAnnotation(Tags.class));
        System.out.println(Arrays.toString(Holder.class.getAnnotationsByType(Tag.class)));
        Inners single = Single.class.getAnnotation(Inners.class);
        System.out.println(single);
        System.out.println("equal=" + inners.equals(Holder.class.getAnnotation(Inners.class))
                + " differ=" + inners.equals(single)
                + " hash=" + (inners.hashCode() == Holder.class.getAnnotation(Inners.class).hashCode()));
        Inner first = inners.value()[0];
        System.out.println(first + " value=" + first.value() + " type=" + first.annotationType().getSimpleName());
    }
}
