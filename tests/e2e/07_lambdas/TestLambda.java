import java.util.ArrayList;
import java.util.Arrays;
import java.util.Collections;
import java.util.function.Function;
import java.util.function.Predicate;

public class TestLambda {

    @FunctionalInterface
    interface Transformer {
        int transform(int x);
    }

    static int apply(int value, Transformer t) {
        return t.transform(value);
    }

    public static void main(String[] args) {
        // 基本 lambda
        Transformer doubler = x -> x * 2;
        System.out.println(apply(5, doubler));
        System.out.println(apply(3, x -> x + 10));

        // ArrayList.forEach
        ArrayList<String> names = new ArrayList<>();
        names.add("Alice");
        names.add("Bob");
        names.add("Charlie");
        names.forEach(name -> System.out.println(name));

        // Predicate
        Predicate<String> isLong = s -> s.length() > 4;
        System.out.println(isLong.test("Alice"));
        System.out.println(isLong.test("Bob"));

        // Function
        Function<String, Integer> len = s -> s.length();
        System.out.println(len.apply("Hello"));

        // Collections.sort + Comparator lambda
        ArrayList<Integer> nums = new ArrayList<>(Arrays.asList(3, 1, 4, 1, 5, 9, 2, 6));
        Collections.sort(nums, (a, b) -> a - b);
        System.out.println(nums.get(0));
        System.out.println(nums.get(nums.size() - 1));

        // 方法引用
        names.forEach(System.out::println);

        // 多行 lambda（块体）
        Transformer factorial = n -> {
            int result = 1;
            for (int i = 2; i <= n; i++) result *= i;
            return result;
        };
        System.out.println(apply(5, factorial));
    }
}
