public class NullStringTest {
    public static void main(String[] args) {
        // Test 1: String initialized to null, then assigned
        String s = null;
        System.out.println(s == null);  // true
        s = "world";
        System.out.println(s == null);  // false
        System.out.println(s);          // world

        // Test 2: Object initialized to null, then assigned
        Object obj = null;
        System.out.println(obj == null);  // true
        obj = "hello";
        System.out.println(obj == null);  // false
        System.out.println(obj);          // hello

        // Test 3: Multiple null reassignments
        String x = "start";
        System.out.println(x == null);  // false
        x = null;
        System.out.println(x == null);  // true
        x = "end";
        System.out.println(x);         // end

        // Test 4: Null via method parameter (Object param can be null)
        System.out.println(isNull(null));     // true
        System.out.println(isNull("abc"));    // false

        // Test 5: Null in if-else branches
        String result;
        Object val = null;
        if (val != null) {
            result = val.toString();
        } else {
            result = "default";
        }
        System.out.println(result);  // default

        // Test 6: Method returning null
        String fromMethod = getValue(true);
        System.out.println(fromMethod == null);  // true
        String fromMethod2 = getValue(false);
        System.out.println(fromMethod2 == null);  // false
        System.out.println(fromMethod2);           // hello

        // Test 7: Null return used in if-else
        String r = getValue(true);
        if (r != null) {
            System.out.println(r);
        } else {
            System.out.println("was null");  // was null
        }
    }

    static boolean isNull(Object o) {
        return o == null;
    }

    static String getValue(boolean returnNull) {
        if (returnNull) {
            return null;
        }
        return "hello";
    }
}
