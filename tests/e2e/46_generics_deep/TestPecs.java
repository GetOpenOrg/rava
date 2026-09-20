import java.util.*;
import java.util.function.Function;

public class TestPecs {
    static <T extends Comparable<? super T>> T max(List<T> list) {
        T best = list.get(0);
        for (T t : list) {
            if (t.compareTo(best) > 0) best = t;
        }
        return best;
    }

    static void copy(List<? super Integer> dest, List<Integer> src) {
        dest.addAll(src);
    }

    static <T> void print(List<? extends Number> list) {
        for (Number n : list) System.out.println("n=" + n);
    }

    public static void main(String[] args) {
        List<Integer> ints = List.of(3, 9, 1, 7);
        System.out.println("max=" + max(ints));
        List<Number> dest = new ArrayList<>();
        copy(dest, ints);
        System.out.println("dest=" + dest);
        List<Double> dbls = List.of(1.5, 2.5);
        print(dbls);
        System.out.println("maxStr=" + max(List.of("c", "a", "b")));
    }
}
