import java.util.ArrayList;
import java.util.Arrays;
import java.util.Collections;
import java.util.HashSet;
import java.util.List;

public class SetOperationsTest {
    public static void main(String[] args) {
        // 1. Build sets manually
        HashSet<Integer> setA = new HashSet<>();
        setA.add(1); setA.add(2); setA.add(3); setA.add(4); setA.add(5);

        HashSet<Integer> setB = new HashSet<>();
        setB.add(3); setB.add(4); setB.add(5); setB.add(6); setB.add(7);

        // 2. Intersection via retainAll (on a copy)
        HashSet<Integer> intersection = new HashSet<>();
        for (Object x : setA) intersection.add((Integer) x);
        intersection.retainAll(setB);
        List<Integer> interList = new ArrayList<>();
        for (Object x : intersection) interList.add((Integer) x);
        Collections.sort(interList);
        System.out.println("intersection: " + interList);

        // 3. Union via addAll (on a copy)
        HashSet<Integer> union = new HashSet<>();
        for (Object x : setA) union.add((Integer) x);
        union.addAll(setB);
        List<Integer> unionList = new ArrayList<>();
        for (Object x : union) unionList.add((Integer) x);
        Collections.sort(unionList);
        System.out.println("union: " + unionList);

        // 4. Difference via removeAll (on a copy)
        HashSet<Integer> diff = new HashSet<>();
        for (Object x : setA) diff.add((Integer) x);
        diff.removeAll(setB);
        List<Integer> diffList = new ArrayList<>();
        for (Object x : diff) diffList.add((Integer) x);
        Collections.sort(diffList);
        System.out.println("difference: " + diffList);

        // 5. containsAll
        HashSet<Integer> subset = new HashSet<>();
        subset.add(2); subset.add(3);
        System.out.println("A containsAll {2,3}: " + setA.containsAll(subset));
        subset.add(6);
        System.out.println("A containsAll {2,3,6}: " + setA.containsAll(subset));

        // 6. Unique names from ArrayList
        List<String> names = new ArrayList<>(Arrays.asList("Alice", "Bob", "Alice", "Charlie", "Bob"));
        HashSet<String> uniqueNames = new HashSet<>();
        for (String n : names) uniqueNames.add(n);
        System.out.println("unique count: " + uniqueNames.size());

        // 7. forEach on set
        List<String> collected = new ArrayList<>();
        uniqueNames.forEach(n -> collected.add(n.toString()));
        Collections.sort(collected);
        System.out.println("forEach: " + collected);

        // 8. Set isEmpty / clear
        System.out.println("empty: " + uniqueNames.isEmpty());
        uniqueNames.clear();
        System.out.println("after clear empty: " + uniqueNames.isEmpty());

        System.out.println("Done.");
    }
}
