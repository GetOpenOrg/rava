package com.example.cglib;

/**
 * CGLib 目标类：普通类、无接口。
 * pay() 是 final 方法——CGLib 靠生成子类实现代理，final 方法无法被覆盖增强。
 */
public class OrderService {

    public void createOrder(String orderId) {
        System.out.println("创建订单：" + orderId);
    }

    public final void pay() {
        System.out.println("支付订单（final方法，无法真正被拦截）");
    }
}
