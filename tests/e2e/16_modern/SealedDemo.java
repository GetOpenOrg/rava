public class SealedDemo {
    sealed abstract static class Shape permits Circle, Rectangle {
        abstract double area();
        abstract String name();
    }

    static final class Circle extends Shape {
        final double radius;

        Circle(double radius) {
            this.radius = radius;
        }

        @Override
        public double area() {
            return Math.PI * radius * radius;
        }

        @Override
        public String name() {
            return "Circle";
        }
    }

    static final class Rectangle extends Shape {
        final double width;
        final double height;

        Rectangle(double w, double h) {
            this.width = w;
            this.height = h;
        }

        @Override
        public double area() {
            return width * height;
        }

        @Override
        public String name() {
            return "Rectangle";
        }
    }

    public static void main(String[] args) {
        Shape s1 = new Circle(5.0);
        Shape s2 = new Rectangle(3.0, 4.0);
        System.out.println(s1.name());
        System.out.println(s2.name());
        System.out.println((int) s1.area());
        System.out.println((int) s2.area());
        System.out.println(s1 instanceof Circle);
        System.out.println(s2 instanceof Rectangle);
        System.out.println(s1 instanceof Rectangle);
    }
}
