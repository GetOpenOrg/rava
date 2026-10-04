import java.lang.annotation.Retention;
import java.lang.annotation.RetentionPolicy;

// 注解持有类只作类字面量（L1 不透明类）：类级注解的数组值、嵌套注解数组须随元数据表发射
public class AnnoArrays {
    @Retention(RetentionPolicy.RUNTIME)
    @interface Tags { String[] value(); int[] nums() default {}; }

    @Retention(RetentionPolicy.RUNTIME)
    @interface Inner { String value(); }

    @Retention(RetentionPolicy.RUNTIME)
    @interface Outer { Inner[] value(); }

    @Tags(value = {"alpha", "beta"}, nums = {7, 70000})
    static class ArrHolder {}

    @Outer({@Inner("x1"), @Inner("y2")})
    static class NestedHolder {}

    public static void main(String[] args) {
        System.out.println(ArrHolder.class.getAnnotation(Tags.class).value().length);
        System.out.println(NestedHolder.class.getAnnotation(Outer.class).value()[1].value());
    }
}
