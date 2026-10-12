package com.example.clojurelib;

import clojure.java.api.Clojure;
import clojure.lang.IFn;
import clojure.lang.PersistentVector;

/**
 * Java 调用 Clojure（对应文章 Clojure 节）：
 * 标准交互方式是 clojure.java.api.Clojure 求值——理论上 Clojure 库都是
 * 字节码都能调，但惯用法（持久数据结构、函数式风格）与 Java 心智模型差异大。
 *
 * 输出确定性说明：全部纯函数计算。
 */
public class ClojureApiDemo {

    public static void main(String[] args) {
        // ---------- 1. 基本求值：clojure.core/+ ----------
        IFn plus = Clojure.var("clojure.core", "+");
        System.out.println("[+]        (1 2 3 4) => " + plus.invoke(1, 2, 3, 4));

        // ---------- 2. 按文章方式 require 后调用 clojure.string ----------
        IFn require = Clojure.var("clojure.core", "require");
        require.invoke(Clojure.read("clojure.string"));
        IFn upperCase = Clojure.var("clojure.string", "upper-case");
        System.out.println("[upper]    " + upperCase.invoke("hello clojure"));
        IFn join = Clojure.var("clojure.string", "join");
        System.out.println("[join]     " + join.invoke("-", "abcd"));

        // ---------- 3. 持久数据结构：conj 返回新集合，原集合不变 ----------
        PersistentVector v1 = (PersistentVector) Clojure.read("[1 2 3]");
        IFn conj = Clojure.var("clojure.core", "conj");
        PersistentVector v2 = (PersistentVector) conj.invoke(v1, 4);
        System.out.println("[持久向量] 原 " + v1 + " 不变，conj 后得到新向量 " + v2);

        // ---------- 4. map 高阶函数：inc 组合 ----------
        IFn map = Clojure.var("clojure.core", "map");
        IFn inc = Clojure.var("clojure.core", "inc");
        Object result = map.invoke(inc, v2);
        System.out.println("[map inc]  " + result);

        System.out.println("[结论] 能调；但心智模型不同（不可变、一切皆函数），" +
                "实践中通常以嵌入 Clojure 运行时的方式共存");
    }
}
