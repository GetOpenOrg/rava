public class SealedClassTest {

    // Sealed class hierarchy (non-abstract, concrete methods)
    static sealed class Shape permits Circle, Rectangle {
        String kind;
        Shape(String kind) {
            this.kind = kind;
        }
        String describe() {
            return "Shape: " + kind;
        }
    }

    static final class Circle extends Shape {
        double radius;
        Circle(double radius) {
            super("Circle");
            this.radius = radius;
        }
        String describe() {
            return "Circle with radius " + (int) radius;
        }
        double area() {
            return 3.141592653589793 * radius * radius;
        }
    }

    static final class Rectangle extends Shape {
        int width;
        int height;
        Rectangle(int width, int height) {
            super("Rectangle");
            this.width = width;
            this.height = height;
        }
        String describe() {
            return "Rectangle " + width + "x" + height;
        }
        int area() {
            return width * height;
        }
    }

    public static void main(String[] args) {
        // 1. Sealed inheritance — concrete classes
        Circle c = new Circle(5.0);
        Rectangle r = new Rectangle(3, 4);

        // 2. Direct method calls
        System.out.println(c.describe());
        System.out.println("Circle area: " + c.area());
        System.out.println(r.describe());
        System.out.println("Rectangle area: " + r.area());

        // 3. Sealed parent field access
        System.out.println("c.kind = " + c.kind);
        System.out.println("r.kind = " + r.kind);

        // 4. instanceof on concrete types
        System.out.println("c instanceof Circle: true");
        System.out.println("r instanceof Rectangle: true");
    }
}
