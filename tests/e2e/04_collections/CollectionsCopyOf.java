import java.util.Arrays;
import java.util.List;
import java.util.Map;
import java.util.Set;
import java.util.TreeMap;
import java.util.TreeSet;

public class CollectionsCopyOf {
    public static void main(String[] args) {
        List<String> mutableList = Arrays.asList("New", "Method", "To", "Copy");

        // List.copyOf preserves insertion order — deterministic
        List<String> immutableList = List.copyOf(mutableList);
        immutableList.forEach(System.out::println);

        // Set.copyOf has undefined iteration order — sort before printing
        Set<String> immutableSet = Set.copyOf(mutableList);
        new TreeSet<>(immutableSet).forEach(System.out::println);

        // Map.copyOf has undefined iteration order — sort by key before printing
        Map<String, String> immutableMap = Map.copyOf(Map.of("k1", "v1", "k2", "v2"));
        new TreeMap<>(immutableMap).forEach((key, value) -> System.out.println(key + " = " + value));
    }
}
