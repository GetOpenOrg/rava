import java.util.LinkedHashSet;
import java.util.Iterator;

public class LinkedHashSetDemo {
    public static void main(String[] args) {
        LinkedHashSet<String> set = new LinkedHashSet<>();
        set.add("banana");
        set.add("apple");
        set.add("cherry");
        set.add("apple"); // duplicate — should be ignored

        System.out.println(set.size());
        System.out.println(set.contains("apple"));
        System.out.println(set.contains("mango"));

        // Insertion order preserved
        for (String s : set) {
            System.out.println(s);
        }

        // remove
        set.remove("apple");
        System.out.println(set.size());
        System.out.println(set.contains("apple"));

        // addAll
        LinkedHashSet<Integer> nums = new LinkedHashSet<>();
        for (int i = 5; i >= 1; i--) {
            nums.add(i);
        }
        System.out.println(nums.size());
        for (int n : nums) {
            System.out.print(n + " ");
        }
        System.out.println();

        // isEmpty / clear
        LinkedHashSet<String> empty = new LinkedHashSet<>();
        System.out.println(empty.isEmpty());
        empty.add("x");
        System.out.println(empty.isEmpty());
        empty.clear();
        System.out.println(empty.isEmpty());

        System.out.println("done");
    }
}
