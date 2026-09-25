import java.util.function.Function;
import java.util.function.Consumer;
import java.util.ArrayList;

public class MethodRefTest {
    public static void main(String[] args) {
        // Static method reference: Integer::valueOf
        Function<String, Integer> parser = Integer::parseInt;
        System.out.println("Parsed 42: " + parser.apply("42"));
        System.out.println("Parsed 100: " + parser.apply("100"));

        // Instance method reference on a type: String::length
        Function<String, Integer> lengthFn = String::length;
        System.out.println("Length of hello: " + lengthFn.apply("hello"));
        System.out.println("Length of hi: " + lengthFn.apply("hi"));

        // Instance method reference on an object
        ArrayList<String> list = new ArrayList<>();
        list.add("one");
        list.add("two");
        list.add("three");

        Consumer<String> printer = System.out::println;
        list.forEach(printer);
    }
}
