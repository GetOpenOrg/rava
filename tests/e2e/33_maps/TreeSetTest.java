import java.util.TreeSet;
import java.util.Iterator;

/**
 * Tests java.util.TreeSet with natural ordering.
 */
public class TreeSetTest {
    public static void main(String[] args) {
        // Test 1: Basic sorted insertion
        TreeSet<Integer> nums = new TreeSet<>();
        nums.add(5);
        nums.add(1);
        nums.add(3);
        nums.add(2);
        nums.add(4);
        System.out.println("sorted: " + nums);

        // Test 2: Duplicates rejected
        boolean added = nums.add(3);
        System.out.println("add duplicate: " + added);
        System.out.println("size: " + nums.size());

        // Test 3: first/last
        System.out.println("first: " + nums.first());
        System.out.println("last: " + nums.last());

        // Test 4: contains
        System.out.println("contains 3: " + nums.contains(3));
        System.out.println("contains 9: " + nums.contains(9));

        // Test 5: remove
        boolean removed = nums.remove(3);
        System.out.println("removed 3: " + removed);
        System.out.println("after remove: " + nums);

        // Test 6: pollFirst/pollLast
        TreeSet<String> strs = new TreeSet<>();
        strs.add("cherry");
        strs.add("apple");
        strs.add("banana");
        System.out.println("strings: " + strs);
        System.out.println("pollFirst: " + strs.pollFirst());
        System.out.println("pollLast: " + strs.pollLast());
        System.out.println("remaining: " + strs);

        // Test 7: isEmpty/clear
        System.out.println("isEmpty: " + strs.isEmpty());
        strs.clear();
        System.out.println("after clear isEmpty: " + strs.isEmpty());

        // Test 8: Iterator
        TreeSet<Integer> iter_set = new TreeSet<>();
        iter_set.add(30);
        iter_set.add(10);
        iter_set.add(20);
        StringBuilder sb = new StringBuilder();
        Iterator<Integer> it = iter_set.iterator();
        while (it.hasNext()) {
            if (sb.length() > 0) sb.append(",");
            sb.append(it.next());
        }
        System.out.println("iterator: " + sb.toString());
    }
}
