public class TestVarargs {

    static int sum(int... nums) {
        int total = 0;
        for (int n : nums) total += n;
        return total;
    }

    static String join(String sep, String... parts) {
        StringBuilder sb = new StringBuilder();
        for (int i = 0; i < parts.length; i++) {
            if (i > 0) sb.append(sep);
            sb.append(parts[i]);
        }
        return sb.toString();
    }

    static double max(double first, double... rest) {
        double m = first;
        for (double v : rest) {
            if (v > m) m = v;
        }
        return m;
    }

    static int count(Object... items) {
        return items.length;
    }

    public static void main(String[] args) {
        System.out.println(sum());            // 0
        System.out.println(sum(1));           // 1
        System.out.println(sum(1, 2, 3));     // 6
        System.out.println(sum(10, 20, 30, 40)); // 100

        System.out.println(join(", ", "a", "b", "c")); // a, b, c
        System.out.println(join("-", "x"));            // x
        System.out.println(join("|"));                 // (empty)

        System.out.println(max(3.0));             // 3.0
        System.out.println(max(1.0, 5.0, 2.0));  // 5.0

        System.out.println(count("hello", 42, true)); // 3

        // varargs with array
        int[] arr = {4, 5, 6};
        System.out.println(sum(arr)); // 15
    }
}
