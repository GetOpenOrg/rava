import java.util.ArrayList;
import java.util.Arrays;
import java.util.List;
import java.util.Optional;
import java.util.stream.Stream;
import java.util.stream.IntStream;
import java.util.stream.Collectors;

public class StreamAdvancedTest {
    public static void main(String[] args) {
        // 1. distinct
        List<Integer> nums = new ArrayList<>(Arrays.asList(1, 2, 2, 3, 3, 3, 4));
        List<Integer> unique = nums.stream().distinct().collect(Collectors.toList());
        System.out.println("distinct: " + unique.size());

        // 2. limit & skip
        List<Integer> first3 = nums.stream().limit(3).collect(Collectors.toList());
        System.out.println("limit 3: " + first3.size());
        List<Integer> skip3 = nums.stream().skip(3).collect(Collectors.toList());
        System.out.println("skip 3: " + skip3.size());

        // 3. peek (side effect + passthrough)
        List<String> peeked = new ArrayList<>();
        List<Integer> result = nums.stream()
            .peek(x -> peeked.add("saw:" + x))
            .filter(x -> x > 2)
            .collect(Collectors.toList());
        System.out.println("peek count: " + peeked.size());
        System.out.println("filter>2: " + result.size());

        // 4. findFirst
        Optional<Integer> first = nums.stream().filter(x -> x > 2).findFirst();
        System.out.println("findFirst>2: " + (first.isPresent() ? first.get() : -1));

        Optional<Integer> none = nums.stream().filter(x -> x > 100).findFirst();
        System.out.println("findFirst>100: " + (none.isPresent() ? none.get() : -1));

        // 5. anyMatch / allMatch / noneMatch
        boolean any = nums.stream().anyMatch(x -> x > 3);
        System.out.println("anyMatch>3: " + any);
        boolean all = nums.stream().allMatch(x -> x > 0);
        System.out.println("allMatch>0: " + all);
        boolean noneM = nums.stream().noneMatch(x -> x > 10);
        System.out.println("noneMatch>10: " + noneM);

        // 6. count
        long cnt = nums.stream().filter(x -> x % 2 == 0).count();
        System.out.println("count even: " + cnt);

        // 7. reduce with lambda
        int sum = nums.stream().reduce(0, (a, b) -> a + b);
        System.out.println("reduce sum: " + sum);

        // 8. mapToInt + sum
        List<String> words = new ArrayList<>(Arrays.asList("hello", "world", "hi"));
        int totalLen = words.stream().mapToInt(s -> s.length()).sum();
        System.out.println("mapToInt sum: " + totalLen);

        // 9. sorted with comparator (reverse order)
        List<Integer> sorted = nums.stream()
            .distinct()
            .sorted((a, b) -> b - a)
            .collect(Collectors.toList());
        System.out.println("sorted desc: " + sorted);

        // 10. collect joining
        String joined = words.stream().collect(Collectors.joining(", "));
        System.out.println("joining: " + joined);

        // 11. collect toSet (just check size)
        long setSize = nums.stream().collect(Collectors.toSet()).size();
        System.out.println("toSet size: " + setSize);

        // 12. IntStream range + sum
        int rangeSum = IntStream.range(1, 6).sum();
        System.out.println("IntStream sum 1-5: " + rangeSum);

        // 13. IntStream filter + count
        long evenCount = IntStream.rangeClosed(1, 10).filter(x -> x % 2 == 0).count();
        System.out.println("IntStream even 1-10: " + evenCount);

        // 14. Optional advanced
        Optional<String> optStr = Optional.of("hello");
        String upper = optStr.map(s -> s.toUpperCase()).orElse("NONE");
        System.out.println("Optional map: " + upper);

        Optional<String> empty = Optional.empty();
        String def = empty.orElse("default");
        System.out.println("orElse empty: " + def);

        boolean isEmpty = Optional.empty().isEmpty();
        System.out.println("isEmpty: " + isEmpty);

        // 15. flatMap
        List<List<Integer>> nested = new ArrayList<>();
        nested.add(new ArrayList<>(Arrays.asList(1, 2)));
        nested.add(new ArrayList<>(Arrays.asList(3, 4)));
        nested.add(new ArrayList<>(Arrays.asList(5)));
        List<Integer> flat = nested.stream()
            .flatMap(l -> l.stream())
            .collect(Collectors.toList());
        System.out.println("flatMap: " + flat);

        // 16. Stream.of
        List<Integer> ofResult = Stream.of(10, 20, 30)
            .filter(x -> x >= 20)
            .collect(Collectors.toList());
        System.out.println("Stream.of filter: " + ofResult);

        System.out.println("Done.");
    }
}
