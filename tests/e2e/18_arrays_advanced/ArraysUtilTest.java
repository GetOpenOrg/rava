import java.util.Arrays;

public class ArraysUtilTest {
    public static void main(String[] args) {
        // Arrays.sort(int[])
        int[] nums = {5, 3, 1, 4, 2};
        Arrays.sort(nums);
        System.out.println("sorted: " + Arrays.toString(nums));

        // Arrays.sort(int[], from, to)
        int[] partial = {9, 7, 5, 3, 1};
        Arrays.sort(partial, 1, 4);
        System.out.println("partial sorted: " + Arrays.toString(partial));

        // Arrays.copyOf
        int[] original = {10, 20, 30};
        int[] longer = Arrays.copyOf(original, 5);
        System.out.println("copyOf longer: " + Arrays.toString(longer));
        int[] shorter = Arrays.copyOf(original, 2);
        System.out.println("copyOf shorter: " + Arrays.toString(shorter));

        // Arrays.copyOfRange
        int[] range = Arrays.copyOfRange(original, 1, 3);
        System.out.println("copyOfRange: " + Arrays.toString(range));

        // Arrays.fill
        int[] filled = new int[4];
        Arrays.fill(filled, 7);
        System.out.println("fill: " + Arrays.toString(filled));

        // Arrays.equals
        int[] a = {1, 2, 3};
        int[] b = {1, 2, 3};
        int[] c = {1, 2, 4};
        System.out.println("equals same: " + Arrays.equals(a, b));
        System.out.println("equals diff: " + Arrays.equals(a, c));

        // Arrays.sort(double[])
        double[] doubles = {3.14, 1.41, 2.72};
        Arrays.sort(doubles);
        System.out.println("sorted doubles: " + Arrays.toString(doubles));

        // Arrays.toString(String[])
        String[] names = {"Charlie", "Alice", "Bob"};
        Arrays.sort(names);
        System.out.println("sorted names: " + Arrays.toString(names));
    }
}
