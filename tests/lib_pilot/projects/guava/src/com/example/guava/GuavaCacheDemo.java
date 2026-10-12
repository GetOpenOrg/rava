package com.example.guava;

import java.util.concurrent.ExecutionException;
import java.util.concurrent.atomic.AtomicInteger;

import com.google.common.cache.CacheBuilder;
import com.google.common.cache.CacheLoader;
import com.google.common.cache.LoadingCache;

/**
 * Guava 本地缓存 LoadingCache（对应文章 CacheBuilder 一节）。
 * 用加载计数器验证"第一次 get 触发 load，之后命中缓存"。
 *
 * 注：Guava 官方已建议新项目改用 Caffeine（API 同源），本示例演示文章主角。
 * 输出确定性说明：单线程顺序访问，不做任何依赖时间的过期断言。
 */
public class GuavaCacheDemo {

    public static void main(String[] args) throws ExecutionException {
        AtomicInteger loadCount = new AtomicInteger();

        LoadingCache<String, String> cache = CacheBuilder.newBuilder()
                .maximumSize(100)
                .build(CacheLoader.from(key -> {
                    loadCount.incrementAndGet(); // 每次真正加载计数
                    return "value-of-" + key;
                }));

        String first = cache.get("k1");       // 触发加载 #1
        String second = cache.get("k1");      // 命中缓存，不再加载
        String third = cache.get("k2");       // 触发加载 #2

        System.out.println("[get] k1 首次 = " + first);
        System.out.println("[get] k1 再次 = " + second + "（与首次同值）");
        System.out.println("[get] k2 首次 = " + third);
        System.out.println("[统计] 实际加载次数 = " + loadCount.get() + "（期望 2：k1/k2 各一次）");
        System.out.println("[统计] 命中率 = " + String.format("%.2f", cache.stats().hitRate())
                + "（4 次访问 2 次命中）");
        System.out.println("[统计] 当前缓存条目 = " + cache.size());
    }
}
