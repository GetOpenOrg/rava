import java.util.ArrayList;
import java.util.List;
import java.util.stream.Collectors;
import java.util.stream.IntStream;
import java.util.stream.Stream;

public class StreamTest {
    public static void main(String[] args) {
        // 1. filter + collect(toList)
        ArrayList<Integer> numbers = new ArrayList<>();
        numbers.add(1);
        numbers.add(2);
        numbers.add(3);
        numbers.add(4);
        numbers.add(5);
        numbers.add(6);

        List<Integer> evens = numbers.stream()
            .filter(n -> n % 2 == 0)
            .collect(Collectors.toList());
        System.out.println("Evens: " + evens);

        // 2. map + collect(toList)
        List<Integer> doubled = numbers.stream()
            .map(n -> n * 2)
            .collect(Collectors.toList());
        System.out.println("Doubled: " + doubled);

        // 3. forEach — count via stream
        long forEachCount = numbers.stream().count();
        System.out.println("ForEach count: " + forEachCount);

        // 4. reduce — sum
        int sum = (Integer) numbers.stream()
            .reduce(0, (a, b) -> (Integer) a + (Integer) b);
        System.out.println("Sum: " + sum);

        // 5. count
        long count = numbers.stream().count();
        System.out.println("Count: " + count);

        // 6. findFirst
        System.out.println("First: " + numbers.stream().findFirst().get());

        // 7. sorted + collect
        ArrayList<Integer> unsorted = new ArrayList<>();
        unsorted.add(5);
        unsorted.add(1);
        unsorted.add(3);
        unsorted.add(2);
        unsorted.add(4);
        List<Integer> sorted = unsorted.stream()
            .sorted()
            .collect(Collectors.toList());
        System.out.println("Sorted: " + sorted);

        // 8. filter + map chain
        List<Integer> result = numbers.stream()
            .filter(n -> n > 2)
            .map(n -> n * 10)
            .collect(Collectors.toList());
        System.out.println("Filtered+Mapped: " + result);

        // 9. IntStream.range + sum
        int rangeSum = IntStream.range(1, 6).sum();
        System.out.println("IntStream range sum: " + rangeSum);

        // 10. anyMatch / noneMatch
        boolean hasEven = numbers.stream().anyMatch(n -> n % 2 == 0);
        boolean noneNegative = numbers.stream().noneMatch(n -> n < 0);
        System.out.println("Has even: " + hasEven);
        System.out.println("None negative: " + noneNegative);
    }
}
