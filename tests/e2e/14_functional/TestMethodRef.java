import java.util.Arrays;
import java.util.List;
import java.util.function.Function;
import java.util.function.Predicate;

public class TestMethodRef {
    static int doubleIt(int x) {
        return x * 2;
    }

    static boolean isPositive(int x) {
        return x > 0;
    }

    public static void main(String[] args) {
        List<String> words = Arrays.asList("hello", "world", "java");

        // Method reference: instance method of class
        words.forEach(System.out::println);

        // Method reference: static method
        Function<Integer, Integer> dbl = TestMethodRef::doubleIt;
        System.out.println(dbl.apply(5));

        // Method reference: instance method of object
        String prefix = ">>>>";
        Function<String, String> addPrefix = prefix::concat;
        System.out.println(addPrefix.apply("test"));

        // Constructor reference
        Function<String, StringBuilder> sbMaker = StringBuilder::new;
        StringBuilder sb = sbMaker.apply("initial");
        System.out.println(sb.toString());
    }
}
