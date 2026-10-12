package com.example.cglib;

import java.util.concurrent.atomic.AtomicInteger;

import org.springframework.cglib.proxy.Dispatcher;
import org.springframework.cglib.proxy.Enhancer;

/**
 * 特色3：Dispatcher 动态分发。
 * 与 LazyLoader 的区别：每次方法调用都会回调 loadObject() 取一个（可以是新的）实例。
 *
 * 输出顺序固定说明：实例编号用递增计数器而非随机/纳秒值，
 * 且不打印对象哈希（每次运行不同）。
 */
public class DispatcherDemo {

    /** 固定递增的实例编号，保证输出可预期。 */
    private static final AtomicInteger INSTANCE_SEQ = new AtomicInteger();

    static class TaskService {
        private final int id;

        /** CGLib 生成子类时必须能调用无参构造器（文章坑点2），必须显式保留。 */
        TaskService() {
            this.id = 0;
        }

        TaskService(int id) {
            this.id = id;
        }

        public void execute() {
            System.out.println("任务执行，实例id=" + id);
        }
    }

    public static void main(String[] args) {
        Enhancer enhancer = new Enhancer();
        enhancer.setSuperclass(TaskService.class);
        enhancer.setCallback((Dispatcher) () -> new TaskService(
                INSTANCE_SEQ.incrementAndGet())); // 每次调用都新建实例

        TaskService proxy = (TaskService) enhancer.create();
        proxy.execute();  // 实例id=1
        proxy.execute();  // 实例id=2
        proxy.execute();  // 实例id=3：三次调用使用三个不同实例
    }
}
