// lambda 接收者调用 default 方法，default 体内含对外部类型的虚调用（println / StringBuilder / 其他接口）
import java.util.function.Supplier;

public class TestLambdaDefaultForeignCall {
    interface Greeter {
        String name();
        default void hello() { System.out.println("hi " + name()); }
        default String shout() { return new StringBuilder(name()).reverse().toString().toUpperCase(); }
        default int len(Supplier<String> s) { return s.get().length() + name().length(); }
    }

    interface Polite extends Greeter {
        default void hello() { System.out.println("good day, " + name()); }
    }

    public static void main(String[] args) {
        Greeter g = () -> "bob";
        g.hello();
        System.out.println(g.shout());
        System.out.println(g.len(() -> "four"));
        Polite p = () -> "ann";
        p.hello();
        Greeter asG = p;
        asG.hello();
        System.out.println(asG.shout());
    }
}
