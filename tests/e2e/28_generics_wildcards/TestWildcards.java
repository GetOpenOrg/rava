import java.util.ArrayList;
import java.util.List;

public class TestWildcards {

    // upper bounded wildcard: ? extends T — read-only consumer
    static double sumList(List<? extends Number> list) {
        double total = 0;
        for (Number n : list) total += n.doubleValue();
        return total;
    }

    // lower bounded wildcard: ? super T — write-only producer
    static void addNumbers(List<? super Integer> list, int from, int to) {
        for (int i = from; i <= to; i++) list.add(i);
    }

    // unbounded wildcard
    static void printAll(List<?> list) {
        StringBuilder sb = new StringBuilder();
        for (int i = 0; i < list.size(); i++) {
            if (i > 0) sb.append(" ");
            sb.append(list.get(i));
        }
        System.out.println(sb);
    }

    static <T extends Comparable<T>> T findMax(List<T> list) {
        T max = list.get(0);
        for (T item : list) {
            if (item.compareTo(max) > 0) max = item;
        }
        return max;
    }

    public static void main(String[] args) {
        List<Integer> ints = new ArrayList<>();
        ints.add(1); ints.add(2); ints.add(3);
        System.out.println(sumList(ints));   // 6.0

        List<Double> doubles = new ArrayList<>();
        doubles.add(1.5); doubles.add(2.5); doubles.add(3.0);
        System.out.println(sumList(doubles)); // 7.0

        List<Number> numbers = new ArrayList<>();
        addNumbers(numbers, 1, 5);
        printAll(numbers); // 1 2 3 4 5

        List<Object> objects = new ArrayList<>();
        addNumbers(objects, 10, 13);
        printAll(objects); // 10 11 12 13

        // unbounded wildcard
        List<String> strs = new ArrayList<>();
        strs.add("banana"); strs.add("apple"); strs.add("cherry");
        printAll(strs); // banana apple cherry

        // bounded type parameter
        System.out.println(findMax(ints));   // 3
        System.out.println(findMax(strs));   // cherry (lexicographic)

        // mixed list via upper bound
        List<Integer> mixedInts = new ArrayList<>();
        mixedInts.add(10); mixedInts.add(20); mixedInts.add(5);
        System.out.println(sumList(mixedInts)); // 35.0
    }
}
