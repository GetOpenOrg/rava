import java.util.ArrayList;
import java.util.EnumSet;
import java.util.List;

/**
 * 实参是类型变量擦除上界的视图、形参是类型变量（javac 对 `(E) x` 发射 checkcast 到 E 的
 * 擦除上界；JDK EnumSet.copyOf 的 `result.add((E) i.next())` 揭出 E0308）。
 * 覆盖：EnumSet.copyOf（多元素 / 单元素 / 非 EnumSet 集合）、用户泛型类 `E extends Number`
 * 与 `E extends Enum<E>` 的 `(E) obj` 传入 E 形参方法、之后按 E 取回的虚分派。
 */
public class TestTypeVarBoundArg {
    enum Day { MON, TUE, WED, THU }

    static class Holder<E extends Number> {
        final List<E> items = new ArrayList<>();
        void put(E e) { items.add(e); }
        @SuppressWarnings("unchecked")
        void putAll(List<Object> xs) {
            for (Object o : xs) put((E) o);
        }
        double sum() {
            double s = 0;
            for (E e : items) s += e.doubleValue();
            return s;
        }
    }

    static class EnumBag<E extends Enum<E>> {
        final List<E> items = new ArrayList<>();
        void add(E e) { items.add(e); }
        @SuppressWarnings("unchecked")
        void addRaw(Object o) { add((E) o); }
        String names() {
            StringBuilder sb = new StringBuilder();
            for (E e : items) sb.append(e.name()).append(':').append(e.ordinal()).append(' ');
            return sb.toString().trim();
        }
    }

    public static void main(String[] args) {
        List<Day> days = new ArrayList<>(List.of(Day.WED, Day.MON, Day.THU));
        System.out.println(EnumSet.copyOf(days));
        System.out.println(EnumSet.copyOf(List.of(Day.TUE)));
        System.out.println(EnumSet.copyOf(EnumSet.of(Day.THU, Day.MON)));

        Holder<Integer> h = new Holder<>();
        List<Object> raw = new ArrayList<>(List.of(1, 2, 3));
        h.putAll(raw);
        h.put(10);
        System.out.println(h.items + " " + h.sum());

        EnumBag<Day> bag = new EnumBag<>();
        bag.addRaw(Day.THU);
        bag.addRaw(Day.MON);
        bag.add(Day.TUE);
        System.out.println(bag.names());
    }
}
