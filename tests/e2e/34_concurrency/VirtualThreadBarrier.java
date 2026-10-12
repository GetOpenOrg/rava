
import java.util.concurrent.CountDownLatch;
import java.util.concurrent.CyclicBarrier;
import java.util.concurrent.ExecutorService;
import java.util.concurrent.Executors;
import java.util.concurrent.atomic.AtomicInteger;

/**
 * 知识点8-技巧2：虚拟线程 + 屏障（Java 21 Project Loom）。
 * 虚拟线程极轻量，可大量创建；配合可复用屏障做分阶段同步。
 *
 * 输出顺序固定说明：屏障动作按批次顺序触发，用计数器标注批次号；
 * 不打印虚拟线程对象（其名称/编号由调度器分配，每次运行不同）。
 */
public class VirtualThreadBarrier {

    private static final int TASKS = 30;
    private static final int BATCH_SIZE = 10;

    public static void main(String[] args) throws Exception {
        CountDownLatch done = new CountDownLatch(TASKS);
        AtomicInteger batch = new AtomicInteger();
        CyclicBarrier barrier = new CyclicBarrier(BATCH_SIZE, () ->
                System.out.println("第 " + batch.incrementAndGet() + " 批虚拟线程全部到达屏障"));

        try (ExecutorService executor = Executors.newVirtualThreadPerTaskExecutor()) {
            for (int i = 1; i <= TASKS; i++) {
                executor.submit(() -> {
                    try {
                        barrier.await();
                        done.countDown();
                        return null;
                    } catch (Exception e) {
                        return null;
                    }
                });
            }
        } // try-with-resources 隐式 close() 等待全部任务结束

        done.await();
        System.out.println(TASKS + " 个虚拟线程分 " + TASKS / BATCH_SIZE + " 批同步完成");
    }
}
