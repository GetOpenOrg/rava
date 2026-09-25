import java.util.ArrayList;
import java.util.List;

public class MultiClassOopTest {

    // Test: polymorphism via ArrayList<Object> + instanceof + casting
    public static void main(String[] args) {
        // 1. ArrayList holding mixed types via autoboxing
        List<Object> items = new ArrayList<>();
        items.add(42);
        items.add("hello");
        items.add(3.14);
        items.add(true);

        for (Object item : items) {
            if (item instanceof Integer) {
                System.out.println("int: " + item);
            } else if (item instanceof String) {
                System.out.println("str: " + item);
            } else if (item instanceof Double) {
                System.out.println("dbl: " + item);
            } else if (item instanceof Boolean) {
                System.out.println("bool: " + item);
            }
        }

        // 2. String.format with multiple types
        String msg = String.format("count=%d, name=%s, pi=%.2f", 10, "test", 3.14159);
        System.out.println(msg);

        // 3. Nested collections
        List<List<Integer>> matrix = new ArrayList<>();
        for (int i = 0; i < 3; i++) {
            List<Integer> row = new ArrayList<>();
            for (int j = 0; j < 3; j++) {
                row.add(i * 3 + j + 1);
            }
            matrix.add(row);
        }
        System.out.println("matrix: " + matrix);

        // 4. List operations chain
        List<Integer> nums = new ArrayList<>();
        for (int i = 1; i <= 10; i++) nums.add(i);
        int sum = 0;
        for (int n : nums) {
            if (n % 2 == 0) sum += n;
        }
        System.out.println("even sum 1-10: " + sum);

        // 5. String manipulation chain
        String s = "  Hello, World!  ";
        String result = s.trim().toLowerCase().replace("hello", "hi").replace("world", "earth");
        System.out.println("chain: " + result);

        // 6. StringBuilder in loop
        StringBuilder sb = new StringBuilder();
        for (int i = 0; i < 5; i++) {
            if (i > 0) sb.append("-");
            sb.append(i * 10);
        }
        System.out.println("sb: " + sb.toString());

        // 7. Math operations
        System.out.println("max: " + Math.max(10, 20));
        System.out.println("min: " + Math.min(10, 20));
        System.out.println("abs: " + Math.abs(-42));
        System.out.println("pow: " + (int) Math.pow(2, 10));

        // 8. Array and list interop
        int[] arr = {5, 3, 1, 4, 2};
        java.util.Arrays.sort(arr);
        StringBuilder arrStr = new StringBuilder("[");
        for (int i = 0; i < arr.length; i++) {
            if (i > 0) arrStr.append(", ");
            arrStr.append(arr[i]);
        }
        arrStr.append("]");
        System.out.println("sorted arr: " + arrStr.toString());

        System.out.println("Done.");
    }
}
