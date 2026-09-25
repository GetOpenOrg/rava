import java.util.ArrayList;
import java.util.HashMap;

public class CollectionTest {
    public static void testArrayList() {
        ArrayList<String> items = new ArrayList<>();
        items.add("first");
        items.add("second");
        items.add("third");

        for (String item : items) {
            System.out.println(item);
        }
    }

    public static void testHashMap() {
        HashMap<String, String> capitals = new HashMap<>();
        capitals.put("France", "Paris");
        capitals.put("Japan", "Tokyo");
        capitals.put("Germany", "Berlin");

        System.out.println(capitals.get("France"));
        System.out.println(capitals.get("Japan"));
        System.out.println(capitals.size());
    }

    public static void main(String[] args) {
        testArrayList();
        testHashMap();
        System.out.println("Done");
    }
}
