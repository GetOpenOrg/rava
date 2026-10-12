
import java.util.concurrent.CyclicBarrier;

/**
 * 知识点1：CyclicBarrier 基础用法。
 * 3 个线程全部到达屏障点后，先执行屏障动作，再一起继续。
 *
 * 输出顺序固定说明：线程按 id 错峰启动（id*50ms）保证“到达”顺序；
 * 屏障释放后再次错峰打印，保证“继续”顺序——调度器唤醒顺序不影响控制台输出。
 */
public class BarrierBasic {

    public static void main(String[] args) {
        // 创建屏障：3 个线程都到达后，执行 Runnable 动作
        CyclicBarrier barrier = new CyclicBarrier(3, () ->
                System.out.println("【屏障触发】所有线程都到齐了，开始下一阶段！"));

        for (int i = 1; i <= 3; i++) {
            final int id = i;
            new Thread(() -> {
                try {
                    Thread.sleep(id * 50L); // 错峰启动：固定到达顺序 1→2→3
                    System.out.println("线程" + id + " 到达屏障，等待其他人...");
                    barrier.await(); // 等待其他线程
                    Thread.sleep(id * 50L); // 释放后再错峰：固定继续顺序 1→2→3
                    System.out.println("线程" + id + " 继续执行后续任务");
                } catch (Exception e) {
                    e.printStackTrace();
                }
            }).start();
        }
    }
}
