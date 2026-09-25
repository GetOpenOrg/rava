import java.util.*;

public class ImmutableCollections {
    public static void main(String[] args) {
        // List.of
        List<String> list = List.of("a", "b", "c");
        System.out.println(list.size());        // 3
        System.out.println(list.get(1));        // b
        System.out.println(list.contains("c")); // true

        // Set.of
        Set<Integer> set = Set.of(10, 20, 30);
        System.out.println(set.size());         // 3
        System.out.println(set.contains(20));   // true
        System.out.println(set.contains(99));   // false

        // Map.of
        Map<String, Integer> map = Map.of("x", 1, "y", 2);
        System.out.println(map.size());         // 2
        System.out.println(map.get("x"));       // 1
        System.out.println(map.get("y"));       // 2
    }
}
