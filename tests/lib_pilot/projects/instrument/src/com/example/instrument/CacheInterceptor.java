package com.example.instrument;

import java.lang.reflect.Method;
import java.util.Arrays;
import java.util.Map;
import java.util.concurrent.ConcurrentHashMap;

import org.springframework.cglib.proxy.MethodInterceptor;
import org.springframework.cglib.proxy.MethodProxy;

/**
 * 方法级缓存拦截器（文章第六节）：
 * 以“方法名 + 参数哈希”为 key 缓存返回值，命中直接返回，未命中执行后写入缓存。
 * 文章结尾的“小作业：用 CGLib 实现缓存代理”即此实现。
 */
public class CacheInterceptor implements MethodInterceptor {

    private final Map<String, Object> cache = new ConcurrentHashMap<>();

    @Override
    public Object intercept(Object obj, Method method, Object[] args,
                            MethodProxy proxy) throws Throwable {
        String key = method.getName() + ":" + Arrays.hashCode(args);

        if (cache.containsKey(key)) {
            System.out.println("【缓存命中】" + method.getName());
            return cache.get(key);
        }

        Object result = proxy.invokeSuper(obj, args);
        cache.put(key, result);
        System.out.println("【缓存未命中】" + method.getName() + "，已存入缓存");
        return result;
    }
}
