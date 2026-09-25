import java.util.HashMap;
import java.util.TreeMap;
import java.util.LinkedHashMap;

public class MapAdvancedTest {
    public static void main(String[] args) {
        // Test 1: putIfAbsent
        HashMap<String, Integer> map = new HashMap<>();
        map.put("a", 1);
        map.putIfAbsent("a", 99);
        map.putIfAbsent("b", 2);
        System.out.println(map.get("a"));  // 1
        System.out.println(map.get("b"));  // 2

        // Test 2: getOrDefault
        System.out.println(map.getOrDefault("a", 0));   // 1
        System.out.println(map.getOrDefault("z", -1));  // -1

        // Test 3: compute
        map.compute("a", (k, v) -> v + 10);
        System.out.println(map.get("a"));  // 11

        // Test 4: merge
        map.merge("a", 100, (v1, v2) -> v1 + v2);
        System.out.println(map.get("a"));  // 111
        map.merge("c", 3, (v1, v2) -> v1 + v2);
        System.out.println(map.get("c"));  // 3

        // Test 5: replaceAll
        map.replaceAll((k, v) -> v * 2);
        System.out.println(map.get("a"));  // 222
        System.out.println(map.get("b"));  // 4
        System.out.println(map.get("c"));  // 6

        // Test 6: TreeMap — sorted order
        TreeMap<String, Integer> treeMap = new TreeMap<>();
        treeMap.put("banana", 2);
        treeMap.put("apple", 1);
        treeMap.put("cherry", 3);
        System.out.println(treeMap.toString());  // {apple=1, banana=2, cherry=3}
        System.out.println(treeMap.firstKey());  // apple
        System.out.println(treeMap.lastKey());   // cherry

        // Test 7: LinkedHashMap — insertion order
        LinkedHashMap<String, Integer> linked = new LinkedHashMap<>();
        linked.put("c", 3);
        linked.put("a", 1);
        linked.put("b", 2);
        System.out.println(linked.toString());  // {c=3, a=1, b=2}

        // Test 8: containsKey
        System.out.println(linked.containsKey("a"));    // true
        System.out.println(linked.containsKey("z"));    // false

        // Test 9: remove + size
        linked.remove("a");
        System.out.println(linked.size());  // 2
        System.out.println(linked.containsKey("a"));  // false

        // Test 10: Word frequency count pattern using HashMap.merge
        String[] words = {"apple", "banana", "apple", "cherry", "banana", "apple"};
        HashMap<String, Integer> freq = new HashMap<>();
        for (String w : words) {
            freq.merge(w, 1, (a, b) -> a + b);
        }
        System.out.println(freq.get("apple"));   // 3
        System.out.println(freq.get("banana"));  // 2
        System.out.println(freq.get("cherry"));  // 1

        // Test 11: isEmpty / clear
        System.out.println(freq.isEmpty());  // false
        freq.clear();
        System.out.println(freq.isEmpty());  // true
        System.out.println(freq.size());     // 0
    }
}
