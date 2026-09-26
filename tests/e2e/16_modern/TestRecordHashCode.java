import java.util.HashSet;
import java.util.Set;

/**
 * record hashCode（ObjectMethods 31 多项式，S-7）：覆盖每种分量类型的 hash 分支——
 * int / short / byte / char / boolean(true/false) / long(负值) / double(NaN、-0.0) /
 * float(NaN) / String / null 引用 / 嵌套 record / 泛型分量 / 零分量 record；
 * 以及 equals 一致的记录在 HashSet 中去重。
 */
public class TestRecordHashCode {
    record Ints(int i, short s, byte b, char c) {}
    record Flags(boolean t, boolean f) {}
    record Wide(long l, double d, float f) {}
    record Named(String name, Object extra) {}
    record Outer(Named inner, int n) {}
    record Box<T>(T value) {}
    record Empty() {}

    public static void main(String[] args) {
        System.out.println("Ints " + new Ints(7, (short) -3, (byte) 100, 'Z').hashCode());
        System.out.println("Flags " + new Flags(true, false).hashCode());
        System.out.println("Wide " + new Wide(-123456789012345L, 3.25, 1.5f).hashCode());
        System.out.println("Wide NaN " + new Wide(Long.MIN_VALUE, Double.NaN, Float.NaN).hashCode());
        System.out.println("Wide -0.0 " + new Wide(0L, -0.0, -0.0f).hashCode());
        System.out.println("Named " + new Named("alice", 42).hashCode());
        System.out.println("Named null " + new Named(null, null).hashCode());
        System.out.println("Outer " + new Outer(new Named("bob", "x"), 5).hashCode());
        System.out.println("Box " + new Box<>("v").hashCode() + " " + new Box<>(99).hashCode());
        System.out.println("Empty " + new Empty().hashCode());

        Set<Named> set = new HashSet<>();
        set.add(new Named("a", 1));
        set.add(new Named("a", 1));
        set.add(new Named("b", 1));
        System.out.println("set size " + set.size() + " contains " + set.contains(new Named("b", 1)));
    }
}
