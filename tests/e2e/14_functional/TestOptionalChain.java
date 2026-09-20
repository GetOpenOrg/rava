import java.util.Optional;
import java.util.OptionalInt;
import java.util.ArrayList;
import java.util.List;
import java.util.stream.Collectors;

public class TestOptionalChain {

    static Optional<String> lookup(String key) {
        if (key.equals("known")) {
            return Optional.of("value-" + key);
        }
        if (key.equals("empty-lower")) {
            return Optional.empty();
        }
        return Optional.ofNullable(key.length() > 3 ? key.toUpperCase() : null);
    }

    static Optional<Integer> parseInt(String s) {
        try {
            return Optional.of(Integer.valueOf(Integer.parseInt(s)));
        } catch (NumberFormatException e) {
            return Optional.empty();
        }
    }

    public static void main(String[] args) {
        // map / flatMap 链
        System.out.println(lookup("known").map(String::length).orElse(-1));
        System.out.println(lookup("empty-lower").map(String::length).orElse(-1));
        System.out.println(lookup("long key").map(v -> "[" + v + "]").orElse("none"));

        // flatMap 避免嵌套 Optional
        Optional<String> flat = lookup("known").flatMap(v -> Optional.of(v + "-nested"));
        System.out.println("flatMap=" + flat.orElse("empty"));

        Optional<String> flatEmpty = lookup("empty-lower").flatMap(v -> Optional.of(v + "-nested"));
        System.out.println("flatMap empty=" + flatEmpty.orElse("fallback"));

        // filter
        System.out.println("filter len>3=" + lookup("known").filter(v -> v.length() > 3).orElse("too short"));
        System.out.println("filter impossible=" + lookup("known").filter(v -> v.length() > 100).orElse("filtered out"));

        // orElse / orElseGet / orElseThrow
        System.out.println("orElse=" + Optional.<String>empty().orElse("default"));
        System.out.println("orElseGet=" + Optional.<String>empty().orElseGet(() -> "lazy-" + 42));
        try {
            Optional.<String>empty().orElseThrow(() -> new IllegalStateException("missing"));
        } catch (IllegalStateException e) {
            System.out.println("orElseThrow -> " + e.getMessage());
        }

        // ifPresent / ifPresentOrElse（Java 9+）
        List<String> sink = new ArrayList<>();
        lookup("known").ifPresent(v -> sink.add("got:" + v));
        lookup("empty-lower").ifPresent(v -> sink.add("should-not-appear"));
        lookup("empty-lower").ifPresentOrElse(v -> sink.add("present"), () -> sink.add("absent"));
        System.out.println("sink=" + sink);

        // Optional.of vs ofNullable vs empty
        System.out.println("isPresent=" + Optional.of("x").isPresent() + " " + Optional.empty().isPresent());
        System.out.println("isEmpty(Java11)=" + Optional.empty().isEmpty());
        try {
            Optional.of(null);
        } catch (NullPointerException e) {
            System.out.println("of(null) -> " + e.getClass().getSimpleName());
        }
        System.out.println("ofNullable(null) present=" + Optional.ofNullable(null).isPresent());

        // equals / hashCode / toString
        System.out.println("equals=" + Optional.of("a").equals(Optional.of("a")));
        System.out.println("toString=" + Optional.of("a").toString());

        // Optional 作为流的一环
        List<Optional<String>> optionals = new ArrayList<>();
        optionals.add(Optional.of("p"));
        optionals.add(Optional.empty());
        optionals.add(Optional.of("q"));
        List<String> present = optionals.stream()
                .filter(Optional::isPresent)
                .map(Optional::get)
                .collect(Collectors.toList());
        System.out.println("collected=" + present);

        // OptionalInt 原生特化
        OptionalInt oi = java.util.stream.IntStream.of(1, 2, 3).max();
        System.out.println("OptionalInt max=" + (oi.isPresent() ? oi.getAsInt() : -1));
        OptionalInt emptyInt = java.util.stream.IntStream.empty().max();
        System.out.println("OptionalInt empty get=" + emptyInt.orElse(-99));

        // 解析链路组合
        System.out.println("parse ok=" + parseInt("123").orElse(-1));
        System.out.println("parse bad=" + parseInt("abc").orElse(-1));
        System.out.println("parse chain=" + parseInt("10").map(v -> v * 2).filter(v -> v > 15).orElse(0));

        System.out.println("done");
    }
}
