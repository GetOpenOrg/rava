package com.example.rxjava;

import java.util.concurrent.TimeUnit;

import io.reactivex.rxjava3.core.Observable;
import io.reactivex.rxjava3.subjects.PublishSubject;

/**
 * 操作符大全：变换 map/flatMap/buffer，过滤 filter/take/distinct/debounce，
 * 组合 zip/merge/concat，创建 interval。
 */
public class RxOperatorsDemo {

    public static void main(String[] args) throws Exception {
        // ---------- 1. map：1:1 变换 ----------
        Observable.just("Hello", "RxJava", "Operators")
                .map(String::length)
                .subscribe(len -> System.out.println("[map] 长度: " + len));

        // ---------- 2. filter + distinct + take ----------
        Observable.fromArray(3, 1, 2, 3, 5, 1, 4)
                .distinct()              // 去重
                .filter(i -> i >= 2)     // 只保留 >=2
                .take(3)                 // 只取前 3 个
                .subscribe(i -> System.out.println("[filter链] " + i));

        // ---------- 3. flatMap：1 变多（订单 → 明细） ----------
        Observable.just("订单A", "订单B")
                .flatMap(order -> Observable.fromArray(order + "-商品1", order + "-商品2"))
                .subscribe(s -> System.out.println("[flatMap] " + s));

        // ---------- 4. buffer：批量收集 ----------
        Observable.range(1, 10)
                .buffer(4)
                .subscribe(list -> System.out.println("[buffer] " + list));

        // ---------- 5. zip：等待所有源都发射后合并 ----------
        Observable.zip(
                Observable.just("张三", "李四"),
                Observable.just(95, 88),
                (name, score) -> name + " → " + score + "分")
                .subscribe(pair -> System.out.println("[zip] " + pair));

        // ---------- 6. merge：谁快先发谁 ----------
        Observable.merge(
                Observable.just("A1", "A2").delay(20, TimeUnit.MILLISECONDS),
                Observable.just("B1", "B2"))
                .blockingSubscribe(s -> System.out.println("[merge] " + s));

        // ---------- 7. concat：严格按顺序拼接 ----------
        Observable.concat(Observable.just("第一段"), Observable.just("第二段"))
                .blockingSubscribe(s -> System.out.println("[concat] " + s));

        // ---------- 8. interval：定时发射（take 限制数量防止无限流） ----------
        Observable.interval(100, TimeUnit.MILLISECONDS)
                .take(3)
                .blockingSubscribe(tick -> System.out.println("[interval] tick " + tick));

        // ---------- 9. debounce：搜索框防抖 ----------
        PublishSubject<String> input = PublishSubject.create();
        input.debounce(200, TimeUnit.MILLISECONDS) // 停止输入 200ms 才真正触发
                .subscribe(kw -> System.out.println("[debounce] 触发搜索: " + kw));

        input.onNext("j");
        Thread.sleep(50);
        input.onNext("ja");
        Thread.sleep(50);
        input.onNext("java"); // 连续快速输入只有最后一次会触发搜索
        Thread.sleep(400);
    }
}
