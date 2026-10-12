package com.example.aop;

/**
 * AspectJ 入口：直接 new 目标对象（无 Spring、无代理工厂），
 * 编译期织入的增强逻辑已经在 .class 字节码里，调用即生效。
 */
public class AspectJDemo {

    public static void main(String[] args) {
        UserService service = new UserService();

        service.addUser("张三");        // 触发 @Before + 方法体
        System.out.println();

        String user = service.getUser(100); // 触发 @AfterReturning
        System.out.println("调用方拿到: " + user);
        System.out.println();

        service.updateUser("李四");      // 触发 @Around + @After
        System.out.println();

        try {
            service.deleteUser(999);    // 触发 @AfterThrowing
        } catch (IllegalArgumentException e) {
            System.out.println("调用方捕获业务异常: " + e.getMessage());
        }
        System.out.println();

        String order = service.queryOrder("ORDER_001"); // 触发 @Monitor 性能监控
        System.out.println("订单查询结果: " + order);
    }
}
