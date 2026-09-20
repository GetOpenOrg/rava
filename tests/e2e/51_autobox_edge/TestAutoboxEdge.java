import java.util.List;

public class TestAutoboxEdge {
    public static void main(String[] args) {
        Integer a = 127, b = 127;
        System.out.println("127eq=" + (a == b));
        Integer c = 128, d = 128;
        System.out.println("128eq=" + (c == d));
        Integer e = Integer.valueOf(128);
        System.out.println("valueOf128eq=" + (c == e));
        System.out.println("equals128=" + c.equals(d));
        Integer n = null;
        try {
            int x = n;
            System.out.println(x);
        } catch (NullPointerException ex) {
            System.out.println("unboxNull=" + ex.getClass().getSimpleName());
        }
        List<Integer> list = List.of(1, 2, 3);
        System.out.println("list=" + list);
        System.out.println("sum=" + sum(new int[]{1, 2, 3}));
        System.out.println("sumBox=" + sum(new Integer[]{4, 5, 6}));
    }

    static int sum(int[] xs) { int s = 0; for (int x : xs) s += x; return s; }
    static int sum(Integer[] xs) { int s = 0; for (int x : xs) s += x; return s; }
}
