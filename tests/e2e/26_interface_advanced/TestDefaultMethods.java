public class TestDefaultMethods {

    interface Drawable {
        void draw();

        default void drawTwice() {
            draw();
            draw();
        }

        default String getType() {
            return "Drawable";
        }
    }

    interface Colorable {
        String getColor();

        default String describe() {
            return getColor() + " " + getClass().getSimpleName();
        }
    }

    static class Circle implements Drawable, Colorable {
        double radius;
        String color;

        Circle(double radius, String color) {
            this.radius = radius;
            this.color = color;
        }

        @Override
        public void draw() {
            System.out.println("Circle r=" + radius);
        }

        @Override
        public String getColor() { return color; }
    }

    interface Greetable {
        default String greet(String name) {
            return "Hello, " + name + "!";
        }
    }

    interface FormalGreetable extends Greetable {
        @Override
        default String greet(String name) {
            return "Good day, " + name + ".";
        }
    }

    static class FormalPerson implements FormalGreetable {}

    static class InformalPerson implements Greetable {}

    public static void main(String[] args) {
        Circle c = new Circle(5.0, "red");

        // default method
        c.drawTwice();
        // Circle r=5.0
        // Circle r=5.0

        System.out.println(c.getType());     // Drawable
        System.out.println(c.describe());    // red Circle

        // default method override in interface hierarchy
        FormalPerson fp = new FormalPerson();
        InformalPerson ip = new InformalPerson();

        System.out.println(fp.greet("Alice")); // Good day, Alice.
        System.out.println(ip.greet("Bob"));   // Hello, Bob!

        // polymorphic usage via interface ref
        Drawable d = c;
        d.draw();       // Circle r=5.0
        d.drawTwice();  // Circle r=5.0 x2
    }
}
