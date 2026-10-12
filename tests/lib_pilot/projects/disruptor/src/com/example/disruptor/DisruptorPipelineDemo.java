package com.example.disruptor;

import java.util.ArrayList;
import java.util.List;
import java.util.concurrent.atomic.AtomicLong;

import com.lmax.disruptor.BlockingWaitStrategy;
import com.lmax.disruptor.EventHandler;
import com.lmax.disruptor.RingBuffer;
import com.lmax.disruptor.dsl.Disruptor;
import com.lmax.disruptor.dsl.ProducerType;
import com.lmax.disruptor.util.DaemonThreadFactory;

/**
 * Disruptor（一）：单生产者流水线（对应文章"RingBuffer/Sequence/生产消费"概念）。
 * 单生产者发布 1..N 严格有序，消费者累加并按序收集——环形缓冲区不丢不乱的核心卖点。
 *
 * 输出确定性说明：单生产者下事件顺序确定；只打印聚合值与前后片段，不打印耗时。
 */
public class DisruptorPipelineDemo {

    static class LongEvent {
        private long value;

        long get() {
            return value;
        }

        void set(long value) {
            this.value = value;
        }
    }

    public static void main(String[] args) {
        final int count = 1000;
        AtomicLong sum = new AtomicLong();
        List<Long> received = new ArrayList<>(count);

        // 4.0 起 Executor 构造器移除，使用 ThreadFactory；Blocking 策略不烧 CPU
        Disruptor<LongEvent> disruptor = new Disruptor<>(
                LongEvent::new, 1024, DaemonThreadFactory.INSTANCE,
                ProducerType.SINGLE, new BlockingWaitStrategy());

        disruptor.handleEventsWith((EventHandler<LongEvent>) (event, sequence, endOfBatch) -> {
            sum.addAndGet(event.get());
            received.add(event.get()); // 单消费者按序追加
        });

        disruptor.start();
        RingBuffer<LongEvent> ringBuffer = disruptor.getRingBuffer();
        for (long i = 1; i <= count; i++) {
            long value = i;
            ringBuffer.publishEvent((event, sequence) -> event.set(value));
        }
        disruptor.shutdown(); // 等消费者处理完全部事件

        System.out.println("[发布] " + count + " 个事件（1.." + count + "）");
        System.out.println("[消费] sum = " + sum.get() + "（期望 " + (long) count * (count + 1) / 2 + "）");
        System.out.println("[消费] 前5个 = " + received.subList(0, 5));
        System.out.println("[消费] 后5个 = " + received.subList(count - 5, count));
        boolean inOrder = true;
        for (int i = 0; i < count; i++) {
            if (received.get(i) != i + 1) {
                inOrder = false;
                break;
            }
        }
        System.out.println("[顺序] 严格有序 = " + inOrder);
    }
}
