package com.example.guava;

import java.util.List;
import java.util.TreeSet;

import com.google.common.base.Joiner;
import com.google.common.base.Preconditions;
import com.google.common.base.Splitter;
import com.google.common.collect.BiMap;
import com.google.common.collect.HashBiMap;
import com.google.common.collect.HashMultiset;
import com.google.common.collect.ImmutableList;
import com.google.common.collect.ImmutableMap;
import com.google.common.collect.ImmutableSet;
import com.google.common.collect.Multiset;
import com.google.common.collect.Table;
import com.google.common.collect.TreeBasedTable;

/**
 * Google Guava 核心工具演示（对应文章模块清单）：
 * 不可变集合、Multiset/BiMap/Table 扩展集合、Splitter/Joiner、Preconditions。
 *
 * 输出确定性说明：固定数据；HashMultiset 本身无序，打印前统一按元素排序。
 */
public class GuavaCollectionsDemo {

    public static void main(String[] args) {
        // ---------- 1. 不可变集合（声明即常量，拒绝 null） ----------
        ImmutableList<String> names = ImmutableList.of("alice", "bob", "carol");
        ImmutableSet<Integer> codes = ImmutableSet.of(1, 2, 3);
        ImmutableMap<String, Integer> scores = ImmutableMap.of("alice", 95, "bob", 88);
        System.out.println("[Immutable] " + names + " " + codes + " " + scores);
        try {
            names.add("dave"); // 不可变集合 add 直接抛异常
        } catch (UnsupportedOperationException e) {
            System.out.println("[Immutable] add 被拒绝: UnsupportedOperationException");
        }

        // ---------- 2. Multiset：词频统计（打印前按元素排序保证顺序） ----------
        Multiset<String> words = HashMultiset.create(
                List.of("java", "go", "java", "rust", "java", "go"));
        System.out.println("[Multiset] java=" + words.count("java")
                + ", go=" + words.count("go")
                + ", rust=" + words.count("rust")
                + ", 排序元素集=" + new TreeSet<>(words.elementSet()));

        // ---------- 3. BiMap：双向映射，值也唯一 ----------
        BiMap<String, Integer> idByName = HashBiMap.create();
        idByName.put("alice", 1);
        idByName.put("bob", 2);
        System.out.println("[BiMap] alice->" + idByName.get("alice")
                + ", 2->" + idByName.inverse().get(2));
        try {
            idByName.put("carol", 2); // 值冲突，inverse 唯一性被破坏
        } catch (IllegalArgumentException e) {
            System.out.println("[BiMap] 值重复被拒绝: IllegalArgumentException");
        }

        // ---------- 4. Table：双键表格（行=学生, 列=科目） ----------
        Table<String, String, Integer> gradeTable = TreeBasedTable.create(); // 行列都按自然序
        gradeTable.put("张三", "数学", 92);
        gradeTable.put("张三", "语文", 88);
        gradeTable.put("李四", "数学", 79);
        System.out.println("[Table] 张三成绩=" + gradeTable.row("张三")
                + ", 数学列=" + gradeTable.column("数学"));

        // ---------- 5. Splitter / Joiner ----------
        List<String> parts = Splitter.on(',')
                .trimResults()
                .omitEmptyStrings()
                .splitToList(" alice , bob ,, carol ,");
        System.out.println("[Splitter] " + parts);
        System.out.println("[Joiner]   " + Joiner.on(" | ").join(parts));

        // ---------- 6. Preconditions：一句话参数校验 ----------
        try {
            Preconditions.checkArgument(18 > 60, "年龄 %s 不合法", 18);
        } catch (IllegalArgumentException e) {
            System.out.println("[Preconditions] " + e.getMessage());
        }
        System.out.println("[Preconditions] checkNotNull(\"x\") = " + Preconditions.checkNotNull("x"));

        // ---------- 7. BloomFilter：固定参数下对固定输入结果恒定，只打印布尔 ----------
        com.google.common.hash.BloomFilter<String> bloom = com.google.common.hash.BloomFilter.create(
                com.google.common.hash.Funnels.unencodedCharsFunnel(), 100, 0.01);
        bloom.put("alice");
        bloom.put("bob");
        System.out.println("[BloomFilter] alice=" + bloom.mightContain("alice")
                + ", bob=" + bloom.mightContain("bob")
                + ", carol=" + bloom.mightContain("carol") + "（可能误判，参数固定则结果恒定）");
    }
}
