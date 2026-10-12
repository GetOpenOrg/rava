
import java.util.concurrent.CyclicBarrier;

/**
 * 知识点2：屏障 vs 锁。
 * 锁是“不许同时”（互斥），屏障是“必须同时”（集合等待）。
 * 场景：所有玩家加载完毕后同时进入游戏。
 *
 * 输出顺序固定说明：与 BarrierBasic 相同的双错峰模式。
 */
public class LockVsBarrier {

    public static void main(String[] args) {
        CyclicBarrier barrier = new CyclicBarrier(3);

        for (int i = 1; i <= 3; i++) {
            final int id = i;
            new Thread(() -> {
                try {
                    Thread.sleep(id * 50L); // 错峰启动：固定“加载完成”顺序
                    System.out.println("玩家" + id + " 已加载100%");
                    barrier.await(); // 等待所有玩家加载完
                    Thread.sleep(id * 50L); // 错峰：固定“开始游戏”顺序
                    System.out.println("玩家" + id + " 开始游戏");
                } catch (Exception e) {
                    e.printStackTrace();
                }
            }).start();
        }
    }
}
