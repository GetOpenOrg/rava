
import java.util.concurrent.CountDownLatch;
import java.util.concurrent.CyclicBarrier;
import java.util.concurrent.atomic.LongAdder;

/**
 * 知识点4：屏障的分批聚合能力。
 * 100 个线程、每批 10 个并行计算，用可复用屏障分 10 批聚合结果，
 * LongAdder 无锁累加，最后由 CountDownLatch 等待全部完成。
 *
 * 输出顺序固定说明：屏障动作在每批第 10 个到达者线程上顺序执行，
 * “批次聚合完成”恰好打印 10 次；不打印随运行变化的耗时数值。
 */
public class BarrierFeature {

    private static final int TOTAL = 100;
    private static final int BATCH_SIZE = 10;

    public static void main(String[] args) throws Exception {
        CountDownLatch done = new CountDownLatch(TOTAL);
        // 屏障动作：每凑满一批触发一次
        CyclicBarrier barrier = new CyclicBarrier(BATCH_SIZE, () ->
                System.out.println("批次聚合完成"));

        LongAdder total = new LongAdder();

        for (int i = 0; i < TOTAL; i++) {
            new Thread(() -> {
                int result = 42; // 模拟一次计算
                total.add(result);
                try {
                    barrier.await();
                } catch (Exception ignored) {
                } finally {
                    done.countDown();
                }
            }).start();
        }

        done.await();
        System.out.println("全部 " + TOTAL / BATCH_SIZE + " 批聚合完成");
        System.out.println("无锁聚合结果 = " + total.sum() + "（期望 " + TOTAL * 42 + "）");
    }
}
