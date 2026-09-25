import java.util.*;

public class MapEntryDemo {
    public static void main(String[] args) {
        Map<String, Integer> scores = new HashMap<>();
        scores.put("Alice", 90);
        scores.put("Bob", 85);
        scores.put("Charlie", 92);

        // entrySet iteration: sum all values
        int total = 0;
        for (Map.Entry<String, Integer> entry : scores.entrySet()) {
            total += entry.getValue();
        }
        System.out.println(total); // 267

        // Find max entry
        String topName = "";
        int topScore = 0;
        for (Map.Entry<String, Integer> entry : scores.entrySet()) {
            if (entry.getValue() > topScore) {
                topScore = entry.getValue();
                topName = entry.getKey();
            }
        }
        System.out.println(topName);  // Charlie
        System.out.println(topScore); // 92

        // TreeMap entrySet (sorted keys)
        TreeMap<String, Integer> sorted = new TreeMap<>(scores);
        for (Map.Entry<String, Integer> entry : sorted.entrySet()) {
            System.out.println(entry.getKey() + "=" + entry.getValue());
        }
        // Alice=90, Bob=85, Charlie=92 (sorted by key)
    }
}
