import java.util.Objects;

public class TestEqualsHashCode {

    static class Point {
        int x, y;
        Point(int x, int y) { this.x = x; this.y = y; }

        @Override
        public boolean equals(Object obj) {
            if (this == obj) return true;
            if (!(obj instanceof Point)) return false;
            Point p = (Point) obj;
            return x == p.x && y == p.y;
        }

        @Override
        public int hashCode() {
            return Objects.hash(x, y);
        }

        @Override
        public String toString() { return "Point(" + x + ", " + y + ")"; }
    }

    static class Person {
        String name;
        int age;
        Person(String name, int age) { this.name = name; this.age = age; }

        @Override
        public boolean equals(Object obj) {
            if (this == obj) return true;
            if (!(obj instanceof Person)) return false;
            Person p = (Person) obj;
            return age == p.age && Objects.equals(name, p.name);
        }

        @Override
        public int hashCode() {
            return Objects.hash(name, age);
        }

        @Override
        public String toString() { return name + "(" + age + ")"; }
    }

    public static void main(String[] args) {
        Point p1 = new Point(1, 2);
        Point p2 = new Point(1, 2);
        Point p3 = new Point(3, 4);

        System.out.println(p1.equals(p2));   // true
        System.out.println(p1.equals(p3));   // false
        System.out.println(p1.equals(null)); // false
        System.out.println(p1.equals("str")); // false
        System.out.println(p1 == p2);        // false (different references)
        System.out.println(p1.hashCode() == p2.hashCode()); // true

        System.out.println(p1.toString()); // Point(1, 2)
        System.out.println(p3.toString()); // Point(3, 4)

        Person alice1 = new Person("Alice", 30);
        Person alice2 = new Person("Alice", 30);
        Person bob   = new Person("Bob", 25);

        System.out.println(alice1.equals(alice2)); // true
        System.out.println(alice1.equals(bob));    // false
        System.out.println(alice1.hashCode() == alice2.hashCode()); // true

        // reflexive
        System.out.println(p1.equals(p1));       // true
        System.out.println(alice1.equals(alice1)); // true

        // symmetric
        System.out.println(p1.equals(p2) == p2.equals(p1)); // true
    }
}
