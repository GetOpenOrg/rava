package com.example.instrument;

import org.springframework.cglib.proxy.Enhancer;

/**
 * 缓存代理入口：第二次调用同参数方法直接命中缓存，不再触发数据库查询。
 */
public class CacheProxyDemo {

    public static void main(String[] args) {
        Enhancer enhancer = new Enhancer();
        enhancer.setSuperclass(DataService.class);
        enhancer.setCallback(new CacheInterceptor());

        DataService service = (DataService) enhancer.create();

        System.out.println(service.getData(1)); // 未命中：查库并缓存
        System.out.println(service.getData(1)); // 命中：直接返回缓存
        System.out.println(service.getData(2)); // 参数不同：再次查库
    }
}
