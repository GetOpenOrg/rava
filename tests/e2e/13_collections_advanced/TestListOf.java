import java.util.ArrayList;
import java.util.Arrays;
import java.util.Collections;
import java.util.List;
import java.util.Map;
import java.util.Set;

public class TestListOf {

    public static void main(String[] args) {
        // List.of 不可变
        List<String> fixed = List.of("a", "b", "c");
        System.out.println("list=" + fixed + " size=" + fixed.size());
        System.out.println("get(1)=" + fixed.get(1));
        System.out.println("indexOf(b)=" + fixed.indexOf("b"));
        System.out.println("contains(d)=" + fixed.contains("d"));

        // Map.of
        Map<String, Integer> m = Map.of("one", 1, "two", 2, "three", 3);
        System.out.println("size=" + m.size() + " get(two)=" + m.get("two"));
        List<String> keys = new ArrayList<>(m.keySet());
        Collections.sort(keys);
        System.out.println("sorted keys=" + keys);

        // Set.of
        Set<String> set = Set.of("x", "y");
        System.out.println("set size=" + set.size() + " contains(x)=" + set.contains("x"));

        // Map.ofEntries / entry
        Map<String, Integer> m2 = Map.ofEntries(Map.entry("k1", 10), Map.entry("k2", 20));
        System.out.println("entries size=" + m2.size() + " k1=" + m2.get("k1"));

        // 不可修改 → UnsupportedOperationException
        try {
            fixed.add("d");
            System.out.println("add accepted");
        } catch (UnsupportedOperationException e) {
            System.out.println("List.of add -> " + e.getClass().getSimpleName());
        }
        try {
            fixed.set(0, "z");
            System.out.println("set accepted");
        } catch (UnsupportedOperationException e) {
            System.out.println("List.of set -> " + e.getClass().getSimpleName());
        }
        try {
            m.put("four", 4);
            System.out.println("put accepted");
        } catch (UnsupportedOperationException e) {
            System.out.println("Map.of put -> " + e.getClass().getSimpleName());
        }
        try {
            m.remove("one");
            System.out.println("remove accepted");
        } catch (UnsupportedOperationException e) {
            System.out.println("Map.of remove -> " + e.getClass().getSimpleName());
        }

        // null 不允许
        try {
            List.of("a", null);
            System.out.println("null accepted");
        } catch (NullPointerException e) {
            System.out.println("List.of(null) -> " + e.getClass().getSimpleName());
        }
        try {
            Map.of("k", null);
            System.out.println("null value accepted");
        } catch (NullPointerException e) {
            System.out.println("Map.of null value -> " + e.getClass().getSimpleName());
        }

        // copyOf / Arrays.asList 对比
        List<String> mutable = new ArrayList<>(Arrays.asList("p", "q"));
        List<String> copy = List.copyOf(mutable);
        mutable.add("r");
        System.out.println("original=" + mutable + " copy=" + copy);

        List<String> asList = Arrays.asList("m", "n");
        asList.set(0, "M");
        System.out.println("Arrays.asList mutable set=" + asList);
        try {
            asList.add("o");
            System.out.println("Arrays.asList add accepted");
        } catch (UnsupportedOperationException e) {
            System.out.println("Arrays.asList add -> " + e.getClass().getSimpleName());
        }

        // 不可变 List 的迭代
        for (String s : fixed) {
            System.out.print(s + "-");
        }
        System.out.println();

        System.out.println("done");
    }
}
