import java.util.Objects;
import java.util.Optional;

public class JavaBaseMultiTest {
    public static void main(String[] args) {
        // Test Objects methods independently
        System.out.println("Objects.equals a,a: " + Objects.equals("a", "a"));
        System.out.println("Objects.equals a,b: " + Objects.equals("a", "b"));
        System.out.println("Objects.isNull null: " + Objects.isNull(null));
        System.out.println("Objects.nonNull hello: " + Objects.nonNull("hello"));

        // Test Optional methods independently
        Optional<String> opt = Optional.of("hello");
        System.out.println("opt.isPresent: " + opt.isPresent());
        System.out.println("opt.get: " + opt.get());
        System.out.println("empty.isPresent: " + Optional.empty().isPresent());

        // Cross-class: requireNonNull on a string
        String s = Objects.requireNonNull("test");
        System.out.println("requireNonNull: " + s);

        // Cross-class: Objects.toString on Optional's result
        System.out.println("toString opt.get: " + Objects.toString(opt.get()));
        System.out.println("toString null,def: " + Objects.toString(null, "default"));
    }
}
