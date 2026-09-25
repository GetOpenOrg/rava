import java.lang.reflect.Method;
import java.util.ArrayList;
import java.util.List;
import java.util.Map;
import java.util.TreeMap;

/**
 * 反射 Method.invoke 的实参 / 返回值形态（L3 分派，TestRecordComponents 揭出）：char 返回装箱为
 * Character、char / byte / short 形参、基本类型拓宽（Byte / Short / Character → int 形参）、
 * 数组形参与返回、泛型接口载体形参与返回（List / Map）、静态与实例、void、错型实参 →
 * IllegalArgumentException。
 */
public class TestReflectInvokeShapes {
    static class Target {
        char initial(String s) { return s.charAt(0); }
        int code(char c) { return c + 1; }
        short twice(short s) { return (short) (s * 2); }
        byte neg(byte b) { return (byte) -b; }
        int widen(int x) { return x * 10; }
        int sum(int[] xs) { int t = 0; for (int x : xs) t += x; return t; }
        String[] split(String s) { return s.split(","); }
        List<String> tags(int n) { List<String> l = new ArrayList<>(); for (int i = 0; i < n; i++) l.add("t" + i); return l; }
        int size(List<String> l) { return l.size(); }
        Map<String, Integer> index(List<String> l) { Map<String, Integer> m = new TreeMap<>(); for (String s : l) m.put(s, s.length()); return m; }
        static long square(long v) { return v * v; }
        void touch(StringBuilder sb) { sb.append("touched"); }
    }

    static Method m(String name, Class<?>... p) throws Exception {
        return Target.class.getDeclaredMethod(name, p);
    }

    public static void main(String[] args) throws Exception {
        Target t = new Target();
        Object c = m("initial", String.class).invoke(t, "zeta");
        System.out.println("char ret: " + c + " " + (c instanceof Character) + " " + c.getClass().getSimpleName());
        System.out.println("char arg: " + m("code", char.class).invoke(t, 'A'));
        System.out.println("short: " + m("twice", short.class).invoke(t, (short) 21));
        System.out.println("byte: " + m("neg", byte.class).invoke(t, (byte) 7));
        System.out.println("widen Byte->int: " + m("widen", int.class).invoke(t, (byte) 3));
        System.out.println("widen Short->int: " + m("widen", int.class).invoke(t, (short) 4));
        System.out.println("widen Character->int: " + m("widen", int.class).invoke(t, 'a'));
        System.out.println("int[] arg: " + m("sum", int[].class).invoke(t, (Object) new int[] {1, 2, 3, 4}));
        String[] parts = (String[]) m("split", String.class).invoke(t, "a,b,c");
        System.out.println("String[] ret: " + parts.length + " " + parts[2]);
        @SuppressWarnings("unchecked")
        List<String> tags = (List<String>) m("tags", int.class).invoke(t, 3);
        System.out.println("List ret: " + tags);
        System.out.println("List arg: " + m("size", List.class).invoke(t, List.of("x", "y")));
        System.out.println("Map ret: " + m("index", List.class).invoke(t, List.of("aa", "b")));
        System.out.println("static: " + m("square", long.class).invoke(null, 12L));
        StringBuilder sb = new StringBuilder();
        Object v = m("touch", StringBuilder.class).invoke(t, sb);
        System.out.println("void: " + v + " " + sb);
        try {
            m("code", char.class).invoke(t, 65);
        } catch (IllegalArgumentException e) {
            System.out.println("IAE for Integer->char");
        }
    }
}
