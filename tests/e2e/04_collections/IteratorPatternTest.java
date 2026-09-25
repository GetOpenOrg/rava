import java.util.ArrayList;
import java.util.Arrays;
import java.util.Iterator;
import java.util.List;
import java.util.HashMap;
import java.util.Map;

public class IteratorPatternTest {
    public static void main(String[] args) {
        // 1. Basic Iterator usage
        List<String> names = new ArrayList<>(Arrays.asList("Alice", "Bob", "Charlie"));
        Iterator<String> it = names.iterator();
        StringBuilder sb = new StringBuilder();
        while (it.hasNext()) {
            if (sb.length() > 0) sb.append(", ");
            sb.append(it.next());
        }
        System.out.println("iterator: " + sb.toString());

        // 2. For-each on list
        List<Integer> numbers = new ArrayList<>(Arrays.asList(1, 2, 3, 4, 5));
        int sum = 0;
        for (int n : numbers) {
            sum += n;
        }
        System.out.println("sum: " + sum);

        // 3. Map.entrySet iteration
        Map<String, Integer> scores = new HashMap<>();
        scores.put("Alice", 95);
        scores.put("Bob", 87);
        scores.put("Charlie", 92);

        List<String> entries = new ArrayList<>();
        for (Map.Entry<String, Integer> entry : scores.entrySet()) {
            entries.add(entry.getKey() + "=" + entry.getValue());
        }
        // Sort for deterministic output
        entries.sort(null);
        System.out.println("entries: " + entries);

        // 4. Map.keySet iteration
        List<String> keys = new ArrayList<>();
        for (String k : scores.keySet()) {
            keys.add(k);
        }
        keys.sort(null);
        System.out.println("keys: " + keys);

        // 5. Map.values iteration
        List<Integer> values = new ArrayList<>();
        for (int v : scores.values()) {
            values.add(v);
        }
        values.sort(null);
        System.out.println("values: " + values);

        // 6. Nested iteration
        List<List<Integer>> matrix = new ArrayList<>();
        matrix.add(new ArrayList<>(Arrays.asList(1, 2, 3)));
        matrix.add(new ArrayList<>(Arrays.asList(4, 5, 6)));
        matrix.add(new ArrayList<>(Arrays.asList(7, 8, 9)));
        int total = 0;
        for (List<Integer> row : matrix) {
            for (int val : row) {
                total += val;
            }
        }
        System.out.println("matrix sum: " + total);

        // 7. forEach lambda on Map
        List<String> forEachResults = new ArrayList<>();
        scores.forEach((k, v) -> forEachResults.add(k + ":" + v));
        forEachResults.sort(null);
        System.out.println("forEach: " + forEachResults);

        // 8. Map.getOrDefault
        System.out.println("getOrDefault Alice: " + scores.getOrDefault("Alice", 0));
        System.out.println("getOrDefault Dave: " + scores.getOrDefault("Dave", 0));

        // 9. Map.containsKey / containsValue
        System.out.println("containsKey Bob: " + scores.containsKey("Bob"));
        System.out.println("containsKey Dave: " + scores.containsKey("Dave"));

        // 10. Map.size / isEmpty
        System.out.println("size: " + scores.size());
        System.out.println("isEmpty: " + scores.isEmpty());

        System.out.println("Done.");
    }
}
