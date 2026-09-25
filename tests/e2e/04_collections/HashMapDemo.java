import java.util.HashMap;

public class HashMapDemo {
    public static void main(String[] args) {
        HashMap<String, String> map = new HashMap<>();
        map.put("alpha", "first");
        map.put("beta", "second");
        map.put("gamma", "third");
        System.out.println(map.get("beta"));
        System.out.println(map.containsKey("alpha"));
        System.out.println(map.size());
        map.remove("gamma");
        System.out.println(map.size());
    }
}
