public class TestBoundedGenerics {
    static <T extends Number> double sum(T[] arr) {
        double total = 0;
        for (T x : arr) {
            total += x.doubleValue();
        }
        return total;
    }

    static <T extends Comparable<T>> boolean isSorted(T[] arr) {
        for (int i = 1; i < arr.length; i++) {
            if (arr[i - 1].compareTo(arr[i]) > 0) {
                return false;
            }
        }
        return true;
    }

    public static void main(String[] args) {
        Integer[] ints = {1, 2, 3, 4, 5};
        System.out.println(sum(ints));

        Double[] doubles = {1.5, 2.5, 3.0};
        System.out.println(sum(doubles));

        String[] sorted = {"a", "b", "c"};
        System.out.println(isSorted(sorted));

        String[] unsorted = {"b", "a", "c"};
        System.out.println(isSorted(unsorted));
    }
}
