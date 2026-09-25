class Shape {
    String name;

    Shape(String name) {
        this.name = name;
    }

    String getName() {
        return name;
    }
}

class Circle extends Shape {
    int radius;

    Circle(int radius) {
        super("Circle");
        this.radius = radius;
    }
}

public class InstanceOfTest {
    public static void main(String[] args) {
        Shape shape = new Shape("Generic");
        Circle circle = new Circle(5);

        // circle instanceof Shape -> true
        if (circle instanceof Shape) {
            System.out.println("circle is Shape: true");
        } else {
            System.out.println("circle is Shape: false");
        }

        // shape instanceof Circle -> false
        if (shape instanceof Circle) {
            System.out.println("shape is Circle: true");
        } else {
            System.out.println("shape is Circle: false");
        }

        // circle instanceof Circle -> true
        if (circle instanceof Circle) {
            System.out.println("circle is Circle: true");
        } else {
            System.out.println("circle is Circle: false");
        }
    }
}
