import java.util.*;
import java.util.stream.*;

public class CollectorsDemo {
    public static void main(String[] args) {
        // Collectors.joining
        List<String> words = List.of("hello", "world", "java");
        String joined = words.stream().collect(Collectors.joining(", "));
        System.out.println(joined);   // hello, world, java

        String joinedWithBraces = words.stream().collect(Collectors.joining(", ", "[", "]"));
        System.out.println(joinedWithBraces);   // [hello, world, java]

        // Collectors.toMap (key: identity, value: length)
        List<String> names = Arrays.asList("Alice", "Bob", "Carol");
        Map<String, Integer> nameLengths = names.stream()
            .collect(Collectors.toMap(s -> s, s -> s.length()));
        System.out.println(nameLengths.get("Alice"));  // 5
        System.out.println(nameLengths.get("Bob"));    // 3

        // Collectors.groupingBy by first letter (as String substring)
        List<String> fruits = Arrays.asList("apple", "banana", "cherry", "avocado");
        Map<String, List<String>> byFirst = fruits.stream()
            .collect(Collectors.groupingBy(s -> s.substring(0, 1)));
        List<String> aFruits = byFirst.get("a");
        System.out.println(aFruits.size());   // 2

        // Collectors.toList
        List<Integer> nums = Arrays.asList(1, 2, 3, 4, 5);
        List<Integer> toList = nums.stream().collect(Collectors.toList());
        System.out.println(toList.size());    // 5
    }
}
