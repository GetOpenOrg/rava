import java.util.LinkedHashMap;

public class LinkedHashMapDemo {
    public static void main(String[] args) {
        LinkedHashMap<String, Integer> map = new LinkedHashMap<>();
        map.put("banana", 2);
        map.put("apple", 1);
        map.put("cherry", 3);

        // Should output in insertion order
        for (String key : map.keySet()) {
            System.out.println(key + "=" + map.get(key));
        }

        System.out.println("size=" + map.size());
        map.remove("apple");
        System.out.println("after remove size=" + map.size());
        System.out.println("containsKey banana=" + map.containsKey("banana"));
    }
}
