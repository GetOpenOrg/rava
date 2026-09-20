import java.lang.annotation.*;

@Retention(RetentionPolicy.RUNTIME)
@Target({ElementType.METHOD, ElementType.TYPE, ElementType.FIELD})
@interface MyAnno {
    int value() default 1;
    String name() default "x";
}

@Retention(RetentionPolicy.RUNTIME)
@Target(ElementType.METHOD)
@Repeatable(RepeatHolder.class)
@interface Repeat {
    String v();
}

@Retention(RetentionPolicy.RUNTIME)
@Target(ElementType.METHOD)
@interface RepeatHolder {
    Repeat[] value();
}

public class TestAnnotations {
    @MyAnno(value = 5, name = "test")
    @Repeat(v = "a")
    @Repeat(v = "b")
    public static void annotated() {
        // 反射读注解被 callchain cutoff 截断，此处只验证注解语法被接受，
        // 以及编译期存在的注解元素可被普通计算引用。
        int computed = 5 + 1;
        System.out.println("annotated-run=" + computed);
    }

    @Deprecated(since = "1", forRemoval = false)
    public static void deprecatedMethod() {
        System.out.println("deprecated-ok");
    }

    @FunctionalInterface
    interface Op { int apply(int x); }

    public static void main(String[] args) {
        annotated();
        deprecatedMethod();
        Op op = x -> x * 2;
        System.out.println("op=" + op.apply(21));
        System.out.println("annoName=" + MyAnno.class.getSimpleName());
        System.out.println("holderName=" + RepeatHolder.class.getSimpleName());
    }
}
