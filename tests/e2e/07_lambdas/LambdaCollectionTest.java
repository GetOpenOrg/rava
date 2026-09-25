import java.util.ArrayList;
import java.util.function.Consumer;

public class LambdaCollectionTest {
    public static void main(String[] args) {
        // forEach with lambda
        ArrayList<String> names = new ArrayList<>();
        names.add("Alice");
        names.add("Bob");
        names.add("Charlie");

        System.out.println("Names:");
        names.forEach(name -> System.out.println("  " + name));

        // forEach with a more complex lambda
        ArrayList<Integer> numbers = new ArrayList<>();
        numbers.add(1);
        numbers.add(2);
        numbers.add(3);

        System.out.println("Doubled:");
        numbers.forEach(n -> System.out.println("  " + (n * 2)));
    }
}
