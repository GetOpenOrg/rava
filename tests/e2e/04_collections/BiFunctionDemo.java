import java.util.function.*;
import java.util.*;

public class BiFunctionDemo {
    public static void main(String[] args) {
        // BiFunction
        BiFunction<String, Integer, String> repeat = (s, n) -> s.repeat(n);
        System.out.println(repeat.apply("ab", 3));  // ababab

        // BiPredicate
        BiPredicate<String, String> startsWith = (s, prefix) -> s.startsWith(prefix);
        System.out.println(startsWith.test("hello", "he"));  // true

        // BiConsumer
        List<String> results = new ArrayList<>();
        BiConsumer<String, Integer> addN = (s, n) -> {
            for (int i = 0; i < n; i++) results.add(s);
        };
        addN.accept("x", 3);
        System.out.println(results.size());  // 3

        // UnaryOperator
        UnaryOperator<String> upper = String::toUpperCase;
        System.out.println(upper.apply("hello"));  // HELLO

        // IntUnaryOperator
        IntUnaryOperator doubler = n -> n * 2;
        System.out.println(doubler.applyAsInt(5));  // 10

        // ToIntFunction
        ToIntFunction<String> len = String::length;
        System.out.println(len.applyAsInt("hello world"));  // 11
    }
}
