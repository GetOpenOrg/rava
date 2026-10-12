package com.example.scalalib;

import scala.Function1;
import scala.collection.immutable.List;
import scala.jdk.javaapi.CollectionConverters$;

/**
 * Java 调用 Scala 库的"注意点"实证（对应文章 Scala 节）：
 * 1. Scala 函数类型是 scala.Function1 这类类——Java 里可以当 SAM 用，
 *    但出现在签名里时远不如 java.util.function.Function 顺眼
 * 2. 带 implicit 参数的方法在 Java 侧要手动传全参数（sum 需要 Numeric）
 * 3. 集合互转：对象方法多为隐式增强，Java 应走 scala.jdk.javaapi 门面
 * 4. Scala 单例对象在 Java 侧是 Xxx$.MODULE$
 */
public class ScalaInteropDemo {

    public static void main(String[] args) {
        // ---------- 1. Java List → Scala 不可变 List（走 javaapi 门面 + $.MODULE$） ----------
        List<Integer> scalaList = CollectionConverters$.MODULE$
                .asScala(java.util.List.of(1, 2, 3)).toList();

        // ---------- 2. Function1：Scala 函数类型，Java lambda 可以 SAM 化 ----------
        Function1<Integer, Integer> doubleIt = x -> x * 2;
        // 显式类型见证 <Integer>：Java 对 Scala 泛型方法的推断不可靠（痛点之一）
        List<Integer> doubled = scalaList.<Integer>map(doubleIt);
        System.out.println("[map]      " + scalaList + " -> " + doubled);
        System.out.println("[mkString] " + doubled.mkString(", "));

        // ---------- 3. implicit 参数要手动传全：sum(Numeric) ----------
        // Kotlin/Java 里 list.sum() 无参；Scala 的 sum 带隐式 Numeric 参数，
        // Java 侧必须显式传入 IntIsIntegral 单例。
        // 该单例的静态类型不带泛型（Numeric<Object>），需显式转换回 Numeric<Integer>
        @SuppressWarnings("unchecked")
        scala.math.Numeric<Integer> intNumeric = (scala.math.Numeric<Integer>)
                (scala.math.Numeric<?>) scala.math.Numeric.IntIsIntegral$.MODULE$;
        int sum = scalaList.sum(intNumeric);
        System.out.println("[sum]      " + sum + "（Java 侧必须手动传 implicit 的 Numeric 参数）");

        // ---------- 4. 回转 Java 集合（不可变 List 是 Seq 的子类型，命中 asJava(Seq) 重载） ----------
        java.util.List<Integer> backToJava = CollectionConverters$.MODULE$.asJava(doubled);
        System.out.println("[回转Java] " + backToJava.getClass().getSimpleName() + " " + backToJava);

        System.out.println("[结论] 能调，但签名啰嗦；生产代码优先找官方 Java API 门面（如 Spark 的 JavaRDD）");
    }
}
