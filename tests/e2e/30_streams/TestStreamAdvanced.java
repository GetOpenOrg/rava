import java.util.ArrayList;
import java.util.Arrays;
import java.util.List;
import java.util.Optional;
import java.util.OptionalDouble;
import java.util.stream.Collectors;
import java.util.stream.IntStream;
import java.util.stream.Stream;

public class TestStreamAdvanced {

    public static void main(String[] args) {
        // flatMap
        List<List<Integer>> nested = Arrays.asList(
            Arrays.asList(1, 2, 3),
            Arrays.asList(4, 5),
            Arrays.asList(6, 7, 8, 9)
        );
        List<Integer> flat = nested.stream()
            .flatMap(List::stream)
            .collect(Collectors.toList());
        System.out.println(flat); // [1, 2, 3, 4, 5, 6, 7, 8, 9]

        // IntStream.range
        int sumTo10 = IntStream.rangeClosed(1, 10).sum();
        System.out.println(sumTo10); // 55

        List<Integer> rangeList = IntStream.range(0, 5)
            .boxed()
            .collect(Collectors.toList());
        System.out.println(rangeList); // [0, 1, 2, 3, 4]

        // Stream.generate / limit
        List<Integer> zeros = Stream.generate(() -> 0).limit(5)
            .collect(Collectors.toList());
        System.out.println(zeros); // [0, 0, 0, 0, 0]

        // Stream.iterate
        List<Integer> powers = Stream.iterate(1, n -> n * 2)
            .limit(8)
            .collect(Collectors.toList());
        System.out.println(powers); // [1, 2, 4, 8, 16, 32, 64, 128]

        // Optional from stream
        List<Integer> nums = Arrays.asList(3, 1, 4, 1, 5, 9, 2, 6);
        Optional<Integer> first = nums.stream().filter(n -> n > 5).findFirst();
        System.out.println(first.isPresent()); // true
        System.out.println(first.get());       // 9

        Optional<Integer> none = nums.stream().filter(n -> n > 100).findFirst();
        System.out.println(none.isPresent());  // false
        System.out.println(none.orElse(-1));   // -1

        // peek (for debugging / side effects)
        List<String> peeked = new ArrayList<>();
        List<String> processed = Arrays.asList("hello", "world", "java")
            .stream()
            .peek(peeked::add)
            .map(String::toUpperCase)
            .collect(Collectors.toList());
        System.out.println(String.join(" ", peeked.stream().map(s -> "peek:" + s).collect(Collectors.toList())));
        System.out.println(processed); // [HELLO, WORLD, JAVA]

        // takeWhile / dropWhile (Java 9+)
        List<Integer> sorted = Arrays.asList(1, 2, 3, 4, 5, 6, 7, 8, 9, 10);
        List<Integer> taken = sorted.stream()
            .takeWhile(n -> n <= 5)
            .collect(Collectors.toList());
        System.out.println(taken); // [1, 2, 3, 4, 5]

        List<Integer> dropped = sorted.stream()
            .dropWhile(n -> n <= 5)
            .collect(Collectors.toList());
        System.out.println(dropped); // [6, 7, 8, 9, 10]

        // mapToInt + average
        List<String> words = Arrays.asList("hello", "hi", "hey", "howdy");
        OptionalDouble avg = words.stream()
            .mapToInt(String::length)
            .average();
        System.out.println(avg.isPresent()); // true
        System.out.printf("%.2f%n", avg.getAsDouble()); // 3.75
    }
}
