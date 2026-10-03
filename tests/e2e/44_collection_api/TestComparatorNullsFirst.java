import java.util.ArrayList;
import java.util.Comparator;
import java.util.List;

/**
 * Comparator 组合族：nullsFirst/nullsLast 空安全、thenComparing 链、reversed、
 * comparing 键提取（guava Ordering 与框架排序的地基，此前零覆盖）。
 */
public class TestComparatorNullsFirst {

    record Item(String name, Integer price) {
    }

    public static void main(String[] args) {
        List<String> names = new ArrayList<>(java.util.Arrays.asList("banana", null, "apple", null, "cherry"));

        names.sort(Comparator.nullsFirst(Comparator.naturalOrder()));
        System.out.println("nulls-first=" + names);

        names.sort(Comparator.nullsLast(Comparator.naturalOrder()));
        System.out.println("nulls-last=" + names);

        List<Item> items = new ArrayList<>(java.util.Arrays.asList(
                new Item("b", 2),
                new Item(null, 1),
                new Item("a", 2),
                new Item("c", 1)));

        // comparing + thenComparing 链（价格升序，同价按名 nullsFirst）
        items.sort(Comparator.comparing(Item::price)
                .thenComparing(Item::name, Comparator.nullsFirst(Comparator.naturalOrder())));
        for (Item i : items) {
            System.out.println("item " + i.name() + "/" + i.price());
        }

        // reversed 反转整个链
        List<Item> desc = new ArrayList<>(items);
        desc.sort(Comparator.comparing(Item::price).reversed());
        System.out.println("desc-first=" + desc.get(0).price());

        // comparingInt 原始特化
        List<Item> byPrice = new ArrayList<>(items);
        byPrice.sort(Comparator.comparingInt(Item::price));
        System.out.println("prices=" + byPrice.stream().map(Item::price).toList());

        // nullsFirst 外层比较 null 元素本身
        List<String> withNull = new ArrayList<>(java.util.Arrays.asList("x", null));
        withNull.sort(Comparator.nullsFirst(Comparator.naturalOrder()));
        System.out.println("null-elem-first=" + (withNull.get(0) == null));
    }
}
