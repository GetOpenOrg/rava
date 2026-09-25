import java.util.ArrayList;

public class AbstractClassTest {

    // Abstract base class with concrete method that calls abstract methods (virtual dispatch)
    static abstract class Shape {
        String name;
        Shape(String name) { this.name = name; }
        abstract double area();
        abstract double perimeter();
        // This calls this.area() and this.perimeter() — must dispatch to subclass
        String describe() {
            return name + ": area=" + String.format("%.2f", area())
                       + " perimeter=" + String.format("%.2f", perimeter());
        }
    }

    static class Circle extends Shape {
        double radius;
        Circle(double radius) {
            super("Circle");
            this.radius = radius;
        }
        @Override
        double area() { return Math.PI * radius * radius; }
        @Override
        double perimeter() { return 2 * Math.PI * radius; }
    }

    static class Rectangle extends Shape {
        double width, height;
        Rectangle(double width, double height) {
            super("Rectangle");
            this.width = width;
            this.height = height;
        }
        @Override
        double area() { return width * height; }
        @Override
        double perimeter() { return 2 * (width + height); }
    }

    static class Triangle extends Shape {
        double a, b, c;
        Triangle(double a, double b, double c) {
            super("Triangle");
            this.a = a; this.b = b; this.c = c;
        }
        @Override
        double area() {
            double s = (a + b + c) / 2;
            return Math.sqrt(s * (s - a) * (s - b) * (s - c));
        }
        @Override
        double perimeter() { return a + b + c; }
    }

    // Abstract with partial implementation — toString_() calls abstract sound()
    static abstract class Animal {
        String type;
        Animal(String type) { this.type = type; }
        abstract String sound();
        String toString_() {
            return type + " says " + sound();
        }
    }

    static class Dog extends Animal {
        Dog() { super("Dog"); }
        @Override
        String sound() { return "Woof"; }
    }

    static class Cat extends Animal {
        Cat() { super("Cat"); }
        @Override
        String sound() { return "Meow"; }
    }

    // Abstract chain: abstract -> abstract -> concrete
    static abstract class Base {
        abstract String id();
    }

    static abstract class Middle extends Base {
        abstract int priority();
        // Calls abstract methods from both levels
        String summary() { return id() + ":" + priority(); }
    }

    static class Concrete extends Middle {
        @Override
        String id() { return "C1"; }
        @Override
        int priority() { return 5; }
    }

    public static void main(String[] args) {
        // Test 1: Virtual dispatch — describe() calls subclass area()/perimeter()
        Circle c = new Circle(5.0);
        System.out.println(c.describe());
        // Circle: area=78.54 perimeter=31.42

        Rectangle r = new Rectangle(3.0, 4.0);
        System.out.println(r.describe());
        // Rectangle: area=12.00 perimeter=14.00

        Triangle t = new Triangle(3.0, 4.0, 5.0);
        System.out.println(t.describe());
        // Triangle: area=6.00 perimeter=12.00

        // Test 2: Direct method calls still work
        System.out.println(String.format("%.2f", c.area()));     // 78.54
        System.out.println(String.format("%.2f", r.area()));     // 12.00
        System.out.println(String.format("%.2f", t.area()));     // 6.00

        // Test 3: Virtual dispatch through toString_()
        Dog dog = new Dog();
        Cat cat = new Cat();
        System.out.println(dog.toString_());  // Dog says Woof
        System.out.println(cat.toString_());  // Cat says Meow

        // Test 4: Multi-level abstract chain — summary() calls two abstract methods
        Concrete obj = new Concrete();
        System.out.println(obj.summary()); // C1:5

        // Test 5: Sum area using direct calls
        double total = c.area() + r.area() + t.area();
        System.out.println(String.format("%.2f", total));  // 96.54

        // Test 6: Inherited fields from abstract parent
        System.out.println(c.name);  // Circle
        System.out.println(r.name);  // Rectangle
    }
}
