package com.example.aop;

/**
 * AspectJ 目标类：普通 Java 类，无任何接口、无任何框架依赖。
 * 编译期由 ajc 织入 LogAspect / PerformanceAspect 的增强逻辑。
 */
public class UserService {

    public void addUser(String name) {
        System.out.println("添加用户: " + name);
    }

    public String getUser(int id) {
        System.out.println("查询用户: id=" + id);
        return "User" + id;
    }

    public void updateUser(String name) {
        System.out.println("更新用户: " + name);
    }

    /** 演示 @AfterThrowing 异常通知。 */
    public void deleteUser(int id) {
        throw new IllegalArgumentException("用户不存在: " + id);
    }

    /** 演示自定义 @Monitor 注解的性能监控。 */
    @Monitor("订单链路")
    public String queryOrder(String orderId) {
        try {
            Thread.sleep(120); // 模拟慢查询
        } catch (InterruptedException e) {
            Thread.currentThread().interrupt();
        }
        return "ORDER_INFO_" + orderId;
    }
}
