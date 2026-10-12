package com.example.collections;

import org.eclipse.collections.api.list.MutableList;
import org.eclipse.collections.api.list.primitive.IntList;
import org.eclipse.collections.api.list.primitive.LongList;
import org.eclipse.collections.api.list.primitive.DoubleList;
import org.eclipse.collections.api.list.primitive.BooleanList;
import org.eclipse.collections.impl.factory.Lists;
import org.eclipse.collections.impl.factory.primitive.BooleanLists;
import org.eclipse.collections.impl.factory.primitive.DoubleLists;
import org.eclipse.collections.impl.factory.primitive.IntLists;
import org.eclipse.collections.impl.factory.primitive.LongLists;
import org.eclipse.collections.impl.list.Interval;

/**
 * Eclipse Collections 深度解析（一）：富 API 与原生类型集合全家
 * （对应文章"统计判断族 / IntInterval / 八种原生集合"代码段，
 * 与既有 EclipseCollectionsDemo 的 select/collect/IntList 基础不重复）。
 */
public class EcDeepApiDemo {

    public static void main(String[] args) {
        MutableList<Integer> nums = Lists.mutable.with(3, 1, 4, 1, 5, 9, 2, 6);

        // ---------- 1. anySatisfy / allSatisfy / noneSatisfy ----------
        System.out.println("[anySatisfy] 有偶数=" + nums.anySatisfy(n -> n % 2 == 0)
                + ", 全小于10=" + nums.allSatisfy(n -> n < 10)
                + ", 无负数=" + nums.noneSatisfy(n -> n < 0));

        // ---------- 2. partition：一分为二 ----------
        var parts = nums.partition(n -> n % 2 == 0);
        System.out.println("[partition] 偶数=" + parts.getSelected() + " 奇数=" + parts.getRejected());

        // ---------- 3. injectInto：折叠归约（等价 Stream.reduce） ----------
        // EC 有 int/long/float/double 多个原生重载，lambda 需显式指明用哪个函数接口
        int sum = nums.injectInto(0,
                (org.eclipse.collections.api.block.function.primitive.IntObjectToIntFunction<Integer>)
                        (acc, each) -> acc + each);
        int max = nums.injectInto(Integer.MIN_VALUE,
                (org.eclipse.collections.api.block.function.primitive.IntObjectToIntFunction<Integer>)
                        (acc, each) -> Math.max(acc, each));
        System.out.println("[injectInto] sum=" + sum + " max=" + max);

        // ---------- 4. IntInterval：等差数列，一步生成偶数序列 ----------
        IntList evens = org.eclipse.collections.impl.list.primitive.IntInterval.evensFromTo(1, 10);
        System.out.println("[IntInterval] 1..10 的偶数 = " + evens);

        // ---------- 5. 原生类型集合全家（零装箱） ----------
        LongList longs = LongLists.mutable.with(10_000_000_000L, 20_000_000_000L);
        DoubleList doubles = DoubleLists.mutable.with(1.5, 2.5, 3.5);
        BooleanList flags = BooleanLists.mutable.with(true, false, true);
        IntList ints = IntLists.mutable.with(1, 2, 3, 4, 5);
        System.out.println("[LongList] sum=" + longs.sum());
        System.out.println("[DoubleList] average=" + doubles.average());
        System.out.println("[BooleanList] trueCount=" + flags.count(v -> v));
        System.out.println("[IntList] sum=" + ints.sum() + " max=" + ints.max()
                + " median=" + ints.median());
    }
}
