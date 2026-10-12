package com.example.cglib;

import java.util.Arrays;

import org.springframework.cglib.beans.BulkBean;

/**
 * 特色6：BulkBean——为指定的一组属性生成批量 getter/setter 字节码，
 * 绕过反射，一次调用读写多个属性。
 */
public class BulkBeanDemo {

    /** getters 与 setters 的方法名、参数类型按索引一一对应。 */
    private static final BulkBean BULK_BEAN = BulkBean.create(
            Product.class,
            new String[]{"getName", "getPrice"},        // 批量读的方法
            new String[]{"setName", "setPrice"},        // 批量写的方法
            new Class[]{String.class, double.class});   // 对应属性类型

    public static void main(String[] args) {
        Product product = new Product();

        // 批量设置属性
        BULK_BEAN.setPropertyValues(product, new Object[]{"iPhone 15", 6999.0});

        // 批量读取属性
        Object[] values = BULK_BEAN.getPropertyValues(product);
        System.out.println(Arrays.toString(values));
        System.out.println("对象状态: " + product);
    }
}
