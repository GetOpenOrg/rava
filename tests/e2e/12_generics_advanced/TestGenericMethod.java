public class TestGenericMethod {
    static <T extends Comparable<T>> T max(T a, T b) {
        return a.compareTo(b) >= 0 ? a : b;
    }

    static <T> void swap(T[] arr, int i, int j) {
        T tmp = arr[i];
        arr[i] = arr[j];
        arr[j] = tmp;
    }

    static <T> T identity(T value) {
        return value;
    }

    public static void main(String[] args) {
        System.out.println(max(3, 7));
        System.out.println(max("apple", "banana"));

        Integer[] arr = {1, 2, 3, 4, 5};
        swap(arr, 0, 4);
        for (int x : arr) {
            System.out.print(x + " ");
        }
        System.out.println();

        System.out.println(identity("hello"));
        System.out.println(identity(42));
    }
}
