package com.example.kotlinlib;

import java.util.concurrent.CompletableFuture;

import kotlin.coroutines.EmptyCoroutineContext;
import kotlinx.coroutines.CoroutineScope;
import kotlinx.coroutines.CoroutineStart;
import kotlinx.coroutines.Dispatchers;
import kotlinx.coroutines.future.FutureKt;

/**
 * Java 调用 Kotlin 协程：kotlinx.coroutines 的 future 桥接（文章原话：
 * "Java 侧调用要绕一下，但它提供了 kotlinx.coroutines.future 模块，
 * 可以把协程桥接成 CompletableFuture"，1.7+ 该模块已并入 core）。
 *
 * 痛点实证：block 参数是 suspend lambda，Java 侧只能以 Function2
 * (CoroutineScope, Continuation) -> Object 的形式提供，签名冗长。
 *
 * 输出确定性说明：无休眠竞态，future 完成后同步取值。
 */
public class CoroutinesFutureDemo {

    public static void main(String[] args) throws Exception {
        // CoroutineScope 是单方法接口，Java lambda 直接实现
        CoroutineScope scope = () -> Dispatchers.getDefault();

        // ---------- 1. 单个协程 → CompletableFuture ----------
        // Kotlin 写法: future(Dispatchers.getDefault()) { "hello from coroutine" }
        // Java 必须传全 4 参：scope + context + start + suspend lambda(Function2)
        CompletableFuture<String> f1 = FutureKt.future(
                scope, EmptyCoroutineContext.INSTANCE, CoroutineStart.DEFAULT,
                (s, continuation) -> "hello from coroutine");
        System.out.println("[future] " + f1.get());

        // ---------- 2. 两个协程并行计算，Java 侧用 CompletableFuture API 组合 ----------
        CompletableFuture<Integer> a = FutureKt.future(
                scope, EmptyCoroutineContext.INSTANCE, CoroutineStart.DEFAULT,
                (s, continuation) -> 20);
        CompletableFuture<Integer> b = FutureKt.future(
                scope, EmptyCoroutineContext.INSTANCE, CoroutineStart.DEFAULT,
                (s, continuation) -> 22);
        System.out.println("[组合] 20 + 22 = " + a.thenCombine(b, Integer::sum).get());

        System.out.println("[结论] 协程本体在 Kotlin 世界跑，Java 侧只面对 CompletableFuture");
    }
}
