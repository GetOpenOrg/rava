import java.util.List;
import java.util.Set;
import java.util.Map;
import java.util.Arrays;

public class TestCollectionFactory {
    public static void main(String[] args) {
        List<String> l = List.of("a", "b");
        System.out.println("list=" + l);
        Set<Integer> s = Set.of(1, 2, 3);
        System.out.println("set=" + s.stream().sorted().toList());
        Map<String, Integer> m = Map.of("x", 1, "y", 2);
        System.out.println("map=" + m.entrySet().stream().sorted(Map.Entry.comparingByKey()).map(e -> e.getKey() + "=" + e.getValue()).toList());
        Map<String, Integer> me = Map.ofEntries(Map.entry("p", 10));
        System.out.println("entries=" + me);

        try {
            l.add("c");
        } catch (UnsupportedOperationException ex) {
            System.out.println("listAdd=" + ex.getClass().getSimpleName());
        }
        try {
            m.put("z", 3);
        } catch (UnsupportedOperationException ex) {
            System.out.println("mapPut=" + ex.getClass().getSimpleName());
        }
        List<String> lc = List.copyOf(l);
        System.out.println("copyOf=" + lc);
        System.out.println("toArray=" + Arrays.toString(l.toArray(new String[0])));
        System.out.println("setCopyOf=" + Set.copyOf(s).stream().sorted().toList());
    }
}
