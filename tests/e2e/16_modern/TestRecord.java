public class TestRecord {
    record Point(int x, int y) {
        double distance() {
            return Math.sqrt(x * x + y * y);
        }
    }

    record Person(String name, int age) {
        Person {
            if (age < 0) throw new IllegalArgumentException("negative age");
        }
    }

    public static void main(String[] args) {
        Point p = new Point(3, 4);
        System.out.println(p.x());
        System.out.println(p.y());
        System.out.println(p.distance());
        System.out.println(p);

        Point q = new Point(3, 4);
        System.out.println(p.equals(q));

        Person alice = new Person("Alice", 30);
        System.out.println(alice.name());
        System.out.println(alice.age());
    }
}
