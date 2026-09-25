import java.util.Arrays;

public class ArrayOpsTest {
    public static void main(String[] args) {
        // Test 1: Array creation and access
        int[] arr = {5, 3, 1, 4, 2};
        System.out.println(arr.length);  // 5
        System.out.println(arr[0]);      // 5

        // Test 2: Arrays.sort
        Arrays.sort(arr);
        System.out.println(arr[0]);  // 1
        System.out.println(arr[4]);  // 5

        // Test 3: Arrays.fill
        int[] filled = new int[5];
        Arrays.fill(filled, 42);
        System.out.println(filled[0]);  // 42
        System.out.println(filled[4]);  // 42

        // Test 4: Array copy
        int[] copy = Arrays.copyOf(arr, 3);
        System.out.println(copy.length);  // 3
        System.out.println(copy[0]);      // 1
        System.out.println(copy[2]);      // 3

        // Test 5: 2D array
        int[][] matrix = new int[3][3];
        for (int i = 0; i < 3; i++) {
            for (int j = 0; j < 3; j++) {
                matrix[i][j] = i * 3 + j + 1;
            }
        }
        System.out.println(matrix[0][0]);  // 1
        System.out.println(matrix[1][1]);  // 5
        System.out.println(matrix[2][2]);  // 9

        // Test 6: String array
        String[] names = {"Charlie", "Alice", "Bob"};
        Arrays.sort(names);
        System.out.println(names[0]);  // Alice
        System.out.println(names[1]);  // Bob
        System.out.println(names[2]);  // Charlie

        // Test 7: Array sum
        int[] nums = {10, 20, 30, 40, 50};
        int sum = 0;
        for (int n : nums) {
            sum += n;
        }
        System.out.println(sum);  // 150

        // Test 8: Find max
        int max = nums[0];
        for (int i = 1; i < nums.length; i++) {
            if (nums[i] > max) {
                max = nums[i];
            }
        }
        System.out.println(max);  // 50

        // Test 9: Array as method parameter
        System.out.println(average(nums));  // 30.0

        // Test 10: Arrays.binarySearch (on sorted array)
        int[] sorted = {10, 20, 30, 40, 50};
        System.out.println(Arrays.binarySearch(sorted, 30));  // 2
    }

    static double average(int[] arr) {
        int sum = 0;
        for (int n : arr) {
            sum += n;
        }
        return (double) sum / arr.length;
    }
}
