package com.example.rxjava;

import java.util.concurrent.CountDownLatch;

import io.reactivex.rxjava3.core.Observable;
import io.reactivex.rxjava3.disposables.CompositeDisposable;
import io.reactivex.rxjava3.schedulers.Schedulers;

/**
 * 线程切换：subscribeOn 决定数据生产线程，observeOn 决定下游消费线程。
 * 并演示 CompositeDisposable 统一取消订阅（防内存泄漏）。
 */
public class RxThreadDemo {

    public static void main(String[] args) throws Exception {
        CountDownLatch done = new CountDownLatch(1);
        CompositeDisposable disposables = new CompositeDisposable();

        disposables.add(Observable.just("任务数据")
                .doOnNext(d -> System.out.println("[生产] 线程: "
                        + Thread.currentThread().getName()))
                .subscribeOn(Schedulers.io())    // 生产侧切换到 IO 线程池
                .observeOn(Schedulers.single())  // 消费侧切换到 single 线程
                .subscribe(
                        d -> System.out.println("[消费] " + d + "，线程: "
                                + Thread.currentThread().getName()),
                        Throwable::printStackTrace,
                        done::countDown));

        done.await();
        disposables.dispose(); // 统一取消所有订阅
        System.out.println("已统一取消订阅，避免内存泄漏");
    }
}
