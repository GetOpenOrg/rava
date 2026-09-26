// FS-M8：类型变量操作数的 null 判定（ifnull / ifnonnull）在 T 实例化为 String / Integer /
// 用户类 / 数组时与 JVM 一致。
import java.util.*;

public class TestGenericNullCheck {
    static class Box<T> {
        T val;
        Box(T v) { val = v; }
        boolean empty() { return val == null; }
        T orElse(T other) { return val != null ? val : other; }
    }
    static class Pt { final int x; Pt(int x) { this.x = x; } public String toString() { return "Pt" + x; } }

    static <T> boolean isNull(T t) { return t == null; }
    static <T> String describe(T t) { if (t == null) return "null!"; return "val:" + t; }
    static <K, V> int countNullValues(Map<K, V> m) {
        int n = 0;
        for (V v : m.values()) if (v == null) n++;
        return n;
    }
    static <E> E firstNonNull(List<E> xs) {
        for (E e : xs) if (e != null) return e;
        return null;
    }

    public static void main(String[] args) {
        Box<String> s1 = new Box<>(null), s2 = new Box<>("hi");
        Box<Integer> i1 = new Box<>(null), i2 = new Box<>(7);
        Box<Pt> p1 = new Box<>(null);
        Box<int[]> a1 = new Box<>(null), a2 = new Box<>(new int[]{1});
        System.out.println("string " + s1.empty() + " " + s2.empty() + " " + s1.orElse("dflt") + " " + s2.orElse("dflt"));
        System.out.println("integer " + i1.empty() + " " + i2.empty() + " " + i1.orElse(-1));
        System.out.println("user " + p1.empty() + " " + p1.orElse(new Pt(3)));
        System.out.println("array " + a1.empty() + " " + a2.empty());
        String ns = null;
        Integer ni = null;
        System.out.println("isNull " + isNull(ns) + " " + isNull("x") + " " + isNull(ni) + " " + isNull(5) + " " + isNull(new Pt(1)));
        System.out.println("describe " + describe(ns) + " " + describe("x") + " " + describe(ni) + " " + describe(new Pt(2)));
        Map<String, String> m = new HashMap<>();
        m.put("a", null); m.put("b", "B"); m.put("c", null);
        System.out.println("nullValues " + countNullValues(m));
        System.out.println("firstNonNull " + firstNonNull(Arrays.asList(null, null, "z", "w")) + " " + firstNonNull(Arrays.asList((String) null)));
        Optional<String> opt = Optional.ofNullable(ns);
        System.out.println("optional " + opt.isPresent() + " " + opt.orElse("none") + " " + Objects.requireNonNullElse(ns, "else"));
    }
}
