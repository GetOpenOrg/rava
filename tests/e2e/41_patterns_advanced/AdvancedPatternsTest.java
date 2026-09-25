import java.util.ArrayList;
import java.util.Arrays;
import java.util.Collections;
import java.util.HashMap;
import java.util.List;
import java.util.Map;
import java.util.Optional;
import java.util.StringJoiner;
import java.util.stream.Collectors;

public class AdvancedPatternsTest {
    public static void main(String[] args) {
        // 1. HashMap merge for word counting
        Map<String, Integer> wordCount = new HashMap<>();
        List<String> text = new ArrayList<>(Arrays.asList("apple", "banana", "apple", "cherry", "banana", "apple"));
        for (String word : text) {
            wordCount.merge(word, 1, (a, b) -> a + b);
        }
        List<String> counts = new ArrayList<>();
        wordCount.forEach((k, v) -> counts.add(k + "=" + v));
        Collections.sort(counts);
        System.out.println("wordCount: " + counts);

        // 2. Optional chaining
        Optional<String> opt = Optional.of("hello world");
        String result = opt
            .map(s -> s.toUpperCase())
            .filter(s -> s.contains("HELLO"))
            .map(s -> s.replace("WORLD", "JAVA"))
            .orElse("fallback");
        System.out.println("optional chain: " + result);

        Optional<String> empty = Optional.empty();
        String fallback = empty
            .map(s -> s.toUpperCase())
            .orElse("default");
        System.out.println("empty chain: " + fallback);

        // 3. StringJoiner
        StringJoiner sj = new StringJoiner(", ", "[", "]");
        sj.add("one");
        sj.add("two");
        sj.add("three");
        System.out.println("joiner: " + sj.toString());

        StringJoiner emptySj = new StringJoiner(", ", "{", "}");
        System.out.println("empty joiner: " + emptySj.toString());

        // 4. HashMap getOrDefault + containsKey
        Map<String, Integer> scores = new HashMap<>();
        scores.put("Alice", 95);
        scores.put("Bob", 87);
        System.out.println("Alice: " + scores.getOrDefault("Alice", 0));
        System.out.println("Dave: " + scores.getOrDefault("Dave", 0));
        System.out.println("has Bob: " + scores.containsKey("Bob"));
        System.out.println("has Eve: " + scores.containsKey("Eve"));

        // 5. HashMap replaceAll
        Map<String, Integer> prices = new HashMap<>();
        prices.put("apple", 100);
        prices.put("banana", 200);
        prices.replaceAll((k, v) -> v * 2);
        List<String> priceList = new ArrayList<>();
        prices.forEach((k, v) -> priceList.add(k + "=" + v));
        Collections.sort(priceList);
        System.out.println("doubled prices: " + priceList);

        // 6. List copy independence
        List<Integer> original = new ArrayList<>(Arrays.asList(1, 2, 3));
        List<Integer> copy = new ArrayList<>();
        for (int x : original) copy.add(x);
        copy.add(4);
        System.out.println("original: " + original);
        System.out.println("copy: " + copy);

        // 7. Collections reverse + frequency
        List<Integer> nums = new ArrayList<>(Arrays.asList(1, 2, 3, 2, 1));
        Collections.reverse(nums);
        System.out.println("reversed: " + nums);
        System.out.println("freq of 2: " + Collections.frequency(nums, 2));

        // 8. HashMap putIfAbsent
        Map<String, Integer> map = new HashMap<>();
        map.put("a", 1);
        map.putIfAbsent("a", 999);
        map.putIfAbsent("b", 2);
        System.out.println("a: " + map.get("a"));
        System.out.println("b: " + map.get("b"));

        // 9. Stream filter by string length + joining
        List<String> words = new ArrayList<>(Arrays.asList("apple", "banana", "avocado", "blueberry", "cherry"));
        String joined = words.stream()
            .filter(w -> w.length() > 5)
            .sorted()
            .collect(Collectors.joining(", "));
        System.out.println("long words: " + joined);

        // 10. Stream map to lengths
        List<Integer> lengths = words.stream()
            .map(w -> w.length())
            .collect(Collectors.toList());
        System.out.println("lengths: " + lengths);

        // 11. entrySet for-each loop
        List<String> entries = new ArrayList<>();
        for (Map.Entry<String, Integer> e : scores.entrySet()) {
            entries.add(e.getKey() + "=" + e.getValue());
        }
        Collections.sort(entries);
        System.out.println("entries: " + entries);

        System.out.println("Done.");
    }
}
