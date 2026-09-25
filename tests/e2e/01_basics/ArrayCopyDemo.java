public class ArrayCopyDemo {
    public static void main(String[] args) {
        // 1. Basic int[] copy: copy 3 elements starting at index 1
        int[] src = {1, 2, 3, 4, 5};
        int[] dst = new int[5];
        System.arraycopy(src, 1, dst, 0, 3);
        for (int i = 0; i < dst.length; i++) {
            System.out.println(dst[i]);
        }

        // 2. String[] (object array) full copy
        String[] words = {"alpha", "beta", "gamma"};
        String[] copy = new String[3];
        System.arraycopy(words, 0, copy, 0, 3);
        for (int i = 0; i < copy.length; i++) {
            System.out.println(copy[i]);
        }

        // 3. Overlapping copy within the same int[] (shift left by 2)
        int[] arr = {10, 20, 30, 40, 50};
        System.arraycopy(arr, 2, arr, 0, 3);
        for (int i = 0; i < arr.length; i++) {
            System.out.println(arr[i]);
        }

        // 4. Zero-length copy is a no-op
        int[] a = {7, 8, 9};
        int[] b = {1, 2, 3};
        System.arraycopy(a, 0, b, 0, 0);
        for (int i = 0; i < b.length; i++) {
            System.out.println(b[i]);
        }
    }
}
