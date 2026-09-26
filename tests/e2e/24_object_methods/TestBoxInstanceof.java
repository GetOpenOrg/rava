// FS-M4：基本类型装箱进 Object 后的 instanceof / getClass / Comparable 语义。
import java.util.*;

public class TestBoxInstanceof {
    static String kinds(Object o) {
        return (o instanceof Number) + "," + (o instanceof Comparable) + "," + (o instanceof java.io.Serializable)
            + "," + o.getClass().getSimpleName();
    }
    @SuppressWarnings({"unchecked", "rawtypes"})
    static int cmp(Object a, Object b) { return ((Comparable) a).compareTo(b); }

    public static void main(String[] args) {
        Object[] vals = {5, 5L, 2.5, 2.5f, (short) 3, (byte) 4, 'c', true};
        for (Object v : vals) System.out.println(v + " " + kinds(v));
        Number n = 7;
        System.out.println("number " + n.intValue() + " " + n.doubleValue() + " " + (n instanceof Integer));
        System.out.println("cmp " + cmp(3, 9) + " " + cmp(2.5, 1.0) + " " + cmp('b', 'a') + " " + cmp(true, false));
        List<Object> mixed = new ArrayList<>(List.of(3, 1, 2));
        mixed.sort(null);
        System.out.println("sorted " + mixed);
        Map<Object, String> m = new HashMap<>();
        m.put(1, "int"); m.put(1L, "long");
        System.out.println("map " + m.get(1) + " " + m.get(1L) + " " + m.size());
        Object d = 1.0;
        System.out.println("switch " + switch (d) { case Integer i -> "I" + i; case Double x -> "D" + x; default -> "?"; });
    }
}
