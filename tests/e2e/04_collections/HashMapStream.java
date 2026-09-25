import java.util.HashMap;

public class HashMapStream {
    public static void main(String[] args) {
        HashMap<String, Integer> map = new HashMap<>();
        map.put("a", 10);
        map.put("b", 20);
        map.put("c", 30);

        int sum = map.values().stream().mapToInt(Integer::intValue).sum();
        System.out.println(sum); // 60

        long filtered = map.values().stream().filter(v -> v >= 20).count();
        System.out.println(filtered); // 2

        System.out.println(map.size()); // 3
        System.out.println(map.keySet().size()); // 3
    }
}
