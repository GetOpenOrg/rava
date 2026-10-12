package com.example.collections;

import java.util.List;
import java.util.stream.Collectors;

import org.eclipse.collections.api.list.MutableList;
import org.eclipse.collections.api.list.primitive.IntList;
import org.eclipse.collections.api.bag.Bag;
import org.eclipse.collections.api.list.ImmutableList;
import org.eclipse.collections.impl.factory.Bags;
import org.eclipse.collections.impl.factory.Lists;
import org.eclipse.collections.impl.factory.primitive.IntLists;

/**
 * Eclipse Collections（原 GS Collections，高盛开源）五大卖点示例
 * （对应文章 §二"5个让你放弃ArrayList的理由"）：
 * 1. IntList 原始类型集合零装箱  2. select/collect 链式 API
 * 3. Bag 多重集直接计数          4. 真不可变集合
 * 5. 与 JDK Stream 的写法对比
 *
 * 输出确定性说明：数据集固定，collect/select 保序，无并行环节。
 */
public class EclipseCollectionsDemo {

    static final class Person {
        private final String name;
        private final int age;

        Person(String name, int age) {
            this.name = name;
            this.age = age;
        }

        String name() {
            return name;
        }

        int age() {
            return age;
        }

        @Override
        public String toString() {
            return name + "(" + age + ")";
        }
    }

    public static void main(String[] args) {
        // ---------- 理由1：IntList 原始类型集合，零装箱 ----------
        IntList intList = IntLists.mutable.with(10, 20, 30, 40, 50);
        System.out.println("[IntList] " + intList + ", sum=" + intList.sum()
                + ", max=" + intList.max() + "（元素直接以 int 存储，无 Integer 装箱）");

        // ---------- 理由2：select/collect 链式调用 vs JDK Stream ----------
        MutableList<Person> people = Lists.mutable.with(
                new Person("张三", 17), new Person("李四", 22),
                new Person("王五", 30), new Person("赵六", 15));

        // JDK Stream：过滤+映射+收集，3 步
        List<String> jdkNames = people.stream()
                .filter(p -> p.age() >= 18)
                .map(Person::name)
                .collect(Collectors.toList());
        System.out.println("[JDK Stream] " + jdkNames);

        // Eclipse Collections：链式调用，1 行
        MutableList<String> ecNames = people
                .select(p -> p.age() >= 18)
                .collect(Person::name);
        System.out.println("[EC select/collect] " + ecNames);

        // ---------- 理由3：Bag 多重集（JDK 没有的集合类型） ----------
        Bag<String> wordBag = Bags.mutable.with(
                "java", "python", "java", "go", "java");
        System.out.println("[Bag] java=" + wordBag.occurrencesOf("java")
                + ", python=" + wordBag.occurrencesOf("python")
                + ", rust=" + wordBag.occurrencesOf("rust")
                + ", distinct=" + wordBag.toSet()); // toSet 输出无序，此处元素恰好稳定

        // ---------- 理由4：真不可变集合（创建后无法修改，天然线程安全） ----------
        ImmutableList<String> immutable = Lists.immutable.with("a", "b", "c");
        System.out.println("[ImmutableList] " + immutable
                + "，尝试修改会抛 " + UnsupportedOperationException.class.getSimpleName());
        try {
            immutable.castToList().add("d"); // 唯一入口：castToList 返回不可变视图
        } catch (UnsupportedOperationException e) {
            System.out.println("[ImmutableList] add 被拒绝: UnsupportedOperationException");
        }

        // ---------- 理由5：富 API 一步到位（分组/计数/判空） ----------
        System.out.println("[groupBy 年龄段] " + people.groupBy(
                p -> p.age() >= 18 ? "成年" : "未成年"));
        System.out.println("[count] 成年人数 = " + people.count(p -> p.age() >= 18));
        System.out.println("[makeString] " + people.makeString(" | "));
    }
}
