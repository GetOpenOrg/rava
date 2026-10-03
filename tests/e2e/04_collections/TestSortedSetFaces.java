import java.util.Comparator;
import java.util.SortedSet;
import java.util.TreeSet;

/**
 * SortedSet 通道（方法级实测：comparator 7 jar，此前零覆盖）：
 * 自然序 comparator 为 null、定制 comparator 回读、视图族 first/last/headSet/
 * tailSet/subSet。
 */
public class TestSortedSetFaces {

    public static void main(String[] args) {
        TreeSet<String> natural = new TreeSet<>();
        natural.add("c");
        natural.add("a");
        natural.add("b");
        System.out.println("natural-null-comparator=" + (natural.comparator() == null));
        System.out.println("sorted=" + String.join(",", natural));
        System.out.println("first=" + natural.first() + " last=" + natural.last());

        // 定制 comparator 回读（大小写不敏感序）
        TreeSet<String> ci = new TreeSet<>(String.CASE_INSENSITIVE_ORDER);
        ci.add("Bravo");
        ci.add("alpha");
        System.out.println("ci-order=" + String.join(",", ci));
        System.out.println("ci-same-ref=" + (ci.comparator() == String.CASE_INSENSITIVE_ORDER));

        // 视图族
        SortedSet<String> head = natural.headSet("b");      // [a)
        System.out.println("head=" + String.join(",", head) + " size=" + head.size());
        SortedSet<String> tail = natural.tailSet("b");      // [b, c]
        System.out.println("tail=" + String.join(",", tail));
        SortedSet<String> mid = natural.subSet("a", "c");   // [a, c)
        System.out.println("mid=" + String.join(",", mid));

        // 视图联动：向视图添加反映到母集
        head.add("a0");
        System.out.println("view-add-visible=" + natural.contains("a0")
                + " size=" + natural.size());

        // 逆序 comparator + 定制序的 first/last
        TreeSet<Integer> desc = new TreeSet<>(Comparator.reverseOrder());
        desc.add(1);
        desc.add(3);
        desc.add(2);
        System.out.println("desc-first=" + desc.first() + " last=" + desc.last());
    }
}
