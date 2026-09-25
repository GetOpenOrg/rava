import java.util.TreeMap;
import java.util.TreeSet;
import java.util.Map;
import java.util.Set;
import java.util.ArrayList;
import java.util.List;
import java.util.LinkedHashMap;
import java.util.PriorityQueue;

public class SortedCollectionTest {
    public static void main(String[] args) {
        // 1. TreeMap (sorted by key)
        TreeMap<String, Integer> tm = new TreeMap<>();
        tm.put("Charlie", 3);
        tm.put("Alice", 1);
        tm.put("Bob", 2);
        System.out.println("TreeMap: " + tm);
        System.out.println("firstKey: " + tm.firstKey());
        System.out.println("lastKey: " + tm.lastKey());
        System.out.println("containsKey Bob: " + tm.containsKey("Bob"));
        System.out.println("size: " + tm.size());

        // 2. TreeMap iteration (should be sorted)
        List<String> tmKeys = new ArrayList<>();
        for (Map.Entry<String, Integer> e : tm.entrySet()) {
            tmKeys.add(e.getKey() + "=" + e.getValue());
        }
        System.out.println("entries: " + tmKeys);

        // 3. TreeSet (sorted)
        TreeSet<Integer> ts = new TreeSet<>();
        ts.add(30);
        ts.add(10);
        ts.add(20);
        ts.add(10); // duplicate
        System.out.println("TreeSet size: " + ts.size());
        System.out.println("TreeSet first: " + ts.first());
        System.out.println("TreeSet last: " + ts.last());
        System.out.println("TreeSet contains 20: " + ts.contains(20));
        List<Integer> tsItems = new ArrayList<>();
        for (int x : ts) {
            tsItems.add(x);
        }
        System.out.println("TreeSet items: " + tsItems);

        // 4. LinkedHashMap (insertion order)
        LinkedHashMap<String, Integer> lhm = new LinkedHashMap<>();
        lhm.put("first", 1);
        lhm.put("second", 2);
        lhm.put("third", 3);
        List<String> lhmOrder = new ArrayList<>();
        for (Map.Entry<String, Integer> e : lhm.entrySet()) {
            lhmOrder.add(e.getKey());
        }
        System.out.println("LinkedHashMap order: " + lhmOrder);
        System.out.println("get second: " + lhm.get("second"));

        // 5. PriorityQueue (min-heap)
        PriorityQueue<Integer> pq = new PriorityQueue<>();
        pq.add(30);
        pq.add(10);
        pq.add(20);
        System.out.println("PQ peek: " + pq.peek());
        System.out.println("PQ poll: " + pq.poll());
        System.out.println("PQ poll: " + pq.poll());
        System.out.println("PQ poll: " + pq.poll());
        System.out.println("PQ empty: " + pq.isEmpty());

        System.out.println("Done.");
    }
}
