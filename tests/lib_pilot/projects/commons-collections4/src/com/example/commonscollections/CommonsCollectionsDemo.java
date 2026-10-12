package com.example.commonscollections;

import java.util.List;
import java.util.Set;
import java.util.TreeSet;

import org.apache.commons.collections4.Bag;
import org.apache.commons.collections4.BidiMap;
import org.apache.commons.collections4.MultiValuedMap;
import org.apache.commons.collections4.SetUtils;
import org.apache.commons.collections4.bag.HashBag;
import org.apache.commons.collections4.bidimap.DualHashBidiMap;
import org.apache.commons.collections4.collection.PredicatedCollection;
import org.apache.commons.collections4.collection.TransformedCollection;
import org.apache.commons.collections4.list.FixedSizeList;
import org.apache.commons.collections4.map.MultiValueMap;

/**
 * Apache Commons Collections 4 增强 JDK 集合（对应文章的工具类/数据结构/装饰器清单）。
 *
 * 输出确定性说明：Bag/MultiValuedMap 内部无序，打印前统一用 TreeSet 排序。
 */
public class CommonsCollectionsDemo {

    public static void main(String[] args) {
        // ---------- 1. Bag：词频统计（JDK 没有） ----------
        Bag<String> bag = new HashBag<>(
                List.of("java", "go", "java", "rust", "java", "go"));
        System.out.println("[Bag] java=" + bag.getCount("java")
                + ", go=" + bag.getCount("go")
                + ", uniqueSorted=" + new TreeSet<>(bag.uniqueSet()));

        // ---------- 2. BidiMap：双向映射 ----------
        BidiMap<String, Integer> bidi = new DualHashBidiMap<>();
        bidi.put("alice", 1);
        bidi.put("bob", 2);
        System.out.println("[BidiMap] alice->" + bidi.get("alice")
                + ", 2->" + bidi.inverseBidiMap().get(2));

        // ---------- 3. MultiValuedMap：一键多值（ArrayListValued 保持插入顺序） ----------
        MultiValuedMap<String, String> multi = new org.apache.commons.collections4.multimap.ArrayListValuedHashMap<>();
        multi.put("fruits", "apple");
        multi.put("fruits", "banana");
        multi.put("fruits", "cherry");
        multi.put("vegs", "potato");
        System.out.println("[MultiValuedMap] fruits=" + multi.get("fruits")
                + ", keys=" + new TreeSet<>(multi.keySet()));

        // ---------- 4. FixedSizeList：固定尺寸装饰器 ----------
        FixedSizeList<String> fixed = FixedSizeList.fixedSizeList(
                new java.util.ArrayList<>(List.of("a", "b", "c")));
        fixed.set(0, "A"); // set 允许
        System.out.println("[FixedSizeList] set(0,'A') 后 = " + fixed);
        try {
            fixed.add("d"); // add 拒绝
        } catch (UnsupportedOperationException e) {
            System.out.println("[FixedSizeList] add 被拒绝: UnsupportedOperationException");
        }

        // ---------- 5. PredicatedCollection：谓词校验装饰器（只接受偶数） ----------
        PredicatedCollection<Integer> evens = PredicatedCollection.predicatedCollection(
                new java.util.ArrayList<>(),
                n -> n % 2 == 0);
        evens.add(2);
        evens.add(4);
        try {
            evens.add(5); // 违反谓词
        } catch (IllegalArgumentException e) {
            System.out.println("[Predicated] add(5) 被拒绝: IllegalArgumentException（只允许偶数）");
        }
        System.out.println("[Predicated] 当前元素 = " + evens);

        // ---------- 6. TransformedCollection：入队自动转换（转大写） ----------
        java.util.Collection<String> upper = TransformedCollection.transformedCollection(
                new java.util.ArrayList<>(), String::toUpperCase);
        upper.add("hello");
        upper.add("world");
        System.out.println("[Transformed] add 后自动转大写 = " + upper);

        // ---------- 7. SetUtils 集合运算（LinkedHashSet 保序输入） ----------
        Set<Integer> s1 = new java.util.LinkedHashSet<>(List.of(1, 2, 3, 4));
        Set<Integer> s2 = new java.util.LinkedHashSet<>(List.of(3, 4, 5, 6));
        System.out.println("[SetUtils] 交集=" + SetUtils.intersection(s1, s2)
                + ", 并集=" + new TreeSet<>(SetUtils.union(s1, s2))
                + ", s1-s2 差集=" + SetUtils.difference(s1, s2));
    }
}
