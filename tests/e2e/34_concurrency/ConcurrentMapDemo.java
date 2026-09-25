import java.util.concurrent.ConcurrentHashMap;
import java.util.*;

public class ConcurrentMapDemo {
    public static void main(String[] args) {
        ConcurrentHashMap<String, Integer> map = new ConcurrentHashMap<>();
        map.put("a", 1);
        map.put("b", 2);
        map.put("c", 3);

        System.out.println(map.get("a"));
        System.out.println(map.get("b"));
        System.out.println(map.containsKey("c"));
        System.out.println(map.containsKey("d"));
        System.out.println(map.size());

        // putIfAbsent
        map.putIfAbsent("d", 4);
        map.putIfAbsent("a", 99); // should not overwrite
        System.out.println(map.get("d"));
        System.out.println(map.get("a")); // still 1

        // remove
        map.remove("b");
        System.out.println(map.containsKey("b"));
        System.out.println(map.size());

        // iteration (sorted for determinism)
        List<String> keys = new ArrayList<>(map.keySet());
        Collections.sort(keys);
        for (String k : keys) {
            System.out.println(k + "=" + map.get(k));
        }

        // getOrDefault
        System.out.println(map.getOrDefault("z", -1));
        System.out.println(map.getOrDefault("a", -1));

        System.out.println("done");
    }
}
