package com.example.collections;

import java.util.ArrayList;

import org.eclipse.collections.api.bimap.MutableBiMap;
import org.eclipse.collections.api.multimap.list.MutableListMultimap;
import org.eclipse.collections.impl.bimap.mutable.HashBiMap;
import org.eclipse.collections.impl.factory.Multimaps;
import org.eclipse.collections.impl.factory.Lists;

/**
 * Eclipse Collections 深度解析（三）：专用容器 + JDK 集合适配
 * （对应文章 Multimap / BiMap / Lists.adapt 代码段）。
 */
public class EcContainersDemo {

    public static void main(String[] args) {
        // ---------- 1. Multimap：一键多值（JDK 需要 Map<K, List<V>> 手维护） ----------
        MutableListMultimap<String, String> multimap = Multimaps.mutable.list.empty();
        multimap.put("fruits", "apple");
        multimap.put("fruits", "banana");
        multimap.put("fruits", "cherry");
        multimap.put("vegs", "potato");
        System.out.println("[Multimap] fruits=" + multimap.get("fruits")
                + " (view类型=" + multimap.get("fruits").getClass().getSimpleName() + ")");
        System.out.println("[Multimap] keySet=" + multimap.keySet());

        // ---------- 2. BiMap：值也唯一，可反查 ----------
        MutableBiMap<String, Integer> biMap = new HashBiMap<>();
        biMap.put("alice", 1);
        biMap.put("bob", 2);
        System.out.println("[BiMap] alice->" + biMap.get("alice")
                + ", inverse: 2->" + biMap.inverse().get(2));
        try {
            biMap.put("carol", 2); // 值重复：inverse 唯一性被破坏
        } catch (IllegalArgumentException e) {
            System.out.println("[BiMap] 值重复被拒绝: IllegalArgumentException");
        }

        // ---------- 3. Lists.adapt：现有 JDK 集合即刻获得 EC 全套 API ----------
        java.util.List<Integer> jdkList = new ArrayList<>(java.util.List.of(5, 3, 8, 1));
        var adapted = Lists.adapt(jdkList);
        System.out.println("[adapt] select偶数=" + adapted.select(n -> n % 2 == 0)
                + " toSortedList=" + adapted.toSortedList()
                + " makeString=" + adapted.makeString("/"));
        // 适配是视图语义：adapt 上的修改写回原集合
        adapted.add(99);
        System.out.println("[adapt] 添加后原 JDK List = " + jdkList);
    }
}
