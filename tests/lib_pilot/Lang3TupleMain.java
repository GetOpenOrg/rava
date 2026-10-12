
import org.apache.commons.lang3.tuple.ImmutablePair;
import org.apache.commons.lang3.tuple.MutablePair;
import org.apache.commons.lang3.tuple.Pair;
import org.apache.commons.lang3.tuple.Triple;

/**
 * commons-lang3 pilot A14：Pair/Triple 值语义（W1 #4）。
 */
public class Lang3TupleMain {

    static void check(String name, Object expected, Object actual) {
        boolean ok = String.valueOf(expected).equals(String.valueOf(actual));
        System.out.println((ok ? "PASS " : "FAIL ") + name
                + (ok ? "" : " :: expected=<" + expected + "> actual=<" + actual + ">"));
    }

    public static void main(String[] args) {
        Pair<String, Integer> a = Pair.of("left", 1);
        Pair<String, Integer> b = ImmutablePair.of("left", 1);
        Pair<String, Integer> c = Pair.of("right", 1);

        check("accessors", "left/1", a.getLeft() + "/" + a.getRight());
        check("value-semantics", true, a.equals(b));
        check("different", false, a.equals(c));
        check("hashcode-equal", true, a.hashCode() == b.hashCode());
        check("tostring", "(left,1)", a.toString());
        check("compare", -1, Integer.signum(a.compareTo(c)));

        // Mutable 可变、Immutable 拒绝
        MutablePair<String, Integer> m = MutablePair.of("x", 1);
        m.setLeft("y");
        m.setRight(2);
        check("mutable-set", "y/2", m.getLeft() + "/" + m.right);
        try {
            b.setValue(9); // Pair 实现了 Map.Entry；ImmutablePair 的 setValue 恒抛异常
            System.out.println("FAIL immutable-reject");
        } catch (UnsupportedOperationException e) {
            System.out.println("PASS immutable-reject");
        }

        // Triple
        Triple<String, Integer, Boolean> t = Triple.of("mid", 7, true);
        check("triple-accessors", "mid/7/true", t.getLeft() + "/" + t.getMiddle() + "/" + t.getRight());
        check("triple-equals", true, t.equals(Triple.of("mid", 7, true)));
        check("triple-tostring", "(mid,7,true)", t.toString());

        // pair→map entry 语义
        check("map-entry", "left=1", a.getKey() + "=" + a.getValue());
        System.out.println("DONE Lang3TupleMain");
    }
}
