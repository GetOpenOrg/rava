public class RecordTest {

    // Simple record with two int fields
    record Point(int x, int y) {}

    // Record with String field
    record Person(String name, int age) {}

    // Record with single field
    record Wrapper(String value) {}

    public static void main(String[] args) {
        // 1. Construction + accessor
        Point p1 = new Point(3, 4);
        System.out.println("p1.x = " + p1.x());
        System.out.println("p1.y = " + p1.y());

        // 2. toString
        System.out.println("p1.toString = " + p1.toString());

        // 3. hashCode consistency
        Point p2 = new Point(3, 4);
        boolean hashMatch = p1.hashCode() == p2.hashCode();
        System.out.println("p1.hashCode == p2.hashCode: " + hashMatch);

        // 4. equals
        boolean eq1 = p1.equals(p2);
        System.out.println("p1.equals(p2): " + eq1);
        Point p3 = new Point(1, 2);
        boolean eq2 = p1.equals(p3);
        System.out.println("p1.equals(p3): " + eq2);

        // 5. String field record
        Person person = new Person("Alice", 30);
        System.out.println("person.toString = " + person.toString());
        System.out.println("person.name = " + person.name());
        System.out.println("person.age = " + person.age());

        Person person2 = new Person("Alice", 30);
        boolean eq3 = person.equals(person2);
        System.out.println("person.equals(person2): " + eq3);

        Person person3 = new Person("Bob", 25);
        boolean eq4 = person.equals(person3);
        System.out.println("person.equals(person3): " + eq4);

        // 6. Wrapper record
        Wrapper w = new Wrapper("hello");
        System.out.println("w.toString = " + w.toString());
        System.out.println("w.value = " + w.value());
    }
}
