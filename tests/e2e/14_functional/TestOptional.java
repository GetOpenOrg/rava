import java.util.Optional;

public class TestOptional {
    static Optional<String> findByLength(String[] arr, int len) {
        for (String s : arr) {
            if (s.length() == len) {
                return Optional.of(s);
            }
        }
        return Optional.empty();
    }

    public static void main(String[] args) {
        String[] words = {"hi", "hello", "hey", "world"};

        Optional<String> found = findByLength(words, 5);
        System.out.println(found.isPresent());
        System.out.println(found.get());

        Optional<String> notFound = findByLength(words, 10);
        System.out.println(notFound.isPresent());
        System.out.println(notFound.orElse("default"));

        // map / filter
        Optional<Integer> len = found.map(String::length);
        System.out.println(len.get());
    }
}
