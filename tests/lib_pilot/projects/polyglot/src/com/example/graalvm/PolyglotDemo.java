package com.example.graalvm;

import org.graalvm.polyglot.Context;
import org.graalvm.polyglot.Value;

/**
 * 知识点5：Polyglot 多语言互通——Java 直接执行 JavaScript。
 *
 * 依赖 org.graalvm.polyglot:polyglot 提供 API；在 GraalVM JDK 上开箱即用，
 * 普通 JDK 上需在 classpath 追加 GraalJS 实现（org.graalvm.polyglot:js）才能运行。
 */
public class PolyglotDemo {

    public static void main(String[] args) {
        try (Context ctx = Context.create()) {
            // JS 计算平方数列，Java 侧直接拿到结构化结果
            Value result = ctx.eval("js",
                    "Array.from({length:10}, (_, i) => i * i)");
            System.out.println("JS 执行结果: " + result);

            // 也可以传入并调用 JS 函数
            Value square = ctx.eval("js", "x => x * x");
            System.out.println("调用 JS 函数 square(12) = " + square.execute(12));
        }
    }
}
