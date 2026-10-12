package com.example.rxjava;

import java.util.Map;
import java.util.concurrent.ConcurrentHashMap;

import io.reactivex.rxjava3.core.Observable;
import io.reactivex.rxjava3.disposables.CompositeDisposable;

/**
 * 实战进阶（文章第七节）：缓存 → 网络 → 数据库的多级数据流。
 * switchIfEmpty：缓存为空才走网络；onErrorResumeNext：网络失败走数据库兜底；
 * doOnNext：结果副作用写缓存。15 行核心代码替代 40-50 行回调嵌套。
 */
public class RxCacheDemo {

    static final class User {
        private final String name;

        User(String name) {
            this.name = name;
        }

        @Override
        public String toString() {
            return "User{" + name + "}";
        }
    }

    /** 模拟本地缓存（进程内）。 */
    static final Map<String, User> CACHE = new ConcurrentHashMap<>();

    /** 缓存源：命中则发射，未命中则直接完成（空流触发 switchIfEmpty）。 */
    static Observable<User> fromCache(String userId) {
        return Observable.create(emitter -> {
            User user = CACHE.get(userId);
            if (user != null) {
                System.out.println("[缓存] 命中: " + user);
                emitter.onNext(user);
            }
            emitter.onComplete();
        });
    }

    /** 网络源：u_404 模拟请求失败。 */
    static Observable<User> fromNetwork(String userId) {
        return Observable.create(emitter -> {
            System.out.println("[网络] 请求中: " + userId);
            if ("u_404".equals(userId)) {
                emitter.onError(new RuntimeException("网络超时"));
            } else {
                emitter.onNext(new User("网络用户-" + userId));
                emitter.onComplete();
            }
        });
    }

    /** 数据库兜底源。 */
    static Observable<User> fromDb(String userId) {
        return Observable.create(emitter -> {
            System.out.println("[数据库] 兜底查询: " + userId);
            emitter.onNext(new User("数据库用户-" + userId));
            emitter.onComplete();
        });
    }

    /** 统一的多级读取链路。 */
    static Observable<User> load(String userId) {
        return fromCache(userId)
                .switchIfEmpty(fromNetwork(userId)   // 缓存为空 → 走网络
                        .doOnNext(u -> CACHE.put(userId, u))) // 网络结果写缓存
                .onErrorResumeNext(throwable -> fromDb(userId)); // 网络失败 → 数据库兜底
    }

    public static void main(String[] args) {
        CompositeDisposable disposables = new CompositeDisposable();

        // 场景1：缓存未命中 → 网络成功 → 写缓存
        disposables.add(load("u_1001")
                .subscribe(u -> System.out.println("结果: " + u)));

        // 场景2：再次读取 → 缓存命中
        disposables.add(load("u_1001")
                .subscribe(u -> System.out.println("结果: " + u)));

        // 场景3：网络失败 → 数据库兜底
        disposables.add(load("u_404")
                .subscribe(u -> System.out.println("结果: " + u),
                        e -> System.out.println("最终失败: " + e.getMessage())));

        disposables.dispose();
    }
}
