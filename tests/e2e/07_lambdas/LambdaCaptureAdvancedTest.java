import java.util.ArrayList;
import java.util.Arrays;
import java.util.List;
import java.util.function.BiFunction;
import java.util.function.Function;

public class LambdaCaptureAdvancedTest {
    // Helper: apply a BiFunction
    static <T, U, R> R applyBi(BiFunction<T, U, R> fn, T a, U b) {
        return fn.apply(a, b);
    }

    // Helper: apply Function
    static <T, R> R applyFn(Function<T, R> fn, T arg) {
        return fn.apply(arg);
    }

    public static void main(String[] args) {
        // Test 1: BiFunction with captured int variable
        int offset = 10;
        int result1 = applyBi((a, b) -> (Integer)a + (Integer)b + offset, 3, 7);
        System.out.println("BiFunction capture: " + result1);

        // Test 2: Multiple String captures
        String prefix = "Hello";
        String suffix = "!";
        String result2 = applyFn(s -> prefix + " " + s + suffix, "World");
        System.out.println("Multi capture: " + result2);

        // Test 3: Captured int + ArrayList in forEach
        int factor = 5;
        List<Integer> nums = Arrays.asList(1, 2, 3, 4);
        ArrayList<Integer> results = new ArrayList<>();
        nums.forEach(n -> results.add(n * factor));
        System.out.println("Nested capture size: " + results.size());
        // Use explicit loop (avoid for-each which creates Iterator reusing variable slot)
        int sum = 0;
        for (int i = 0; i < results.size(); i++) sum += results.get(i);
        System.out.println("Nested capture sum: " + sum);

        // Test 4: Captured int[] array element access (reference semantics)
        int[] multiplier = {3};
        Function<Integer, Integer> tripler = x -> x * multiplier[0];
        System.out.println("Capture array + param: " + tripler.apply(7));

        // Test 5: Lambda returning captured reference type
        List<String> captured = Arrays.asList("A", "B", "C");
        Function<Integer, String> getter = i -> captured.get(i);
        System.out.println("Capture ref: " + getter.apply(1));

        // Test 6: Chained captures — lambda inside forEach
        int base = 100;
        ArrayList<Integer> mapped = new ArrayList<>();
        nums.forEach(n -> mapped.add(n + base));
        System.out.println("Chained: " + mapped.get(0) + "," + mapped.get(3));

        // Test 7: Capture a String and use in forEach
        String tag = "item";
        ArrayList<String> tagged = new ArrayList<>();
        nums.forEach(n -> tagged.add(tag + n));
        System.out.println("Tag: " + tagged.get(0));
    }
}
