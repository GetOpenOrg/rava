package com.example.disruptor;

import java.util.List;
import java.util.concurrent.CopyOnWriteArrayList;

import com.lmax.disruptor.BlockingWaitStrategy;
import com.lmax.disruptor.RingBuffer;
import com.lmax.disruptor.dsl.Disruptor;
import com.lmax.disruptor.dsl.ProducerType;
import com.lmax.disruptor.util.DaemonThreadFactory;

/**
 * Disruptor（二）：依赖图编排 handleEventsWith(A).then(B)
 * （对应文章"消费模式"：先落盘再复制/先计算再汇报的经典链）。
 * B 拿到的事件必然在 A 处理完之后——框架保证阶段依赖，无需业务方加锁等待。
 *
 * 输出确定性说明：依赖图保证 A 全部先于 B；输出为聚合结论。
 */
public class DisruptorChainDemo {

    static class TaskEvent {
        String task = "";

        void set(String task) {
            this.task = task;
        }
    }

    public static void main(String[] args) {
        int count = 8;
        List<String> stage1Done = new CopyOnWriteArrayList<>();
        List<String> stage2Done = new CopyOnWriteArrayList<>();

        Disruptor<TaskEvent> disruptor = new Disruptor<>(
                TaskEvent::new, 1024, DaemonThreadFactory.INSTANCE,
                ProducerType.SINGLE, new BlockingWaitStrategy());

        // 阶段A（模拟"落盘"）：标记已处理
        var stageA = (com.lmax.disruptor.EventHandler<TaskEvent>) (event, sequence, end) ->
                stage1Done.add("A:" + event.task);
        // 阶段B（模拟"汇报"）：依赖 A 完成后执行
        var stageB = (com.lmax.disruptor.EventHandler<TaskEvent>) (event, sequence, end) ->
                stage2Done.add("B:" + event.task);

        disruptor.handleEventsWith(stageA).then(stageB);
        disruptor.start();

        RingBuffer<TaskEvent> ringBuffer = disruptor.getRingBuffer();
        for (int i = 1; i <= count; i++) {
            String name = "task-" + i;
            ringBuffer.publishEvent((event, sequence) -> event.set(name));
        }
        disruptor.shutdown();

        System.out.println("[阶段A] 处理数 = " + stage1Done.size());
        System.out.println("[阶段B] 处理数 = " + stage2Done.size());
        System.out.println("[B 的首个事件] " + stage2Done.get(0));
        // then() 的语义是"逐事件屏障"：B 处理第 N 个事件前，A 必已处理完第 N 个
        System.out.println("[结论] handleEventsWith(A).then(B)：逐事件保证 A 先于 B，"
                + "两阶段计数一致 = " + (stage1Done.size() == stage2Done.size()));
    }
}
