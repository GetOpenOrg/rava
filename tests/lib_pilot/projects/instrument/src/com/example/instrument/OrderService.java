package com.example.instrument;

/** 被监控的业务类：createOrder 标注了 @Monitor。 */
public class OrderService {

    @Monitor("下单链路")
    public void createOrder(String orderId) throws InterruptedException {
        Thread.sleep(150); // 模拟业务处理
        System.out.println("订单创建成功：" + orderId);
    }

    /** 未标注 @Monitor，不会被监控。 */
    public String queryOrder(String orderId) {
        return "ORDER_INFO_" + orderId;
    }
}
