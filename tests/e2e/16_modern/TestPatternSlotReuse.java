import java.util.List;

/**
 * javac 为 record 模式 / switch 模式生成的合成临时变量在不同分支复用同一局部变量槽、
 * 存放不同类型（int 分量 vs 记录本身；Rosetta RecordPatternsTest 揭出：两类型合并成
 * 一个提升声明 → E0308）。覆盖：switch 记录模式（基本分量 + 嵌套记录）、instanceof 记录
 * 模式 && 链、long/double/String 分量混合、when 守卫、同一方法内多个模式 switch、
 * 模式与 for-each / synchronized 合成槽并存。
 */
public class TestPatternSlotReuse {
    record Point(int x, int y) {}
    enum Color { RED, GREEN }
    record ColoredPoint(Point p, Color c) {}
    record Mixed(long id, double w, String name) {}
    record Box(Object content) {}

    static String describe(Object o) {
        return switch (o) {
            case Point(int x, int y) when x == y -> "diag " + x;
            case Point(int x, int y) -> "point " + x + "," + y;
            case ColoredPoint(Point(int x, int y), Color c) -> "colored " + x + "," + y + " " + c;
            case Mixed(long id, double w, String name) -> "mixed " + id + " " + w + " " + name;
            case Box(Point(int x, int y)) -> "boxed point " + (x + y);
            case Box(String s) -> "boxed string " + s.length();
            case Box(Object other) -> "boxed " + other;
            case null -> "null";
            default -> "other " + o;
        };
    }

    static int twoSwitches(Object a, Object b) {
        int first = switch (a) {
            case Point(int x, int y) -> x * 10 + y;
            case ColoredPoint(Point(int x, int y), Color c) -> x + y + c.ordinal();
            default -> -1;
        };
        int second = switch (b) {
            case Mixed(long id, double w, String n) -> (int) id + n.length();
            case ColoredPoint(Point p, Color c) -> p.x() * 100 + c.ordinal();
            default -> -2;
        };
        return first * 1000 + second;
    }

    static String collide(Object p1, Object p2) {
        if (p1 instanceof Point(int x1, int y1) && p2 instanceof ColoredPoint(Point(int x2, int y2), Color c)) {
            return "P/CP " + (x1 == x2 && y1 == y2) + " " + c;
        }
        if (p1 instanceof ColoredPoint(Point(int x1, int y1), Color c) && p2 instanceof Point(int x2, int y2)) {
            return "CP/P " + (x1 == x2 && y1 == y2) + " " + c;
        }
        if (p1 instanceof ColoredPoint(Point(int x1, int y1), Color c1)
                && p2 instanceof ColoredPoint(Point(int x2, int y2), Color c2)) {
            return "CP/CP " + (x1 == x2 && y1 == y2) + " " + (c1 == c2);
        }
        return "none";
    }

    static int withSyntheticLoops(List<Object> items) {
        int total = 0;
        Object lock = new Object();
        for (Object it : items) {
            synchronized (lock) {
                total += switch (it) {
                    case Point(int x, int y) -> x + y;
                    case Mixed(long id, double w, String n) -> (int) id;
                    default -> 0;
                };
            }
        }
        int[] arr = {1, 2, 3};
        for (int v : arr) {
            total += v;
        }
        return total;
    }

    public static void main(String[] args) {
        Object[] samples = {
            new Point(3, 3), new Point(1, 2), new ColoredPoint(new Point(4, 5), Color.GREEN),
            new Mixed(7L, 2.5, "abc"), new Box(new Point(2, 9)), new Box("hello"), new Box(42), null, "str",
        };
        for (Object o : samples) {
            System.out.println(describe(o));
        }
        System.out.println(twoSwitches(new Point(1, 2), new Mixed(3L, 1.0, "xy")));
        System.out.println(twoSwitches(new ColoredPoint(new Point(1, 1), Color.GREEN),
                new ColoredPoint(new Point(4, 0), Color.RED)));
        System.out.println(twoSwitches("a", "b"));
        Point p = new Point(1, 2);
        ColoredPoint cp = new ColoredPoint(new Point(1, 2), Color.RED);
        ColoredPoint cp2 = new ColoredPoint(new Point(1, 2), Color.RED);
        System.out.println(collide(p, cp));
        System.out.println(collide(cp, p));
        System.out.println(collide(cp, cp2));
        System.out.println(collide(p, p));
        System.out.println(withSyntheticLoops(List.of(new Point(1, 2), new Mixed(10L, 0.5, "q"), "x")));
    }
}
