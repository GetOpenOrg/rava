import java.util.HashMap;
import java.util.Map;
import java.util.Optional;
import java.util.ArrayList;
import java.util.List;
import java.util.stream.Collectors;

public class RuntimeMethodsTest {
    public static void main(String[] args) {
        // 1. HashMap replace
        Map<String, Integer> map = new HashMap<>();
        map.put("a", 1);
        map.put("b", 2);
        Object old = map.replace("a", 10);
        System.out.println("replace: " + old + " -> " + map.get("a"));
        boolean ok = map.replace("b", 2, 20);
        System.out.println("replace_kv: " + ok + " -> " + map.get("b"));
        boolean fail = map.replace("b", 999, 30);
        System.out.println("replace_kv_fail: " + fail + " -> " + map.get("b"));

        // 2. HashMap remove(key, value)
        map.put("c", 3);
        boolean removed = map.remove("c", 999);
        System.out.println("remove_kv_fail: " + removed + " has_c: " + map.containsKey("c"));
        removed = map.remove("c", 3);
        System.out.println("remove_kv: " + removed + " has_c: " + map.containsKey("c"));

        // 3. Optional.or
        Optional<String> empty = Optional.empty();
        Optional<String> result = empty.or(() -> Optional.of("fallback"));
        System.out.println("or: " + result.get());

        // 4. Optional.ifPresentOrElse
        Optional<String> present = Optional.of("hello");
        present.ifPresentOrElse(
            v -> System.out.println("present: " + v),
            () -> System.out.println("empty")
        );
        empty.ifPresentOrElse(
            v -> System.out.println("present: " + v),
            () -> System.out.println("empty action")
        );

        // 5. Optional.stream
        Optional<String> opt = Optional.of("stream-val");
        List<String> list = opt.stream().collect(Collectors.toList());
        System.out.println("opt.stream: " + list);
        List<String> emptyList = Optional.<String>empty().stream().collect(Collectors.toList());
        System.out.println("empty.stream: " + emptyList);

        // 6. Integer unsigned operations
        System.out.println("compareUnsigned: " + Integer.compareUnsigned(-1, 1));
        System.out.println("toUnsignedLong: " + Integer.toUnsignedLong(-1));
        System.out.println("rotateLeft: " + Integer.rotateLeft(1, 3));
        System.out.println("highestOneBit: " + Integer.highestOneBit(10));
        System.out.println("lowestOneBit: " + Integer.lowestOneBit(12));

        // 7. Long unsigned operations
        System.out.println("Long.compareUnsigned: " + Long.compareUnsigned(-1L, 1L));
        System.out.println("Long.rotateLeft: " + Long.rotateLeft(1L, 3));
        System.out.println("Long.highestOneBit: " + Long.highestOneBit(10L));

        System.out.println("Done.");
    }
}
