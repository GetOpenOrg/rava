import java.util.*;

public class CollectionsUtils {
    public static void main(String[] args) {
        // frequency
        List<String> list = new ArrayList<>();
        list.add("a"); list.add("b"); list.add("a"); list.add("c"); list.add("a");
        System.out.println(Collections.frequency(list, "a")); // 3

        // nCopies
        List<String> copies = Collections.nCopies(4, "x");
        System.out.println(copies.size()); // 4
        System.out.println(copies.get(0)); // x

        // emptyList
        List<String> empty = Collections.emptyList();
        System.out.println(empty.size()); // 0

        // sort
        List<Integer> nums = new ArrayList<>();
        nums.add(3); nums.add(1); nums.add(2);
        Collections.sort(nums);
        System.out.println(nums.get(0)); // 1

        // max / min
        System.out.println(Collections.max(nums)); // 3
        System.out.println(Collections.min(nums)); // 1
    }
}
