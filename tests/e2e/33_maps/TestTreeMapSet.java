import java.util.ArrayList;
import java.util.Comparator;
import java.util.HashMap;
import java.util.List;
import java.util.Map;
import java.util.Set;
import java.util.TreeMap;
import java.util.TreeSet;

public class TestTreeMapSet {

    public static void main(String[] args) {
        // TreeMap 按 key 自然序
        TreeMap<String, Integer> tm = new TreeMap<>();
        tm.put("delta", 4);
        tm.put("alpha", 1);
        tm.put("charlie", 3);
        tm.put("bravo", 2);
        System.out.println("firstKey=" + tm.firstKey());
        System.out.println("lastKey=" + tm.lastKey());
        System.out.println("size=" + tm.size());
        System.out.println("keys in order=" + tm.keySet());
        System.out.println("values=" + tm.values());

        // 导航方法
        System.out.println("lower(charlie)=" + tm.lowerKey("charlie"));
        System.out.println("floor(charlie)=" + tm.floorKey("charlie"));
        System.out.println("higher(charlie)=" + tm.higherKey("charlie"));
        System.out.println("ceiling(charlie)=" + tm.ceilingKey("charlie"));

        // 视图
        System.out.println("headMap(charlie)=" + tm.headMap("charlie").keySet());
        System.out.println("tailMap(charlie)=" + tm.tailMap("charlie").keySet());
        System.out.println("subMap(bravo,delta)=" + tm.subMap("bravo", "delta").keySet());

        // pollFirst / pollLast
        Map.Entry<String, Integer> first = tm.pollFirstEntry();
        System.out.println("pollFirst=" + first.getKey() + "->" + first.getValue() + " size=" + tm.size());
        Map.Entry<String, Integer> last = tm.pollLastEntry();
        System.out.println("pollLast=" + last.getKey() + "->" + last.getValue() + " size=" + tm.size());

        // 逆序 Comparator
        Comparator<Integer> desc = (x, y) -> Integer.compare(y, x);
        TreeMap<Integer, String> reversed = new TreeMap<>(desc);
        reversed.put(1, "one");
        reversed.put(3, "three");
        reversed.put(2, "two");
        System.out.println("descending keys=" + reversed.keySet());
        System.out.println("descending first=" + reversed.firstKey());

        // TreeSet
        TreeSet<Integer> ts = new TreeSet<>();
        int[] inputs = {5, 1, 9, 3, 5, 7};
        for (int v : inputs) {
            ts.add(v);
        }
        System.out.println("treeset=" + ts + " size=" + ts.size());
        System.out.println("first=" + ts.first() + " last=" + ts.last());
        System.out.println("lower(5)=" + ts.lower(5) + " higher(5)=" + ts.higher(5));
        System.out.println("ceiling(6)=" + ts.ceiling(6) + " floor(6)=" + ts.floor(6));
        System.out.println("headSet(5)=" + ts.headSet(5));
        System.out.println("tailSet(5)=" + ts.tailSet(5));
        System.out.println("subSet(3,7)=" + ts.subSet(3, 7));
        System.out.println("pollFirst=" + ts.pollFirst() + " pollLast=" + ts.pollLast());
        System.out.println("after poll=" + ts);

        // Comparator.comparing 构造 TreeSet
        TreeSet<String> byLength = new TreeSet<>(Comparator.comparing(String::length).thenComparing(s -> s));
        byLength.add("ccc");
        byLength.add("a");
        byLength.add("bb");
        byLength.add("aa");
        System.out.println("byLength=" + byLength);

        // TreeMap 与 HashMap 内容互换
        Map<String, Integer> hash = new HashMap<>();
        hash.put("x", 10);
        hash.put("y", 20);
        TreeMap<String, Integer> copied = new TreeMap<>(hash);
        System.out.println("copied keys=" + copied.keySet());

        List<String> names = new ArrayList<>(tm.keySet());
        System.out.println("names=" + names);

        Set<Integer> intSet = ts.headSet(3, true);
        System.out.println("headSet(3,true)=" + intSet);

        System.out.println("done");
    }
}
