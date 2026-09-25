
public class RecordDemo {
    record Point(int x, int y) {
        double distance() {
            return Math.sqrt(x * x + y * y);
        }
    }

    record Person(String name, int age) {}

    public static void main(String[] args) {
        Point p = new Point(3, 4);
        System.out.println(p.x());
        System.out.println(p.y());
        System.out.println((int) p.distance());

        Point p2 = new Point(3, 4);
        System.out.println(p.equals(p2));
        System.out.println(p.toString());

        Person alice = new Person("Alice", 30);
        System.out.println(alice.name());
        System.out.println(alice.age());
    }
}
