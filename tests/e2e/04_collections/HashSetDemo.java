import java.util.HashSet;
import java.util.ArrayList;

public class HashSetDemo {
    public static void main(String[] args) {
        HashSet<String> set = new HashSet<>();
        set.add("apple");
        set.add("banana");
        set.add("apple"); // 重复，不加入
        set.add("cherry");

        System.out.println(set.size());           // 3
        System.out.println(set.contains("banana")); // true
        System.out.println(set.contains("mango")); // false

        set.remove("banana");
        System.out.println(set.size());           // 2
        System.out.println(set.isEmpty());        // false

        // addAll from ArrayList
        ArrayList<String> extra = new ArrayList<>();
        extra.add("date");
        extra.add("apple"); // 重复
        set.addAll(extra);
        System.out.println(set.size());           // 3

        // 清空
        set.clear();
        System.out.println(set.isEmpty());        // true
    }
}
