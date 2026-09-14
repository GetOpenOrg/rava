import java.util.Arrays;
import java.util.Comparator;
import java.util.List;

public class TestComparator {
    public static void main(String[] args) {
        List<String> words = Arrays.asList("banana", "apple", "cherry", "date");

        // Sort by natural order
        words.sort(Comparator.naturalOrder());
        System.out.println(words);

        // Sort by length
        words.sort(Comparator.comparingInt(String::length));
        System.out.println(words);

        // Sort by length then alphabetically
        words.sort(Comparator.comparingInt(String::length).thenComparing(Comparator.naturalOrder()));
        System.out.println(words);

        // Reverse
        words.sort(Comparator.reverseOrder());
        System.out.println(words);
    }
}
