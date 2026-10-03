import java.util.Iterator;
import java.util.concurrent.CopyOnWriteArrayList;

/**
 * CopyOnWriteArrayList 快照语义（servlet 监听器列表 / guava 并发集的地基）：
 * 迭代器是创建时快照——迭代中修改不抛 CME、旧迭代器不见新元素；
 * 迭代器自身 remove 不支持；addIfAbsent 去重。
 */
public class TestCopyOnWriteList {

    public static void main(String[] args) {
        CopyOnWriteArrayList<String> list = new CopyOnWriteArrayList<>();
        list.add("a");
        list.add("b");

        // 迭代中修改：不 CME，旧迭代器保持快照
        Iterator<String> it = list.iterator();
        list.add("c");
        StringBuilder snap = new StringBuilder();
        while (it.hasNext()) {
            snap.append(it.next());
        }
        System.out.println("snapshot=" + snap);
        System.out.println("live-size=" + list.size());

        // 新迭代器见到新元素
        StringBuilder fresh = new StringBuilder();
        for (String s : list) {
            fresh.append(s);
        }
        System.out.println("fresh=" + fresh);

        // 迭代器 remove 不支持（快照语义）
        Iterator<String> it2 = list.iterator();
        it2.next();
        try {
            it2.remove();
        } catch (UnsupportedOperationException e) {
            System.out.println("iter-remove-ex=" + e.getClass().getSimpleName());
        }

        // addIfAbsent 去重
        System.out.println("absent-added=" + list.addIfAbsent("d"));
        System.out.println("absent-skipped=" + list.addIfAbsent("a"));
        System.out.println("size=" + list.size());

        // 删除后迭代器快照不受影响
        Iterator<String> before = list.iterator();
        list.remove("a");
        StringBuilder kept = new StringBuilder();
        before.forEachRemaining(kept::append);
        System.out.println("kept-snapshot=" + kept);
        System.out.println("final=" + String.join(",", list));
    }
}
