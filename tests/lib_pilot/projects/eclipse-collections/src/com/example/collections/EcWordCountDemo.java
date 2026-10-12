package com.example.collections;

import java.util.LinkedHashMap;
import java.util.List;
import java.util.Map;

import org.eclipse.collections.api.list.MutableList;
import org.eclipse.collections.api.tuple.primitive.ObjectIntPair;
import org.eclipse.collections.impl.factory.Lists;

/**
 * Eclipse Collections 深度解析（二）：词频统计实战（文章收官代码）。
 * JDK 手写 merge + sort vs EC toBag().topOccurrences 两种写法对照。
 * 词表刻意构造为 Top-3 内次数互不相同（java=4, go=3, kotlin=2），
 * 双方输出完全一致即为确定性证明。
 */
public class EcWordCountDemo {

    private static final List<String> WORDS = List.of(
            "java", "go", "java", "kotlin", "java", "go", "scala", "java", "go", "kotlin", "rust");

    /** JDK 写法：merge + entrySet 排序 + 截取前 3。 */
    static List<String> top3ByJdk() {
        Map<String, Integer> counts = new LinkedHashMap<>();
        for (String w : WORDS) {
            counts.merge(w, 1, Integer::sum);
        }
        return counts.entrySet().stream()
                .sorted(Map.Entry.<String, Integer>comparingByValue().reversed())
                .limit(3)
                .map(e -> e.getKey() + "=" + e.getValue())
                .toList();
    }

    /** EC 写法：toBag().topOccurrences(n) 一步取 Top-N。 */
    static List<String> top3ByEc() {
        MutableList<ObjectIntPair<String>> top =
                Lists.mutable.with(WORDS.toArray(new String[0]))
                        .toBag()
                        .topOccurrences(3);
        return top.collect(p -> p.getOne() + "=" + p.getTwo());
    }

    public static void main(String[] args) {
        System.out.println("[JDK 手写]        " + top3ByJdk());
        System.out.println("[EC topOccurrences] " + top3ByEc());
        System.out.println("[对照] 两种写法结果" + (top3ByJdk().equals(top3ByEc()) ? "一致" : "不一致"));
    }
}
