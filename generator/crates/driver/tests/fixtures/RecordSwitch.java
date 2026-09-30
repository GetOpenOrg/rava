// rava build 集成测试夹具：record 的 ObjectMethods 引导、typeSwitch（含限定枚举常量标签）、enumSwitch、字符串拼接
public class RecordSwitch {
    record Point(int x, long y, double z, boolean b, String name) {}

    enum Color implements Tag { RED, GREEN, BLUE }

    sealed interface Tag permits Color, Label {}

    record Label(String text) implements Tag {}

    static String describe(Object o) {
        return switch (o) {
            case Point p when p.x() > 0 -> "point+" + p.name();
            case Point p -> "point " + p.x();
            case Label l -> "label " + l.text();
            default -> "other";
        };
    }

    static int tagCode(Tag t) {
        return switch (t) {
            case Color.RED -> 1;
            case Color c -> 10 + c.ordinal();
            case Label l -> 100;
        };
    }

    static int colorCode(Color c) {
        return switch (c) {
            case RED -> 1;
            case Color x when x.ordinal() > 1 -> 3;
            default -> 2;
        };
    }

    public static void main(String[] args) {
        Point p = new Point(1, 2L, 3.5, true, "a");
        Point q = new Point(1, 2L, 3.5, true, "a");
        System.out.println(p);
        System.out.println(p + " / " + q);
        System.out.println(p.equals(q) + " " + (p.hashCode() == q.hashCode()));
        System.out.println(new Label("t").equals(new Label("t")) + " " + new Label("t").equals(new Label("u")));
        System.out.println(describe(p) + " " + describe(new Label("t")) + " " + describe("s"));
        System.out.println(tagCode(Color.RED) + " " + tagCode(Color.BLUE) + " " + tagCode(new Label("x")));
        System.out.println(colorCode(Color.RED) + " " + colorCode(Color.GREEN) + " " + colorCode(Color.BLUE));
    }
}
