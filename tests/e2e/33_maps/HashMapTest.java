import java.util.HashMap;

public class HashMapTest {
    public static void main(String[] args) {
        HashMap<String, Integer> map = new HashMap<>();
        map.put("one", 1);
        map.put("two", 2);
        map.put("three", 3);

        System.out.println(map.size());
        System.out.println(map.get("one"));
        System.out.println(map.get("two"));
        System.out.println(map.containsKey("three"));
        System.out.println(map.containsKey("four"));

        map.remove("two");
        System.out.println(map.size());
        System.out.println(map.isEmpty());
    }
}
