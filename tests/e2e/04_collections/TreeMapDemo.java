import java.util.TreeMap;

public class TreeMapDemo {
    public static void main(String[] args) {
        TreeMap<String, String> map = new TreeMap<>();
        map.put("banana", "yellow");
        map.put("apple", "green");
        map.put("cherry", "red");
        System.out.println(map.size());
        System.out.println(map.get("apple"));
        System.out.println(map.containsKey("cherry"));
        map.remove("banana");
        System.out.println(map.size());
    }
}
