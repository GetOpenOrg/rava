sealed interface Shape permits Circle, Square, Triangle {
    double area();

    default String kind() {
        return "shape";
    }
}

record Circle(double r) implements Shape {
    @Override
    public double area() {
        return Math.PI * r * r;
    }
}

record Square(double side) implements Shape {
    @Override
    public double area() {
        return side * side;
    }
}

non-sealed class Triangle implements Shape {
    private final double base;
    private final double height;

    Triangle(double base, double height) {
        this.base = base;
        this.height = height;
    }

    @Override
    public double area() {
        return base * height / 2;
    }

    @Override
    public String kind() {
        return "triangle";
    }
}

sealed interface Expr permits Literal, Add {
    int eval();
}

record Literal(int value) implements Expr {
    @Override
    public int eval() {
        return value;
    }
}

record Add(Expr left, Expr right) implements Expr {
    @Override
    public int eval() {
        return left.eval() + right.eval();
    }
}

public class TestSealed {

    // instanceof 模式匹配（Java 16+）
    static String describeShape(Shape s) {
        if (s instanceof Circle c) {
            return "circle r=" + c.r();
        } else if (s instanceof Square sq) {
            return "square side=" + sq.side();
        } else if (s instanceof Triangle t) {
            return t.kind() + " b/h";
        }
        return "unknown";
    }

    // switch 模式匹配（Java 21）
    static String switchShape(Shape s) {
        return switch (s) {
            case Circle c -> "sw-circle:" + c.r();
            case Square sq -> "sw-square:" + sq.side();
            default -> "sw-other";
        };
    }

    static int evalExpr(Expr e) {
        return e.eval();
    }

    public static void main(String[] args) {
        Shape c = new Circle(2.0);
        Shape sq = new Square(3.0);
        Shape tri = new Triangle(4.0, 5.0);

        System.out.println(describeShape(c));
        System.out.println(describeShape(sq));
        System.out.println(describeShape(tri));

        System.out.println("areas=" + c.area() + " " + sq.area() + " " + tri.area());

        System.out.println(switchShape(c));
        System.out.println(switchShape(sq));
        System.out.println(switchShape(tri));

        // sealed 类的递归组合
        Expr expr = new Add(new Literal(3), new Add(new Literal(4), new Literal(5)));
        System.out.println("expr=" + evalExpr(expr));

        // record + sealed 的 equals / toString
        System.out.println("record equals=" + new Circle(1.0).equals(new Circle(1.0)));
        System.out.println("record toString=" + new Square(2.0));

        // null 匹配
        Shape nothing = null;
        System.out.println("null instanceof=" + (nothing instanceof Circle));
        System.out.println("switch null -> " + switchNull(nothing));

        System.out.println("done");
    }

    static String switchNull(Shape s) {
        return switch (s) {
            case Circle cc -> "circle";
            case null -> "null-shape";
            default -> "other";
        };
    }
}
