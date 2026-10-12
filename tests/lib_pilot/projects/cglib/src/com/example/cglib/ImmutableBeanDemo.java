package com.example.cglib;

import org.springframework.cglib.beans.ImmutableBean;

/**
 * 特色4：ImmutableBean——把可变 Bean 包装成只读视图，任何 setter 调用抛出 IllegalStateException。
 */
public class ImmutableBeanDemo {

    public static void main(String[] args) {
        User user = new User("张三", 25);

        User immutableUser = (User) ImmutableBean.create(user);
        System.out.println("只读视图读取: " + immutableUser.getName() + "," + immutableUser.getAge());

        try {
            immutableUser.setName("李四"); // 修改被拒绝
        } catch (IllegalStateException e) {
            System.out.println("捕获异常: " + e.getMessage());
        }

        // 原对象仍可修改，只读视图随之可见（视图是包装而非拷贝）
        user.setAge(26);
        System.out.println("原对象修改后视图读到 age=" + immutableUser.getAge());
    }
}
