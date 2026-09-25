public class VarargsDemo {
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

    static void printAll(Object... items) {
        for (Object o : items) {
            System.out.println(o);
        }
    }

    public static void main(String[] args) {
        System.out.println(sum(1, 2, 3));           // 6
        System.out.println(sum(10, 20));             // 30
        System.out.println(join("-", "a", "b", "c")); // a-b-c
        printAll("hello", 42, true);                // hello / 42 / true
    }
}
