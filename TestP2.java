public class TestP2 {
    public static int sumArray(int[] arr) {
        int sum = 0;
        for (int i = 0; i < arr.length; i++) {
            sum += arr[i];
        }
        return sum;
    }

    public static void main(String[] args) {
        int[] arr = new int[5];
        arr[0] = 10; arr[1] = 20; arr[2] = 30; arr[3] = 40; arr[4] = 50;
        System.out.println(arr.length);      // 5
        System.out.println(sumArray(arr));   // 150
        System.out.println(arr[2]);          // 30

        int[] arr2 = {1, 2, 3, 4, 5};
        System.out.println(sumArray(arr2));  // 15
    }
}
