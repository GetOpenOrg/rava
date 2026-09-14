public class TestAbstractClass {

    static abstract class Shape {
        String color;
        Shape(String color) { this.color = color; }
        abstract double area();
        String describe() { return color + " shape, area=" + area(); }
    }

    static class Square extends Shape {
        double side;
        Square(String color, double side) { super(color); this.side = side; }
        @Override
        double area() { return side * side; }
    }

    static class Rectangle extends Shape {
        double w, h;
        Rectangle(String color, double w, double h) { super(color); this.w = w; this.h = h; }
        @Override
        double area() { return w * h; }
    }

    static class Triangle extends Shape {
        double base, height;
        Triangle(String color, double base, double height) {
            super(color); this.base = base; this.height = height;
        }
        @Override
        double area() { return 0.5 * base * height; }
    }

    public static void main(String[] args) {
        Square s = new Square("red", 3.0);
        Rectangle r = new Rectangle("blue", 4.0, 5.0);
        Triangle t = new Triangle("green", 6.0, 4.0);

        System.out.println(s.area());     // 9.0
        System.out.println(r.area());     // 20.0
        System.out.println(t.area());     // 12.0

        System.out.println(s.describe()); // red shape, area=9.0
        System.out.println(r.describe()); // blue shape, area=20.0
        System.out.println(t.describe()); // green shape, area=12.0

        // polymorphic array
        Shape[] shapes = { s, r, t, new Square("black", 5.0) };
        for (Shape shape : shapes) {
            System.out.println(shape.area());
        }
        // 9.0, 20.0, 12.0, 25.0

        // instanceof on abstract type
        Shape anyShape = s;
        System.out.println(anyShape instanceof Shape);   // true
        System.out.println(anyShape instanceof Square);  // true
        anyShape = r;
        System.out.println(anyShape instanceof Square);  // false
    }
}
