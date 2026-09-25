import java.util.ArrayList;
import java.util.HashMap;
import java.util.HashSet;
import java.util.List;
import java.util.Map;
import java.util.Collections;
import java.util.Arrays;

public class CollectionAdvancedTest {
    public static void main(String[] args) {
        // 1. Nested collections: Map<String, List<Integer>>
        Map<String, List<Integer>> groups = new HashMap<>();
        groups.put("evens", new ArrayList<>(Arrays.asList(2, 4, 6)));
        groups.put("odds", new ArrayList<>(Arrays.asList(1, 3, 5)));
        System.out.println("evens: " + groups.get("evens"));
        System.out.println("odds: " + groups.get("odds"));

        // 2. HashMap putIfAbsent + computeIfAbsent
        Map<String, Integer> counts = new HashMap<>();
        counts.put("a", 1);
        counts.putIfAbsent("a", 99);
        counts.putIfAbsent("b", 2);
        System.out.println("counts: " + counts.get("a") + "," + counts.get("b"));

        // 3. Collections.unmodifiableList behavior (via nCopies)
        List<String> copies = Collections.nCopies(3, "x");
        System.out.println("copies: " + copies);

        // 4. List.of / Map.of
        List<Integer> immutable = List.of(10, 20, 30);
        System.out.println("immutable: " + immutable);

        // 5. HashMap merge
        Map<String, Integer> m = new HashMap<>();
        m.put("x", 1);
        m.merge("x", 2, (a, b) -> a + b);
        m.merge("y", 5, (a, b) -> a + b);
        System.out.println("merge: x=" + m.get("x") + " y=" + m.get("y"));

        // 6. HashMap replaceAll
        Map<String, Integer> prices = new HashMap<>();
        prices.put("apple", 100);
        prices.put("banana", 200);
        prices.replaceAll((k, v) -> v * 2);
        System.out.println("prices: apple=" + prices.get("apple") + " banana=" + prices.get("banana"));

        // 7. ArrayList removeAll / retainAll
        List<Integer> a = new ArrayList<>(Arrays.asList(1, 2, 3, 4, 5));
        List<Integer> b = new ArrayList<>(Arrays.asList(2, 4, 6));
        List<Integer> copy = new ArrayList<>(a);
        copy.removeAll(b);
        System.out.println("removeAll: " + copy);
        List<Integer> copy2 = new ArrayList<>(a);
        copy2.retainAll(b);
        System.out.println("retainAll: " + copy2);

        // 8. Collections.frequency
        List<Integer> freq = new ArrayList<>(Arrays.asList(1, 2, 2, 3, 3, 3));
        System.out.println("freq(3): " + Collections.frequency(freq, 3));

        // 9. HashSet from collection
        HashSet<Integer> set = new HashSet<>(Arrays.asList(1, 2, 3, 2, 1));
        System.out.println("set size: " + set.size());

        System.out.println("Done.");
    }
}
