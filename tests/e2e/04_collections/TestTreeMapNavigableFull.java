import java.util.NavigableMap;
import java.util.TreeMap;

/**
 * TreeMap 导航族收口：descendingMap / descendingKeySet 视图、firstKey/lastKey、
 * pollFirstEntry/pollLastEntry（floorKey/ceilingKey/headMap/tailMap/subMap 已有他例，
 * 本例补齐族内最后一块——guava TreeBasedTable / spring 有序映射的地基）。
 */
public class TestTreeMapNavigableFull {

    public static void main(String[] args) {
        TreeMap<String, Integer> m = new TreeMap<>();
        m.put("a", 1);
        m.put("c", 3);
        m.put("e", 5);
        m.put("g", 7);

        System.out.println("first=" + m.firstKey() + " last=" + m.lastKey());

        // 降序视图：顺序反转、与原Map联动（视图非拷贝）
        NavigableMap<String, Integer> desc = m.descendingMap();
        System.out.println("desc-keys=" + desc.keySet());
        m.put("d", 4);
        System.out.println("desc-sees-insert=" + desc.containsKey("d"));
        System.out.println("desc-first=" + desc.firstEntry());

        System.out.println("desc-keyset=" + m.descendingKeySet().toArray().length);

        // poll 端点：取走并移除
        System.out.println("poll-first=" + m.pollFirstEntry());
        System.out.println("poll-last=" + m.pollLastEntry());
        System.out.println("after-poll=" + m.keySet());

        // 空表边界
        TreeMap<String, Integer> empty = new TreeMap<>();
        System.out.println("empty-poll=" + empty.pollFirstEntry());
        System.out.println("empty-poll-null=" + (empty.pollLastEntry() == null));
        try {
            empty.firstKey();
        } catch (java.util.NoSuchElementException e) {
            System.out.println("empty-first-ex=" + e.getClass().getSimpleName());
        }
    }
}
