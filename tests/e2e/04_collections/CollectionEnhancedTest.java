import java.util.*;

public class CollectionEnhancedTest {
    public static void main(String[] args) {
        testHashMapEnhanced();
        testTreeMap();
        testOptionalEnhanced();
        testCollectionsUtil();
        testListOf();
        testArrayListEnhanced();
    }

    static void testHashMapEnhanced() {
        System.out.println("=== HashMap Enhanced ===");

        HashMap<String, Integer> map = new HashMap<>();
        map.put("apple", 1);
        map.put("banana", 2);
        map.put("cherry", 3);

        // containsValue
        System.out.println("containsValue 2: " + map.containsValue(2));
        System.out.println("containsValue 99: " + map.containsValue(99));

        // putIfAbsent
        map.putIfAbsent("apple", 100);  // should not change
        map.putIfAbsent("date", 4);     // should add
        System.out.println("apple after putIfAbsent: " + map.get("apple"));
        System.out.println("date after putIfAbsent: " + map.get("date"));

        // keySet
        Set<String> keys = map.keySet();
        System.out.println("keySet size: " + keys.size());

        // values
        Collection<Integer> vals = map.values();
        System.out.println("values size: " + vals.size());

        // entrySet iteration
        int entryCount = 0;
        for (Map.Entry<String, Integer> entry : map.entrySet()) {
            entryCount++;
        }
        System.out.println("entrySet count: " + entryCount);

        System.out.println("size: " + map.size());
    }

    static void testTreeMap() {
        System.out.println("=== TreeMap ===");

        TreeMap<Integer, String> tree = new TreeMap<>();
        tree.put(3, "three");
        tree.put(1, "one");
        tree.put(2, "two");
        tree.put(5, "five");
        tree.put(4, "four");

        System.out.println("size: " + tree.size());
        System.out.println("get 3: " + tree.get(3));
        System.out.println("containsKey 2: " + tree.containsKey(2));
        System.out.println("containsKey 9: " + tree.containsKey(9));
        System.out.println("firstKey: " + tree.firstKey());
        System.out.println("lastKey: " + tree.lastKey());

        tree.remove(3);
        System.out.println("size after remove: " + tree.size());
        System.out.println("isEmpty: " + tree.isEmpty());
    }

    static void testOptionalEnhanced() {
        System.out.println("=== Optional Enhanced ===");

        Optional<Integer> opt = Optional.of(5);

        // map
        Optional<Integer> mapped = opt.map(x -> x * 2);
        System.out.println("map 5*2: " + mapped.get());

        // filter
        Optional<Integer> filtered = opt.filter(x -> x > 3);
        System.out.println("filter >3 present: " + filtered.isPresent());
        Optional<Integer> filtered2 = opt.filter(x -> x > 10);
        System.out.println("filter >10 present: " + filtered2.isPresent());

        // orElseGet
        Optional<Integer> empty = Optional.empty();
        int result = empty.orElseGet(() -> 42);
        System.out.println("orElseGet: " + result);

        // orElse
        int result2 = empty.orElse(99);
        System.out.println("orElse: " + result2);

        // ifPresent
        opt.ifPresent(x -> System.out.println("ifPresent: " + x));
    }

    static void testCollectionsUtil() {
        System.out.println("=== Collections ===");

        ArrayList<Integer> list = new ArrayList<>();
        list.add(3);
        list.add(1);
        list.add(4);
        list.add(1);
        list.add(5);

        // sort
        Collections.sort(list);
        System.out.println("sorted: " + list);

        // reverse
        Collections.reverse(list);
        System.out.println("reversed: " + list);

        // frequency
        System.out.println("frequency of 1: " + Collections.frequency(list, 1));

        // min / max
        System.out.println("min: " + Collections.min(list));
        System.out.println("max: " + Collections.max(list));

        // singletonList
        List<String> single = Collections.singletonList("hello");
        System.out.println("singletonList: " + single);

        // emptyList
        List<Object> emptyL = Collections.emptyList();
        System.out.println("emptyList size: " + emptyL.size());
    }

    static void testListOf() {
        System.out.println("=== Factory Methods ===");

        List<Integer> list = List.of(1, 2, 3);
        System.out.println("List.of: " + list);
        System.out.println("List.of size: " + list.size());

        Set<String> set = Set.of("a", "b", "c");
        System.out.println("Set.of size: " + set.size());
    }

    static void testArrayListEnhanced() {
        System.out.println("=== ArrayList Enhanced ===");

        ArrayList<Integer> list = new ArrayList<>();
        list.add(10);
        list.add(20);
        list.add(30);
        list.add(20);

        // indexOf / lastIndexOf
        System.out.println("indexOf 20: " + list.indexOf(20));
        System.out.println("lastIndexOf 20: " + list.lastIndexOf(20));
        System.out.println("indexOf 99: " + list.indexOf(99));

        // subList
        List<Integer> sub = list.subList(1, 3);
        System.out.println("subList(1,3): " + sub);

        // add(index, element)
        list.add(1, 15);
        System.out.println("after add(1,15): " + list);

        // addAll
        ArrayList<Integer> other = new ArrayList<>();
        other.add(40);
        other.add(50);
        list.addAll(other);
        System.out.println("after addAll: " + list);
        System.out.println("size: " + list.size());
    }
}
