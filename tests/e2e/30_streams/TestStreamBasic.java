import java.util.Arrays;
import java.util.List;
import java.util.stream.Collectors;

public class TestStreamBasic {

    public static void main(String[] args) {
        List<Integer> nums = Arrays.asList(1, 2, 3, 4, 5, 6, 7, 8, 9, 10);

        // filter
        List<Integer> evens = nums.stream()
            .filter(n -> n % 2 == 0)
            .collect(Collectors.toList());
        System.out.println(evens); // [2, 4, 6, 8, 10]

        // map
        List<Integer> squares = nums.stream()
            .map(n -> n * n)
            .collect(Collectors.toList());
        System.out.println(squares); // [1, 4, 9, 16, 25, 36, 49, 64, 81, 100]

        // filter + map
        List<Integer> evenSquares = nums.stream()
            .filter(n -> n % 2 == 0)
            .map(n -> n * n)
            .collect(Collectors.toList());
        System.out.println(evenSquares); // [4, 16, 36, 64, 100]

        // reduce
        int sum = nums.stream().reduce(0, Integer::sum);
        System.out.println(sum); // 55

        int product = nums.stream()
            .filter(n -> n <= 5)
            .reduce(1, (a, b) -> a * b);
        System.out.println(product); // 120

        // count
        long count = nums.stream().filter(n -> n > 5).count();
        System.out.println(count); // 5

        // anyMatch, allMatch, noneMatch
        System.out.println(nums.stream().anyMatch(n -> n > 9));  // true
        System.out.println(nums.stream().allMatch(n -> n > 0));  // true
        System.out.println(nums.stream().noneMatch(n -> n > 10)); // true

        // min, max
        System.out.println(nums.stream().min(Integer::compare).get()); // 1
        System.out.println(nums.stream().max(Integer::compare).get()); // 10

        // forEach
        List<String> names = Arrays.asList("Alice", "Bob", "Charlie");
        names.stream().forEach(System.out::println);
        // Alice, Bob, Charlie

        // distinct + sorted
        List<Integer> dupes = Arrays.asList(3, 1, 4, 1, 5, 9, 2, 6, 5, 3);
        List<Integer> unique = dupes.stream()
            .distinct()
            .sorted()
            .collect(Collectors.toList());
        System.out.println(unique); // [1, 2, 3, 4, 5, 6, 9]
    }
}
