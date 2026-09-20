import java.util.List;
import java.util.function.Function;

public class TestLambdaVar {
    public static void main(String[] args) {
        Function<Integer, Integer> f = (var x) -> x * x;
        System.out.println("varLambda=" + f.apply(6));
        Function<String, Integer> g = (var s) -> s.length();
        System.out.println("varLambdaLen=" + g.apply("abcd"));
        List<Integer> nums = List.of(1, 2, 3);
        int sum = nums.stream().map((var n) -> n + 1).reduce(0, Integer::sum);
        System.out.println("sum=" + sum);
        Runnable r = () -> System.out.println("lambda-ok");
        r.run();
    }
}
