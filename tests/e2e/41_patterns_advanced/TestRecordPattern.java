public class TestRecordPattern {
    interface Expr {}
    record Point(int x, int y) implements Expr {}
    record Node(int v) implements Expr {}
    record Leaf(int v) implements Expr {}
    record Tree(Expr node) implements Expr {}
    record Box(Point p) implements Expr {}

    static String describe(Object o) {
        if (o instanceof Point(int x, int y)) {
            return "point(" + x + "," + y + ")";
        }
        return "other";
    }

    static String classify(Object o) {
        return switch (o) {
            case Point(int x, int y) -> "P:" + (x + y);
            case Box(Point p) -> "B:" + p.x();
            case null -> "null";
            default -> "?";
        };
    }

    static String nested(Object o) {
        return switch (o) {
            case Tree(Node(int v)) -> "tree-node-" + v;
            case Tree(Leaf(int v)) -> "tree-leaf-" + v;
            default -> "?";
        };
    }

    public static void main(String[] args) {
        System.out.println(describe(new Point(3, 4)));
        System.out.println(describe("hi"));
        System.out.println(classify(new Point(1, 2)));
        System.out.println(classify(new Box(new Point(5, 6))));
        System.out.println(classify(null));
        System.out.println(classify("x"));
        System.out.println(nested(new Tree(new Node(9))));
        System.out.println(nested(new Tree(new Leaf(7))));
    }
}
