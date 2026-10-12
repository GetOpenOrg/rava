package com.example.cglib;

import org.springframework.cglib.beans.BeanCopier;

/**
 * 特色5：BeanCopier——基于字节码生成的属性拷贝，比反射式工具（如 Apache BeanUtils）快一个数量级。
 * copier 创建成本高，应静态缓存、反复复用。
 */
public class BeanCopierDemo {

    /** 创建一次，全局复用（这是 BeanCopier 性能优势的前提）。 */
    private static final BeanCopier COPIER = BeanCopier.create(User.class, UserVO.class, false);

    public static void main(String[] args) {
        User user = new User("王五", 30);
        UserVO vo = new UserVO();

        // 第一百万次与第一次调用同样快，无需反射
        COPIER.copy(user, vo, null);
        System.out.println("拷贝结果: " + vo.getName() + "," + vo.getAge());

        // 批量拷贝（复用同一个 copier，这是 BeanCopier 性能优势的前提；
        // 不打印随运行变化的耗时数值）
        int times = 1_000_000;
        for (int i = 0; i < times; i++) {
            UserVO target = new UserVO();
            COPIER.copy(user, target, null);
        }
        System.out.println("批量拷贝 " + times + " 次完成（复用同一个 copier 实例）");
    }
}
