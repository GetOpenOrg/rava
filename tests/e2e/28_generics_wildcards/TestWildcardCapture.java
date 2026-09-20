import java.util.ArrayList;
import java.util.List;

public class TestWildcardCapture {

    // <? extends Number> 只能读，不能写
    static double sumOf(List<? extends Number> nums) {
        double total = 0.0;
        for (Number n : nums) {
            total += n.doubleValue();
        }
        return total;
    }

    // <? super Integer> 可以写 Integer 及其子类型
    static void addInts(List<? super Integer> dst, int count) {
        for (int i = 0; i < count; i++) {
            dst.add(i);
        }
    }

    // 通配符捕获：helper 方法把 ? 捕获成具名类型变量 T
    static <T> void swapHelper(List<T> list, int i, int j) {
        T tmp = list.get(i);
        list.set(i, list.get(j));
        list.set(j, tmp);
    }

    static void swap(List<?> list, int i, int j) {
        swapHelper(list, i, j);
    }

    // reverse 也依赖捕获
    static void reverse(List<?> list) {
        reverseHelper(list);
    }

    static <T> void reverseHelper(List<T> list) {
        List<T> reversed = new ArrayList<>();
        for (int i = list.size() - 1; i >= 0; i--) {
            reversed.add(list.get(i));
        }
        for (int i = 0; i < reversed.size(); i++) {
            list.set(i, reversed.get(i));
        }
    }

    static int firstInt(List<? extends Number> nums) {
        Number n = nums.get(0);
        return n.intValue();
    }

    static <T extends Comparable<T>> T maxOf(List<? extends T> list) {
        T best = list.get(0);
        for (T t : list) {
            if (t.compareTo(best) > 0) {
                best = t;
            }
        }
        return best;
    }

    public static void main(String[] args) {
        List<Integer> ints = new ArrayList<>();
        ints.add(1);
        ints.add(2);
        ints.add(3);

        List<Double> doubles = new ArrayList<>();
        doubles.add(1.5);
        doubles.add(2.5);

        System.out.println("sum ints=" + sumOf(ints));
        System.out.println("sum doubles=" + sumOf(doubles));
        System.out.println("first int=" + firstInt(ints));

        List<Number> numbers = new ArrayList<>();
        addInts(numbers, 3);
        System.out.println("numbers=" + numbers);

        List<Object> objects = new ArrayList<>();
        addInts(objects, 2);
        System.out.println("objects=" + objects);

        swap(ints, 0, 2);
        System.out.println("swapped ints=" + ints);

        reverse(ints);
        System.out.println("reversed ints=" + ints);

        reverse(doubles);
        System.out.println("reversed doubles=" + doubles);

        List<String> words = new ArrayList<>();
        words.add("pear");
        words.add("apple");
        words.add("zebra");
        System.out.println("max word=" + maxOf(words));

        System.out.println("max int=" + maxOf(ints));

        // 嵌套通配符
        List<List<? extends Number>> nested = new ArrayList<>();
        nested.add(ints);
        nested.add(doubles);
        System.out.println("nested size=" + nested.size() + " first sum=" + sumOf(nested.get(0)));

        System.out.println("done");
    }
}
