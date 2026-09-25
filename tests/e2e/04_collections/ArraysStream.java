import java.util.Arrays;
import java.util.stream.IntStream;

public class ArraysStream {
    public static void main(String[] args) {
        // for-each on int array
        int[] nums = {3, 1, 4, 1, 5};
        int sum = 0;
        for (int n : nums) {
            sum += n;
        }
        System.out.println(sum); // 14

        // Arrays.sort and for-each
        int[] sorted = {5, 2, 8, 1, 9};
        Arrays.sort(sorted);
        System.out.println(sorted[0]); // 1
        System.out.println(sorted[4]); // 9

        // Arrays.stream on int array -> sum
        int[] data = {10, 20, 30, 40};
        int streamSum = Arrays.stream(data).sum();
        System.out.println(streamSum); // 100

        // Arrays.stream on String array
        String[] words = {"hello", "world", "java"};
        long count = Arrays.stream(words).filter(s -> s.length() > 4).count();
        System.out.println(count); // 2 (hello=5, world=5)
    }
}
