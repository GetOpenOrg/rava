import java.util.ArrayList;
import java.util.Arrays;
import java.util.Collections;
import java.util.List;

/**
 * Tests List operations with lambda expressions.
 */
public class ListLambdaTest {
    public static void main(String[] args) {
        // 1. removeIf with lambda predicate
        List<Integer> nums = new ArrayList<>(Arrays.asList(1, 2, 3, 4, 5, 6, 7, 8, 9, 10));
        nums.removeIf(n -> n % 2 == 0);
        System.out.println("odds: " + nums);

        // 2. replaceAll with lambda operator
        List<String> names = new ArrayList<>(Arrays.asList("alice", "bob", "charlie"));
        names.replaceAll(s -> s.substring(0, 1).toUpperCase() + s.substring(1));
        System.out.println("capitalized: " + names);

        // 3. Collections.sort with lambda comparator
        List<String> words = new ArrayList<>(Arrays.asList("banana", "fig", "cherry", "apple"));
        Collections.sort(words, (a, b) -> Integer.compare(a.length(), b.length()));
        System.out.println("by length: " + words);

        // 4. list.sort with lambda
        List<Integer> vals = new ArrayList<>(Arrays.asList(5, 3, 8, 1, 9));
        vals.sort((a, b) -> b - a);
        System.out.println("reversed: " + vals);

        // 5. removeIf with string predicate
        List<String> colors = new ArrayList<>(Arrays.asList("red", "green", "blue", "purple", "gold"));
        colors.removeIf(c -> c.length() > 4);
        System.out.println("short colors: " + colors);

        // 6. replaceAll with integer operation
        List<Integer> squares = new ArrayList<>(Arrays.asList(1, 2, 3, 4, 5));
        squares.replaceAll(n -> n * n);
        System.out.println("squares: " + squares);
    }
}
