import java.util.Arrays;

public class TestArraysUtil {
    public static void main(String[] args) {
        int[] arr = {5, 3, 1, 4, 2};
        Arrays.sort(arr);
        System.out.println(Arrays.toString(arr));

        int idx = Arrays.binarySearch(arr, 3);
        System.out.println(idx);

        int[] copy = Arrays.copyOf(arr, 3);
        System.out.println(Arrays.toString(copy));

        int[] range = Arrays.copyOfRange(arr, 1, 4);
        System.out.println(Arrays.toString(range));

        int[] filled = new int[5];
        Arrays.fill(filled, 7);
        System.out.println(Arrays.toString(filled));

        System.out.println(Arrays.equals(arr, new int[]{1, 2, 3, 4, 5}));

        String[] words = {"banana", "apple", "cherry"};
        Arrays.sort(words);
        System.out.println(Arrays.toString(words));
    }
}
