import java.util.ArrayList;
import java.util.HashMap;

public class JavaBaseAlphaTest {
    public static void main(String[] args) {
        ArrayList<String> list = new ArrayList<>();
        list.add("hello");
        list.add("world");
        System.out.println(list.size());
        System.out.println(list.get(0));

        HashMap<String, Integer> map = new HashMap<>();
        map.put("a", 1);
        map.put("b", 2);
        System.out.println(map.size());
        System.out.println(map.get("a"));
    }
}
