public class TestPatternMatch {
    sealed interface Shape permits TestPatternMatch.Circle, TestPatternMatch.Rectangle, TestPatternMatch.Triangle {}
    record Circle(double radius) implements Shape {}
    record Rectangle(double width, double height) implements Shape {}
    record Triangle(double base, double height) implements Shape {}

    static double area(Shape s) {
        return switch (s) {
            case Circle c -> Math.PI * c.radius() * c.radius();
            case Rectangle r -> r.width() * r.height();
            case Triangle t -> 0.5 * t.base() * t.height();
        };
    }

    static String describe(Object obj) {
        return switch (obj) {
            case Integer i when i > 0 -> "positive int: " + i;
            case Integer i -> "non-positive int: " + i;
            case String s when s.isEmpty() -> "empty string";
            case String s -> "string: " + s;
            default -> "other: " + obj;
        };
    }

    public static void main(String[] args) {
        System.out.printf("%.2f%n", area(new Circle(5)));
        System.out.printf("%.2f%n", area(new Rectangle(4, 6)));
        System.out.printf("%.2f%n", area(new Triangle(3, 8)));

        System.out.println(describe(42));
        System.out.println(describe(-1));
        System.out.println(describe(""));
        System.out.println(describe("hello"));
    }
}
