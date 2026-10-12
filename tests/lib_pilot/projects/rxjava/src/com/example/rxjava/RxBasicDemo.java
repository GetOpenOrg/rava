package com.example.rxjava;

import io.reactivex.rxjava3.core.Observable;
import io.reactivex.rxjava3.core.Observer;
import io.reactivex.rxjava3.disposables.Disposable;

/**
 * 核心三部曲：Observable（数据源）→ 操作符（加工）→ Observer/subscribe（消费）。
 */
public class RxBasicDemo {

    public static void main(String[] args) {
        // ---------- 1. 完整写法：Observable.create + Observer ----------
        Observable<String> observable = Observable.create(emitter -> {
            emitter.onNext("Hello");
            emitter.onNext("RxJava");
            emitter.onComplete();
        });

        Observer<String> observer = new Observer<>() {
            @Override
            public void onSubscribe(Disposable d) {
                System.out.println("[onSubscribe] 建立订阅");
            }

            @Override
            public void onNext(String s) {
                System.out.println("[onNext] 收到: " + s);
            }

            @Override
            public void onError(Throwable e) {
                System.out.println("[onError] " + e.getMessage());
            }

            @Override
            public void onComplete() {
                System.out.println("[onComplete] 数据流结束");
            }
        };
        observable.subscribe(observer);

        System.out.println();

        // ---------- 2. 实际开发常用的简洁写法 ----------
        Observable.just("Hello", "RxJava")
                .subscribe(s -> System.out.println("收到: " + s));
    }
}
