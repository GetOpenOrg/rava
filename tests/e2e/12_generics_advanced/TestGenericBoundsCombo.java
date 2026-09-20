import java.util.ArrayList;
import java.util.List;

interface Measurable {
    double size();
}

class Length implements Measurable, Comparable<Length> {
    final int value;

    Length(int value) {
        this.value = value;
    }

    @Override
    public double size() {
        return value;
    }

    @Override
    public int compareTo(Length other) {
        return Integer.compare(value, other.value);
    }

    @Override
    public String toString() {
        return "L" + value;
    }
}

// 多边界：T 必须同时实现两个接口
class Registry<T extends Measurable & Comparable<T>> {
    private final List<T> items = new ArrayList<>();

    void add(T item) {
        items.add(item);
    }

    T largest() {
        T best = items.get(0);
        for (T item : items) {
            if (item.compareTo(best) > 0) {
                best = item;
            }
        }
        return best;
    }

    int count() {
        return items.size();
    }
}

public class TestGenericBoundsCombo {

    // 递归边界：T 必须能和自身比较
    static <T extends Comparable<T>> T maxOf(List<T> list) {
        T best = list.get(0);
        for (T t : list) {
            if (t.compareTo(best) > 0) {
                best = t;
            }
        }
        return best;
    }

    // 多个边界参数
    static <T extends Comparable<T>, U extends Measurable> String pair(T t, U u) {
        return String.valueOf(t) + "/" + u.size();
    }

    // 边界中引用其他类型参数
    static <A extends Comparable<A>, B extends Comparable<B>> int compareCross(A a, B b) {
        return a.compareTo(a) + b.compareTo(b);
    }

    // 通配符 + 边界组合
    static double totalSize(List<? extends Measurable> items) {
        double sum = 0;
        for (Measurable m : items) {
            sum += m.size();
        }
        return sum;
    }

    // 交集类型的通用写法
    static <T extends Number & Comparable<T>> String describe(T value) {
        return value.intValue() + ":" + value.compareTo(value);
    }

    public static void main(String[] args) {
        Registry<Length> registry = new Registry<>();
        registry.add(new Length(3));
        registry.add(new Length(9));
        registry.add(new Length(5));
        System.out.println("count=" + registry.count() + " largest=" + registry.largest());

        List<String> words = new ArrayList<>();
        words.add("banana");
        words.add("apple");
        words.add("cherry");
        System.out.println("max word=" + maxOf(words));

        List<Integer> nums = new ArrayList<>();
        nums.add(4);
        nums.add(11);
        System.out.println("max num=" + maxOf(nums));

        System.out.println(pair("hello", new Length(7)));
        System.out.println("cross=" + compareCross("a", 5));

        List<Measurable> measures = new ArrayList<>();
        measures.add(new Length(2));
        measures.add(new Length(4));
        System.out.println("total=" + totalSize(measures));

        System.out.println(describe(Integer.valueOf(42)));
        System.out.println(describe(Double.valueOf(9.9)));

        // 泛型方法在同类中被不同实参多次调用（不同具体化的字节码碎片）
        System.out.println(maxOf(new ArrayList<>(List.of("x", "yz"))));
        System.out.println(maxOf(new ArrayList<>(List.of(1, 2, 3))));

        System.out.println("done");
    }
}
