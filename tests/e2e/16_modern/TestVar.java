import java.util.ArrayList;
import java.util.TreeMap;

public class TestVar {
    public static void main(String[] args) {
        var x = 42;
        var s = "hello";
        var list = new ArrayList<String>();

        list.add("a");
        list.add("b");
        list.add("c");

        System.out.println(x);
        System.out.println(s);
        System.out.println(list.size());

        var map = new TreeMap<String, Integer>();
        map.put("one", 1);
        map.put("two", 2);

        for (var entry : map.entrySet()) {
            System.out.println(entry.getKey() + "=" + entry.getValue());
        }
    }
}
