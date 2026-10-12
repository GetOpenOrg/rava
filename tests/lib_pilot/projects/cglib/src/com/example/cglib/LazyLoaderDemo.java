package com.example.cglib;

import org.springframework.cglib.proxy.Enhancer;
import org.springframework.cglib.proxy.LazyLoader;

/**
 * 特色2：LazyLoader 延迟加载。
 * enhancer.create() 只创建“空壳”代理，首次真正使用对象时才回调 loadObject() 创建重量级对象。
 */
public class LazyLoaderDemo {

    static class HeavyObject {
        HeavyObject() {
            System.out.println("创建重量级对象，模拟耗时初始化...");
            try {
                Thread.sleep(100);
            } catch (InterruptedException e) {
                Thread.currentThread().interrupt();
            }
        }

        @Override
        public String toString() {
            return "HeavyObject 实例";
        }
    }

    public static void main(String[] args) {
        Enhancer enhancer = new Enhancer();
        enhancer.setSuperclass(HeavyObject.class);
        enhancer.setCallback((LazyLoader) () -> {
            System.out.println("LazyLoader.loadObject() 被触发");
            return new HeavyObject(); // 首次调用时才创建，且只创建一次
        });

        HeavyObject proxy = (HeavyObject) enhancer.create();
        System.out.println("代理创建完成，尚未真正创建重量级对象");

        System.out.println("第一次使用: " + proxy); // 此刻才触发真实创建
        System.out.println("第二次使用: " + proxy); // 复用同一实例
    }
}
