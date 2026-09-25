import java.util.TreeSet;

public class TreeSetDemo {
    public static void main(String[] args) {
        TreeSet<Integer> set = new TreeSet<>();
        set.add(5);
        set.add(2);
        set.add(8);
        set.add(2); // 重复，不加入
        set.add(1);

        System.out.println(set.size());       // 4
        System.out.println(set.first());      // 1
        System.out.println(set.last());       // 8
        System.out.println(set.contains(5));  // true
        System.out.println(set.contains(9));  // false

        System.out.println(set.floor(4));     // 2
        System.out.println(set.ceiling(3));   // 5
        System.out.println(set.higher(5));    // 8
        System.out.println(set.lower(5));     // 2

        set.remove(2);
        System.out.println(set.size());       // 3
        System.out.println(set.isEmpty());    // false

        // 字符串 TreeSet
        TreeSet<String> words = new TreeSet<>();
        words.add("banana");
        words.add("apple");
        words.add("cherry");
        System.out.println(words.first());    // apple
        System.out.println(words.last());     // cherry
    }
}
