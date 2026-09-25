public class AbstractShape {
    abstract static class Shape {
        public abstract double area();
        public abstract String name();
    }

    static class Circle extends Shape {
        private double radius;
        public Circle(double r) { this.radius = r; }
        public double area() { return 3.14159 * radius * radius; }
        public String name() { return "circle"; }
    }

    static class Rectangle extends Shape {
        private double w, h;
        public Rectangle(double w, double h) { this.w = w; this.h = h; }
        public double area() { return w * h; }
        public String name() { return "rectangle"; }
    }

    public static void main(String[] args) {
        Shape c = new Circle(2.0);
        Shape r = new Rectangle(3.0, 4.0);

        // Virtual dispatch through abstract base type
        System.out.println((int) c.area());  // 12  (3.14159 * 4 ≈ 12)
        System.out.println((int) r.area());  // 12  (3 * 4 = 12)
        System.out.println(c.name());        // circle
        System.out.println(r.name());        // rectangle
    }
}
