import java.util.function.Function;
import java.util.function.Predicate;
import java.util.function.Consumer;
import java.util.function.Supplier;
import java.util.function.BiFunction;

public class TestFunctionalInterface {

    @FunctionalInterface
    interface Transformer<T> {
        T transform(T input);
    }

    @FunctionalInterface
    interface Combiner<A, B, R> {
        R combine(A a, B b);
    }

    public static void main(String[] args) {
        // Lambda as functional interface
        Transformer<String> upper = s -> s.toUpperCase();
        Transformer<Integer> doubled = n -> n * 2;

        System.out.println(upper.transform("hello"));  // HELLO
        System.out.println(doubled.transform(21));     // 42

        // Custom combiner
        Combiner<Integer, Integer, Integer> add = (a, b) -> a + b;
        Combiner<String, Integer, String> repeat = (s, n) -> s.repeat(n);

        System.out.println(add.combine(3, 4));          // 7
        System.out.println(repeat.combine("ab", 3));    // ababab

        // java.util.function.*
        Function<String, Integer> length = String::length;
        System.out.println(length.apply("hello"));      // 5

        Function<Integer, Integer> sq = x -> x * x;
        Function<Integer, String> sqStr = sq.andThen(Object::toString);
        System.out.println(sqStr.apply(5));             // 25

        Predicate<String> isEmpty = String::isEmpty;
        Predicate<String> notEmpty = isEmpty.negate();
        System.out.println(isEmpty.test(""));           // true
        System.out.println(notEmpty.test("hello"));     // true
        System.out.println(isEmpty.and(notEmpty).test("x")); // false

        Consumer<String> print = System.out::println;
        print.accept("consumer works");                 // consumer works

        Supplier<String> greeting = () -> "hello supplier";
        System.out.println(greeting.get());             // hello supplier

        BiFunction<Integer, Integer, Integer> multiply = (a, b) -> a * b;
        System.out.println(multiply.apply(6, 7));       // 42
    }
}
