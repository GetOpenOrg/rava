import java.util.ArrayList;
import java.util.HashMap;
import java.util.HashSet;

public class TestCollections {
    public static void main(String[] args) {
        // ArrayList<Integer>
        ArrayList<Integer> list = new ArrayList<>();
        list.add(10);
        list.add(20);
        list.add(30);
        System.out.println(list.size());   // 3
        System.out.println(list.get(1));   // 20

        // HashMap<String, Integer>
        HashMap<String, Integer> map = new HashMap<>();
        map.put("one", 1);
        map.put("two", 2);
        map.put("three", 3);
        System.out.println(map.size());         // 3
        System.out.println(map.get("two"));     // 2
        System.out.println(map.containsKey("one")); // true

        // HashSet<Integer>
        HashSet<Integer> set = new HashSet<>();
        set.add(100);
        set.add(200);
        set.add(100);   // duplicate
        System.out.println(set.size());        // 2
        System.out.println(set.contains(200)); // true
    }
}
