
import java.util.concurrent.CountDownLatch;
import java.util.concurrent.CyclicBarrier;
import java.util.concurrent.TimeUnit;
import java.util.concurrent.TimeoutException;

/**
 * 知识点3：屏障的三种核心用法。
 * 1) CountDownLatch：一次性等待 N 个事件完成
 * 2) CyclicBarrier：可循环复用，适合分阶段任务
 * 3) 带超时的 await：防止个别线程卡死导致全体死等
 *
 * 输出顺序固定说明：各线程按 id 错峰执行，多阶段任务在每次 await 后再次错峰，
 * 保证每个阶段的打印顺序恒为 线程1 → 线程2 → 线程3。
 */
public class BarrierUsage {

    public static void main(String[] args) throws Exception {
        // ---------- 用法1：CountDownLatch（一次性） ----------
        CountDownLatch latch = new CountDownLatch(2);
        new Thread(() -> {
            try {
                Thread.sleep(50); // 错峰：图片先完成
                System.out.println("下载图片完成");
            } catch (InterruptedException ignored) {
            } finally {
                latch.countDown();
            }
        }).start();
        new Thread(() -> {
            System.out.println("下载视频完成");
            latch.countDown();
        }).start();
        latch.await();
        System.out.println("【用法1】所有资源下载完毕，开始渲染\n");

        // ---------- 用法2：CyclicBarrier（循环使用，两个阶段） ----------
        CyclicBarrier cyclicBarrier = new CyclicBarrier(3);
        CountDownLatch stageDone = new CountDownLatch(3);
        for (int i = 1; i <= 3; i++) {
            final int id = i;
            new Thread(() -> {
                try {
                    Thread.sleep(id * 50L); // 阶段1前错峰
                    System.out.println("线程" + id + " 执行第1阶段任务");
                    cyclicBarrier.await(); // 第1阶段集合点

                    Thread.sleep(id * 50L); // 阶段2前错峰（屏障可复用）
                    System.out.println("线程" + id + " 执行第2阶段任务");
                    cyclicBarrier.await(); // 第2阶段集合点
                } catch (Exception e) {
                    e.printStackTrace();
                } finally {
                    stageDone.countDown();
                }
            }).start();
        }
        stageDone.await();
        System.out.println("【用法2】两个阶段全部完成\n");

        // ---------- 用法3：带超时的屏障 ----------
        CyclicBarrier timeoutBarrier = new CyclicBarrier(2);
        new Thread(() -> {
            try {
                Thread.sleep(3000);       // 模拟一个“迟到”的线程
                timeoutBarrier.await();
            } catch (Exception ignored) {
            }
        }).start();

        try {
            timeoutBarrier.await(1, TimeUnit.SECONDS);
        } catch (TimeoutException e) {
            System.out.println("【用法3】超时退出，不等了");
        }
    }
}
