import java.util.Arrays;
import java.util.List;
import java.util.Map;
import java.util.stream.Collectors;

public class TestStreamCollectors {

    static class Person {
        String name;
        int age;
        String dept;

        Person(String name, int age, String dept) {
            this.name = name;
            this.age = age;
            this.dept = dept;
        }

        @Override
        public String toString() { return name; }
    }

    public static void main(String[] args) {
        List<String> words = Arrays.asList("hello", "world", "java", "streams", "rocks");

        // joining
        String joined = words.stream().collect(Collectors.joining(", "));
        System.out.println(joined); // hello, world, java, streams, rocks

        String withBrackets = words.stream().collect(Collectors.joining(", ", "[", "]"));
        System.out.println(withBrackets); // [hello, world, java, streams, rocks]

        // toList (via collect)
        List<String> upper = words.stream()
            .map(String::toUpperCase)
            .collect(Collectors.toList());
        System.out.println(upper); // [HELLO, WORLD, JAVA, STREAMS, ROCKS]

        // counting
        long count = words.stream()
            .filter(w -> w.length() > 4)
            .collect(Collectors.counting());
        System.out.println(count); // 3

        // groupingBy
        List<Person> people = Arrays.asList(
            new Person("Alice", 30, "Engineering"),
            new Person("Bob", 25, "Marketing"),
            new Person("Charlie", 28, "Engineering"),
            new Person("Dave", 35, "Marketing"),
            new Person("Eve", 27, "Engineering")
        );

        Map<String, List<Person>> byDept = people.stream()
            .collect(Collectors.groupingBy(p -> p.dept));

        List<String> engNames = byDept.get("Engineering").stream()
            .map(p -> p.name)
            .sorted()
            .collect(Collectors.toList());
        System.out.println(engNames); // [Alice, Charlie, Eve]

        List<String> mktNames = byDept.get("Marketing").stream()
            .map(p -> p.name)
            .sorted()
            .collect(Collectors.toList());
        System.out.println(mktNames); // [Bob, Dave]

        // partitioningBy
        Map<Boolean, List<String>> byLength = words.stream()
            .collect(Collectors.partitioningBy(w -> w.length() > 4));
        List<String> longWords = byLength.get(true).stream().sorted().collect(Collectors.toList());
        List<String> shortWords = byLength.get(false).stream().sorted().collect(Collectors.toList());
        System.out.println(longWords);  // [hello, rocks, streams, world]
        System.out.println(shortWords); // [java]

        // summarizingInt
        List<Integer> nums = Arrays.asList(1, 2, 3, 4, 5);
        int total = nums.stream().collect(Collectors.summingInt(Integer::intValue));
        System.out.println(total); // 15
    }
}
