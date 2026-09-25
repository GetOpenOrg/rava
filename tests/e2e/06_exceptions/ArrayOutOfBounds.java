public class ArrayOutOfBounds {
    public static void main(String[] args) {
        int[] arr = {1, 2, 3};

        // Test 1: out-of-bounds read (index too large)
        try {
            int x = arr[5];
            System.out.println("Should not reach here");
        } catch (ArrayIndexOutOfBoundsException e) {
            System.out.println("Caught: " + e.getMessage());
        }

        // Test 2: out-of-bounds write (negative index)
        try {
            arr[-1] = 99;
            System.out.println("Should not reach here");
        } catch (ArrayIndexOutOfBoundsException e) {
            System.out.println("Caught negative: " + e.getMessage());
        }

        System.out.println("Done");
    }
}
