public class VarargsTest {

    // 自定义 varargs 方法: int...
    static int sum(int... numbers) {
        int total = 0;
        for (int n : numbers) {
            total += n;
        }
        return total;
    }

    // 自定义 varargs 方法: String...
    static String joinStrings(String... parts) {
        StringBuilder sb = new StringBuilder();
        for (int i = 0; i < parts.length; i++) {
            if (i > 0) sb.append(", ");
            sb.append(parts[i]);
        }
        return sb.toString();
    }

    // varargs 与普通参数混合
    static String prefixed(String prefix, int... values) {
        StringBuilder sb = new StringBuilder(prefix);
        sb.append(": ");
        for (int i = 0; i < values.length; i++) {
            if (i > 0) sb.append(", ");
            sb.append(values[i]);
        }
        return sb.toString();
    }

    // 自定义 varargs 方法: Object...
    static void printAll(Object... items) {
        for (Object item : items) {
            System.out.println("  " + item);
        }
    }

    public static void main(String[] args) {
        // 1. int... varargs
        System.out.println("sum() = " + sum());
        System.out.println("sum(1, 2, 3) = " + sum(1, 2, 3));
        System.out.println("sum(10, 20) = " + sum(10, 20));

        // 2. String... varargs
        System.out.println("join() = " + joinStrings());
        System.out.println("join(a, b, c) = " + joinStrings("a", "b", "c"));

        // 3. 混合参数 varargs
        System.out.println(prefixed("Numbers", 1, 2, 3));

        // 4. Object... varargs (autoboxing)
        System.out.println("Object varargs:");
        printAll(42, "hello", 3.14);

        // 5. String.format
        System.out.println(String.format("Name: %s, Age: %d", "Alice", 30));
        System.out.println(String.format("Pi = %.2f", 3.14159));
        System.out.println(String.format("100%%"));
    }
}
