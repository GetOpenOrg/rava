import java.util.Iterator;
import java.util.LinkedHashMap;
import java.util.Map;

/**
 * LinkedHashMap 访问序模式与 removeEldestEntry 逐出（mybatis L2 缓存与各类 LRU
 * 的 JDK 原型，e2e 此前零覆盖）：get 触发重排、插入触发逐出、访问序 vs 插入序差异。
 */
public class TestLinkedHashMapLru {

    static class BoundedLru<K, V> extends LinkedHashMap<K, V> {
        final int cap;

        BoundedLru(int cap) {
            super(16, 0.75f, true);   // accessOrder = true
            this.cap = cap;
        }

        @Override
        protected boolean removeEldestEntry(Map.Entry<K, V> eldest) {
            return size() > cap;
        }
    }

    public static void main(String[] args) {
        // 插入序（默认）：迭代顺序 = 插入顺序
        LinkedHashMap<String, Integer> ins = new LinkedHashMap<>();
        ins.put("a", 1);
        ins.put("b", 2);
        ins.put("c", 3);
        System.out.println("insertion=" + ins.keySet());

        // 访问序：get 推到尾部
        LinkedHashMap<String, Integer> acc = new LinkedHashMap<>(16, 0.75f, true);
        acc.put("a", 1);
        acc.put("b", 2);
        acc.put("c", 3);
        acc.get("a");
        System.out.println("after-get=" + acc.keySet());
        acc.getOrDefault("b", 0);
        System.out.println("after-getOrDefault=" + acc.keySet());
        // put 已有键也触发访问
        acc.put("c", 33);
        System.out.println("after-reput=" + acc.keySet());

        // removeEldestEntry 逐出：容量 2，插第三个时逐出最久未访问
        BoundedLru<String, String> lru = new BoundedLru<>(2);
        lru.put("k1", "v1");
        lru.put("k2", "v2");
        lru.get("k1");                    // k2 变最旧
        lru.put("k3", "v3");              // 逐出 k2
        System.out.println("lru-keys=" + lru.keySet());
        System.out.println("k1-present=" + lru.containsKey("k1"));
        System.out.println("k2-evicted=" + !lru.containsKey("k2"));

        // 迭代器顺序 = 当前访问序快照
        Iterator<String> it = lru.keySet().iterator();
        StringBuilder order = new StringBuilder();
        while (it.hasNext()) {
            order.append(it.next());
        }
        System.out.println("iterate=" + order);
    }
}
