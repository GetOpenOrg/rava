import java.util.HashMap;

public class HashMapComputeTest {
    public static void main(String[] args) {
        // --- computeIfAbsent ---
        HashMap<String, String> map1 = new HashMap<>();
        map1.put("a", "hello");

        // Key absent -> compute and insert
        String v1 = map1.computeIfAbsent("b", k -> k + "_value");
        System.out.println("computeIfAbsent absent: " + v1);
        System.out.println("map has b: " + map1.get("b"));

        // Key present -> return existing, don't compute
        String v2 = map1.computeIfAbsent("a", k -> "replaced");
        System.out.println("computeIfAbsent present: " + v2);

        // --- computeIfPresent ---
        HashMap<String, String> map2 = new HashMap<>();
        map2.put("x", "ten");

        // Key present -> update
        String v3 = map2.computeIfPresent("x", (k, v) -> k + "=" + v);
        System.out.println("computeIfPresent present: " + v3);
        System.out.println("map x: " + map2.get("x"));

        // Key absent -> null
        String v4 = map2.computeIfPresent("y", (k, v) -> k + "=" + v);
        System.out.println("computeIfPresent absent: " + v4);

        // --- compute (key present) ---
        HashMap<String, String> map3 = new HashMap<>();
        map3.put("a", "old");

        // Key present -> recompute
        String v5 = map3.compute("a", (k, v) -> k + "+" + v);
        System.out.println("compute present: " + v5);

        // --- merge ---
        HashMap<String, String> map4 = new HashMap<>();
        map4.put("a", "hello");

        // Key present -> merge
        String v7 = map4.merge("a", " world", (old, val) -> old + val);
        System.out.println("merge present: " + v7);

        // Key absent -> insert value directly
        String v8 = map4.merge("b", "new", (old, val) -> old + val);
        System.out.println("merge absent: " + v8);

        // --- replaceAll ---
        HashMap<String, String> map5 = new HashMap<>();
        map5.put("a", "1");
        map5.put("b", "2");
        map5.put("c", "3");

        map5.replaceAll((k, v) -> k + "=" + v);
        System.out.println("replaceAll a: " + map5.get("a"));
        System.out.println("replaceAll b: " + map5.get("b"));
        System.out.println("replaceAll c: " + map5.get("c"));

        System.out.println("All tests passed!");
    }
}
