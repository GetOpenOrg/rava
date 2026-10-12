
import java.util.concurrent.Phaser;

/**
 * 知识点7：Phaser——线程数动态变化的多阶段同步。
 * 与 CyclicBarrier 的区别：Phaser 支持动态注册/注销参与方，适合参与者数量不固定的场景。
 *
 * 输出顺序固定说明：工人线程按 id 错峰打印每个阶段的工作日志；
 * 主线程不参与同步（避免注册数随注销时序变化），注册数恒为 3 → 3 → 0。
 */
public class PhaserDemo {

    public static void main(String[] args) {
        Phaser phaser = new Phaser() {
            @Override
            protected boolean onAdvance(int phase, int registeredParties) {
                // 每个阶段所有参与者到达后回调；返回 true 表示终止
                System.out.println("阶段" + phase + "完成，当前注册数: " + registeredParties);
                return registeredParties == 0; // 无人注册时终止
            }
        };

        int workerCount = 3;
        for (int i = 1; i <= workerCount; i++) {
            phaser.register(); // 动态注册参与者
            final int id = i;
            new Thread(() -> {
                try {
                    Thread.sleep(id * 50L); // 阶段0前错峰
                    System.out.println("工人" + id + " 完成阶段0的工作");
                    phaser.arriveAndAwaitAdvance();

                    Thread.sleep(id * 50L); // 阶段1前错峰
                    System.out.println("工人" + id + " 完成阶段1的工作");
                    phaser.arriveAndAwaitAdvance();
                } catch (InterruptedException e) {
                    Thread.currentThread().interrupt();
                } finally {
                    phaser.arriveAndDeregister(); // 干完活注销自己
                }
            }).start();
        }
    }
}
