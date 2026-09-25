import java.util.HashSet;

public class HashSetTest {
    public static void main(String[] args) {
        HashSet<String> set = new HashSet<>();
        set.add("apple");
        set.add("banana");
        set.add("apple");

        System.out.println(set.size());
        System.out.println(set.contains("apple"));
        System.out.println(set.contains("cherry"));

        set.remove("banana");
        System.out.println(set.size());
        System.out.println(set.isEmpty());
    }
}
