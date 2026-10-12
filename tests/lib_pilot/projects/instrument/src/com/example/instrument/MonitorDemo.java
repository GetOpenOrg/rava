package com.example.instrument;

import org.springframework.cglib.proxy.Enhancer;

/**
 * 性能监控实战入口：用 CGLib 代理 OrderService，
 * @Monitor 方法自动输出耗时统计，无需在业务代码里手写计时代码。
 */
public class MonitorDemo {

    public static void main(String[] args) throws InterruptedException {
        Enhancer enhancer = new Enhancer();
        enhancer.setSuperclass(OrderService.class);
        enhancer.setCallback(new PerformanceInterceptor());

        OrderService service = (OrderService) enhancer.create();

        service.createOrder("ORDER_001");              // 触发慢查询日志（>100ms）
        System.out.println(service.queryOrder("ORDER_001")); // 不触发监控
    }
}
