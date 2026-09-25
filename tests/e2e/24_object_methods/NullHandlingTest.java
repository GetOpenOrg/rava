public class NullHandlingTest {
    static class Node {
        String value;
        Node(String value) {
            this.value = value;
        }
        String getValue() { return value; }
    }

    // Method accepting nullable parameter
    static String safeToString(Object obj) {
        if (obj == null) {
            return "null";
        }
        return obj.toString();
    }

    // Method returning Object or null
    static Object findItem(java.util.ArrayList<Object> list, int index) {
        if (index < 0 || index >= list.size()) {
            return null;
        }
        return list.get(index);
    }

    // Method with null-returning path
    static Object getNullable(boolean returnNull) {
        if (returnNull) {
            return null;
        }
        return "not null";
    }

    public static void main(String[] args) {
        // Test 1: Null from method call
        Object obj = getNullable(true);
        System.out.println(obj == null);       // true
        obj = getNullable(false);
        System.out.println(obj == null);       // false
        System.out.println(obj);               // not null

        // Test 2: Null parameter passing via Object
        System.out.println(safeToString(null));      // null
        System.out.println(safeToString("world"));   // world
        System.out.println(safeToString(42));         // 42

        // Test 3: Null in collections
        java.util.ArrayList<String> items = new java.util.ArrayList<>();
        items.add("first");
        items.add(null);
        items.add("third");
        System.out.println(items.size());    // 3
        System.out.println(items.get(0));    // first
        System.out.println(items.get(1) == null); // true
        System.out.println(items.get(2));    // third

        // Test 4: Optional.ofNullable
        java.util.Optional<String> empty = java.util.Optional.ofNullable(null);
        java.util.Optional<String> present = java.util.Optional.ofNullable("value");
        System.out.println(empty.isPresent());   // false
        System.out.println(present.isPresent()); // true
        System.out.println(present.get());       // value
        System.out.println(empty.orElse("fallback")); // fallback

        // Test 5: Object field access
        Node node = new Node("only");
        System.out.println(node.value);       // only
        System.out.println(node.getValue());  // only

        // Test 6: Null from collection lookup
        java.util.ArrayList<Object> data = new java.util.ArrayList<>();
        data.add("alpha");
        data.add("beta");
        Object found = findItem(data, 0);
        System.out.println(found);            // alpha
        Object notFound = findItem(data, 99);
        System.out.println(notFound == null); // true

        // Test 7: Null in conditional from method
        Object val = findItem(data, 99);
        String result;
        if (val != null) {
            result = val.toString();
        } else {
            result = "default";
        }
        System.out.println(result);          // default

        // Test 8: HashMap with null values
        java.util.HashMap<String, String> map = new java.util.HashMap<>();
        map.put("key1", "value1");
        map.put("key2", null);
        System.out.println(map.get("key1"));         // value1
        System.out.println(map.containsKey("key2")); // true
        System.out.println(map.size());              // 2
    }
}
