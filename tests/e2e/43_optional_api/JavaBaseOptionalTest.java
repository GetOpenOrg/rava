import java.util.Optional;

public class JavaBaseOptionalTest {
    public static void main(String[] args) {
        // Optional.of
        Optional<String> opt = Optional.of("hello");
        System.out.println("isPresent: " + opt.isPresent());
        System.out.println("get: " + opt.get());

        // Optional.empty
        Optional<String> empty = Optional.empty();
        System.out.println("empty isPresent: " + empty.isPresent());

        // Optional.ofNullable
        Optional<String> nullable = Optional.ofNullable(null);
        System.out.println("nullable isPresent: " + nullable.isPresent());
        Optional<String> nonNull = Optional.ofNullable("world");
        System.out.println("nonNull isPresent: " + nonNull.isPresent());
        System.out.println("nonNull get: " + nonNull.get());

        // orElse
        System.out.println("empty orElse: " + empty.orElse("default"));
        System.out.println("opt orElse: " + opt.orElse("default"));

        // toString
        System.out.println("opt toString: " + opt.toString());
        System.out.println("empty toString: " + empty.toString());
    }
}
