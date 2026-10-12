package com.example.kotlinlib;

import java.util.List;

import kotlin.collections.CollectionsKt;
import kotlin.text.StringsKt;

/**
 * Java 调用 Kotlin 标准库（对应文章 Kotlin 节"典型形态"）：
 * Kotlin 顶层函数和扩展函数编译成静态方法，挂在"文件名Kt"的类里，
 * Java 侧 import 这些 XxxKt 类即可调用。
 *
 * 同时演示文章提到的两个痛点：
 * - 默认参数不会生成重载（除非库作者标 @JvmOverloads）→ joinToString 必须传全 7 个参数
 * - 扩展函数（List.forEach、String.repeat 等）→ 全部变成静态方法
 *
 * 输出确定性说明：固定集合、固定文案。
 */
public class KotlinStdLibDemo {

    public static void main(String[] args) {
        List<Integer> list = List.of(1, 2, 3);

        // ---------- 1. 文章原文：扩展函数 forEach → CollectionsKt 静态方法 ----------
        // 注意：Kotlin 的 Unit 在 Java 侧是"返回 null"，lambda 必须显式 return null
        System.out.print("[forEach] ");
        CollectionsKt.forEach(list, i -> {
            System.out.print(i + " ");
            return null; // Unit
        });
        System.out.println();

        // ---------- 2. 更多集合扩展函数（全是 CollectionsKt 的静态方法） ----------
        System.out.println("[maxOrNull] " + CollectionsKt.maxOrNull(list));
        System.out.println("[reversed]  " + CollectionsKt.reversed(list));
        System.out.println("[first]     " + CollectionsKt.first(list));

        // ---------- 3. 字符串扩展函数在 StringsKt 里 ----------
        System.out.println("[repeat]    " + StringsKt.repeat("ab", 3));
        System.out.println("[lines]     " + StringsKt.lines("a\nb\nc"));
        System.out.println("[isBlank]   " + StringsKt.isBlank("   "));

        // ---------- 4. 默认参数陷阱：joinToString 的 7 个参数一个都不能省 ----------
        // Kotlin: list.joinToString(separator=", ", prefix="[", postfix="]") 全部有默认值
        String joined = CollectionsKt.joinToString(list, ", ", "[", "]", -1, "...", null);
        System.out.println("[joinToString] " + joined);
        System.out.println("[joinToString] （对比：Kotlin 源码只需写 list.joinToString()）");
    }
}
