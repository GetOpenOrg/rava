import java.util.ArrayList;
import java.util.HashMap;
import java.util.Map;

/**
 * Tests static initializer blocks and static field initialization.
 */
public class StaticInitTest {

    static int counter = 10;
    static String label;
    static ArrayList<String> items;
    static Map<String, Integer> lookup;

    // Static initializer block
    static {
        label = "initialized";
        items = new ArrayList<>();
        items.add("alpha");
        items.add("beta");
        items.add("gamma");

        lookup = new HashMap<>();
        lookup.put("x", 1);
        lookup.put("y", 2);
        lookup.put("z", 3);
    }

    // Second static initializer block
    static {
        counter += 5;
    }

    // Inner class with its own static initializer
    static class Config {
        static String mode = "default";
        static int version;

        static {
            version = 42;
        }

        static String describe() {
            return mode + "-v" + version;
        }
    }

    public static void main(String[] args) {
        // Test 1: Static field initialization
        System.out.println("counter: " + counter);
        System.out.println("label: " + label);

        // Test 2: Static initializer with collections
        System.out.println("items: " + items);
        System.out.println("items size: " + items.size());

        // Test 3: Static HashMap
        System.out.println("lookup x: " + lookup.get("x"));
        System.out.println("lookup y: " + lookup.get("y"));
        System.out.println("lookup z: " + lookup.get("z"));

        // Test 4: Inner class static initializer
        System.out.println("config: " + Config.describe());

        // Test 5: Verify static state persists
        counter += 100;
        System.out.println("counter after: " + counter);
    }
}
