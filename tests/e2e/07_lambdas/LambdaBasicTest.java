import java.util.function.Function;
import java.util.function.Predicate;
import java.util.function.Consumer;

public class LambdaBasicTest {
    public static void main(String[] args) {
        // Basic lambda: x -> x * 2
        Function<Integer, Integer> doubler = x -> x * 2;
        System.out.println("Double 5: " + doubler.apply(5));
        System.out.println("Double 10: " + doubler.apply(10));

        // Predicate: x -> x > 0
        Predicate<Integer> isPositive = x -> x > 0;
        System.out.println("5 > 0: " + isPositive.test(5));
        System.out.println("-1 > 0: " + isPositive.test(-1));

        // Consumer: x -> print it
        Consumer<String> printer = s -> System.out.println("Got: " + s);
        printer.accept("hello");
        printer.accept("world");
    }
}
