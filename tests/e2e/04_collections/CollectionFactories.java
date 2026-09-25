import java.util.*;

public class CollectionFactories {
    public static void main(String[] args) {
        // new ArrayList<>(collection) — copy constructor
        List<String> original = new ArrayList<>();
        original.add("a");
        original.add("b");
        original.add("c");

        List<String> copy = new ArrayList<>(original);
        System.out.println(copy.size());   // 3

        // Map.getOrDefault
        Map<String, Integer> map = new HashMap<>();
        map.put("x", 1);
        System.out.println(map.getOrDefault("x", 0));  // 1
        System.out.println(map.getOrDefault("y", 99)); // 99

        // Collections.unmodifiableList
        List<String> unmod = Collections.unmodifiableList(original);
        System.out.println(unmod.size());   // 3
        System.out.println(unmod.get(0));   // a
    }
}
