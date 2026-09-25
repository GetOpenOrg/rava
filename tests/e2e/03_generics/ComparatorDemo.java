import java.util.ArrayList;
import java.util.Collections;
import java.util.Comparator;

public class ComparatorDemo {
    static class Person implements Comparable<Person> {
        String name;
        int age;
        Person(String name, int age) { this.name = name; this.age = age; }

        @Override
        public int compareTo(Person other) {
            return Integer.compare(this.age, other.age);
        }

        @Override
        public String toString() { return name + ":" + age; }
    }

    public static void main(String[] args) {
        ArrayList<Person> people = new ArrayList<>();
        people.add(new Person("Charlie", 30));
        people.add(new Person("Alice", 25));
        people.add(new Person("Bob", 35));

        // Sort by natural ordering (Comparable)
        Collections.sort(people);
        for (Person p : people) {
            System.out.println(p);
        }

        System.out.println("---");

        // Sort by name using Comparator lambda
        Collections.sort(people, (a, b) -> a.name.compareTo(b.name));
        for (Person p : people) {
            System.out.println(p);
        }
    }
}
