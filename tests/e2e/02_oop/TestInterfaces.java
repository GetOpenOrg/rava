public class TestInterfaces {

    interface Drawable {
        void draw();

        default String description() {
            return "a drawable object";
        }
    }

    interface Resizable {
        void resize(double factor);
    }

    static class Circle implements Drawable, Resizable {
        double radius;

        Circle(double radius) {
            this.radius = radius;
        }

        @Override
        public void draw() {
            System.out.println("Circle r=" + radius);
        }

        @Override
        public void resize(double factor) {
            radius *= factor;
        }
    }

    static class Rectangle implements Drawable {
        double width, height;

        Rectangle(double width, double height) {
            this.width = width;
            this.height = height;
        }

        @Override
        public void draw() {
            System.out.println("Rectangle " + width + "x" + height);
        }
    }

    public static void main(String[] args) {
        Circle c = new Circle(5.0);
        c.draw();
        System.out.println(c.description());

        c.resize(2.0);
        c.draw();

        Rectangle r = new Rectangle(3.0, 4.0);
        r.draw();
        System.out.println(r.description());

        // 接口类型引用
        Drawable d = new Circle(1.0);
        d.draw();

        // instanceof
        System.out.println(c instanceof Drawable);
        System.out.println(c instanceof Resizable);
        System.out.println(r instanceof Resizable);

        // 接口数组多态
        Drawable[] shapes = { c, r, new Circle(3.0) };
        for (Drawable s : shapes) {
            s.draw();
        }
    }
}
