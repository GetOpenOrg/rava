import java.util.ArrayList;
import java.util.Collections;
import java.util.HashSet;
import java.util.List;
import java.util.Set;

class SimpleKey {
    final int id;
    final String tag;

    SimpleKey(int id, String tag) {
        this.id = id;
        this.tag = tag;
    }

    @Override
    public boolean equals(Object o) {
        if (!(o instanceof SimpleKey)) return false;
        SimpleKey other = (SimpleKey) o;
        return id == other.id && tag.equals(other.tag);
    }

    @Override
    public int hashCode() {
        return id * 31 + tag.hashCode();
    }

    @Override
    public String toString() {
        return "Key(" + id + "," + tag + ")";
    }
}

public class TestHashSetOps {

    static void dump(Set<String> set, String tag) {
        List<String> sorted = new ArrayList<>(set);
        Collections.sort(sorted);
        System.out.println(tag + sorted);
    }

    public static void main(String[] args) {
        Set<String> names = new HashSet<>();
        names.add("alice");
        names.add("bob");
        names.add("alice");
        names.add("carol");
        System.out.println("size=" + names.size());
        System.out.println("contains alice=" + names.contains("alice"));
        System.out.println("contains dave=" + names.contains("dave"));
        System.out.println("remove bob=" + names.remove("bob"));
        System.out.println("remove bob again=" + names.remove("bob"));
        dump(names, "names=");

        // 自定义 equals/hashCode 决定去重
        Set<SimpleKey> keys = new HashSet<>();
        keys.add(new SimpleKey(1, "a"));
        keys.add(new SimpleKey(1, "a"));
        keys.add(new SimpleKey(1, "b"));
        keys.add(new SimpleKey(2, "a"));
        System.out.println("keys size=" + keys.size());
        System.out.println("has(1,a)=" + keys.contains(new SimpleKey(1, "a")));
        System.out.println("has(9,z)=" + keys.contains(new SimpleKey(9, "z")));

        // 集合运算（仅用 removeAll/retainAll/addAll/containsAll）
        Set<String> a = new HashSet<>();
        a.add("x");
        a.add("y");
        a.add("z");
        Set<String> b = new HashSet<>();
        b.add("y");
        b.add("z");
        b.add("w");
        System.out.println("containsAll b=" + a.containsAll(b));
        Set<String> union = new HashSet<>(a);
        union.addAll(b);
        dump(union, "union=");
        Set<String> inter = new HashSet<>(a);
        inter.retainAll(b);
        dump(inter, "intersection=");
        Set<String> diff = new HashSet<>(a);
        diff.removeAll(b);
        dump(diff, "difference=");

        // 空集与 clear
        Set<String> empty = new HashSet<>();
        System.out.println("empty isEmpty=" + empty.isEmpty() + " size=" + empty.size());
        names.clear();
        System.out.println("names cleared isEmpty=" + names.isEmpty());

        // null 元素允许
        Set<String> withNull = new HashSet<>();
        withNull.add(null);
        withNull.add("x");
        System.out.println("withNull size=" + withNull.size() + " hasNull=" + withNull.contains(null));

        System.out.println("done");
    }
}
