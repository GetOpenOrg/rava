import java.util.ArrayList;
import java.util.Arrays;
import java.util.List;
import java.util.function.Function;
import java.util.function.Predicate;
import java.util.stream.Collectors;

public class FunctionalCompositionTest {
    public static void main(String[] args) {
        // 1. Function.andThen
        Function<Integer, Integer> doubleIt = x -> x * 2;
        Function<Integer, Integer> addTen = x -> x + 10;
        Function<Integer, Integer> doubleThenAdd = doubleIt.andThen(addTen);
        System.out.println("andThen(5): " + doubleThenAdd.apply(5)); // 5*2+10=20

        // 2. Function.compose
        Function<Integer, Integer> addThenDouble = doubleIt.compose(addTen);
        System.out.println("compose(5): " + addThenDouble.apply(5)); // (5+10)*2=30

        // 3. Predicate.and / or / negate
        Predicate<Integer> isPositive = x -> x > 0;
        Predicate<Integer> isEven = x -> x % 2 == 0;
        Predicate<Integer> positiveAndEven = isPositive.and(isEven);
        Predicate<Integer> positiveOrEven = isPositive.or(isEven);
        Predicate<Integer> notPositive = isPositive.negate();

        System.out.println("4 pos&even: " + positiveAndEven.test(4));
        System.out.println("3 pos&even: " + positiveAndEven.test(3));
        System.out.println("-2 pos|even: " + positiveOrEven.test(-2));
        System.out.println("-3 pos|even: " + positiveOrEven.test(-3));
        System.out.println("-1 !pos: " + notPositive.test(-1));
        System.out.println("1 !pos: " + notPositive.test(1));

        // 4. Function as parameter
        List<Integer> nums = new ArrayList<>(Arrays.asList(1, 2, 3, 4, 5));
        List<Integer> squared = applyToAll(nums, x -> x * x);
        System.out.println("squared: " + squared);

        List<Integer> doubled = applyToAll(nums, x -> x * 2);
        System.out.println("doubled: " + doubled);

        // 5. Predicate as parameter
        List<Integer> filtered = filterAll(nums, x -> x > 3);
        System.out.println("filtered>3: " + filtered);

        List<Integer> evens = filterAll(nums, x -> x % 2 == 0);
        System.out.println("evens: " + evens);

        // 6. Method reference with stream
        List<String> words = new ArrayList<>(Arrays.asList("hello", "world", "hi"));
        List<String> upper = words.stream().map(String::toUpperCase).collect(Collectors.toList());
        System.out.println("upper: " + upper);

        // 6b. Method ref toLowerCase
        List<String> lower = new ArrayList<>(Arrays.asList("HELLO", "WORLD")).stream()
            .map(String::toLowerCase).collect(Collectors.toList());
        System.out.println("lower: " + lower);

        // 7. Chaining stream operations with lambdas
        List<Integer> result = nums.stream()
            .filter(x -> x % 2 == 1)
            .map(x -> x * 10)
            .collect(Collectors.toList());
        System.out.println("odd*10: " + result);

        // 8. Function returning function (higher-order)
        Function<Integer, Integer> multiplier = makeMultiplier(3);
        System.out.println("multiplier(5): " + multiplier.apply(5));
        System.out.println("multiplier(7): " + multiplier.apply(7));

        // 9. Multiple function parameters
        List<Integer> processed = processAll(nums, x -> x + 10, x -> x > 12);
        System.out.println("processed: " + processed);

        // 10. Lambda capturing local variable
        int factor = 100;
        List<Integer> scaled = applyToAll(nums, x -> x * factor);
        System.out.println("scaled: " + scaled);

        System.out.println("Done.");
    }

    static List<Integer> applyToAll(List<Integer> list, Function<Integer, Integer> f) {
        List<Integer> result = new ArrayList<>();
        for (int x : list) {
            result.add(f.apply(x));
        }
        return result;
    }

    static List<Integer> filterAll(List<Integer> list, Predicate<Integer> p) {
        List<Integer> result = new ArrayList<>();
        for (int x : list) {
            if (p.test(x)) {
                result.add(x);
            }
        }
        return result;
    }

    static Function<Integer, Integer> makeMultiplier(int factor) {
        return x -> x * factor;
    }

    static List<Integer> processAll(List<Integer> list, Function<Integer, Integer> transform, Predicate<Integer> filter) {
        List<Integer> result = new ArrayList<>();
        for (int x : list) {
            int transformed = transform.apply(x);
            if (filter.test(transformed)) {
                result.add(transformed);
            }
        }
        return result;
    }
}
