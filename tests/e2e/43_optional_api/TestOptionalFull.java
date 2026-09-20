import java.util.Optional;

public class TestOptionalFull {
    public static void main(String[] args) {
        Optional<String> o = Optional.of("v");
        o.ifPresentOrElse(v -> System.out.println("present=" + v), () -> System.out.println("empty"));
        Optional<String> e = Optional.empty();
        e.ifPresentOrElse(v -> System.out.println("present"), () -> System.out.println("empty-branch"));
        System.out.println("or=" + e.or(() -> Optional.of("fallback")).get());
        System.out.println("stream=" + o.stream().map(String::toUpperCase).findFirst().orElse("?"));
        System.out.println("emptyStream=" + e.stream().count());
        System.out.println("flatMap=" + Optional.of(2).flatMap(x -> Optional.of(x * 10)).get());
        System.out.println("filterEmpty=" + Optional.of(5).filter(x -> x > 100).orElse(-1));
        try {
            e.orElseThrow();
            System.out.println("no throw");
        } catch (Exception ex) {
            System.out.println("threw=" + ex.getClass().getSimpleName());
        }
    }
}
