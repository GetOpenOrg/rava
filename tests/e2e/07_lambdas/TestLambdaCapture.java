import java.util.function.Function;
import java.util.function.IntUnaryOperator;
import java.util.function.Supplier;
import java.util.ArrayList;
import java.util.List;

public class TestLambdaCapture {

    private String instanceField = "field-value";
    private int count = 0;
    private static String staticField = "static-value";

    static String transform(String s, Function<String, String> f) {
        return f.apply(s);
    }

    Function<String, String> makeCapturingLambda() {
        String local = "captured-local";
        int n = 10;
        // 捕获 effectively final 的局部变量 + this(实例字段)
        return input -> instanceField + "|" + local + "|" + n + "|" + input;
    }

    Supplier<String> captureStaticAndCount() {
        return () -> {
            count++;
            return staticField + ":count=" + count;
        };
    }

    IntUnaryOperator makeAdder(int base) {
        return v -> v + base;
    }

    Supplier<String> captureThisExplicitly() {
        return () -> TestLambdaCapture.this.instanceField + "#explicit";
    }

    public static void main(String[] args) {
        TestLambdaCapture host = new TestLambdaCapture();

        // 捕获局部变量：lambda 返回后局部变量仍可用
        Function<String, String> f = host.makeCapturingLambda();
        System.out.println(f.apply("ARG"));
        System.out.println(f.apply("ARG2"));

        // 捕获实例字段并保持可变状态
        Supplier<String> s = host.captureStaticAndCount();
        System.out.println(s.get());
        System.out.println(s.get());
        System.out.println("host.count=" + host.count);

        // 参数捕获（每次调用形成独立捕获）
        IntUnaryOperator add5 = host.makeAdder(5);
        IntUnaryOperator add100 = host.makeAdder(100);
        System.out.println(add5.applyAsInt(1) + " " + add100.applyAsInt(1));

        System.out.println(host.captureThisExplicitly().get());

        // lambda 捕获后在 lambda 内部再构造 lambda（嵌套捕获）
        Function<Integer, Function<Integer, Integer>> curried = a -> (b -> a * b);
        Function<Integer, Integer> times3 = curried.apply(3);
        System.out.println("curried 3*4=" + times3.apply(4));

        // 循环中创建 lambda：每次迭代捕获不同的值
        List<Supplier<Integer>> makers = new ArrayList<>();
        for (int i = 0; i < 3; i++) {
            final int frozen = i * 10;
            makers.add(() -> frozen);
        }
        int total = 0;
        for (Supplier<Integer> sup : makers) {
            total += sup.get();
        }
        System.out.println("loop capture total=" + total);

        // lambda 捕获后跨方法传递
        System.out.println(transform("x", host.makeCapturingLambda()));

        // 静态上下文的 lambda（不捕获 this）
        Function<String, String> upper = str -> str.toUpperCase() + "!";
        System.out.println(upper.apply("abc"));

        System.out.println("done");
    }
}
