import java.util.concurrent.atomic.AtomicInteger;
import java.util.concurrent.atomic.AtomicLong;

// 规模：10⁵ 个虚拟线程各 sleep 2 s 后汇总（计划 §21.8.4 a3-T6）。sleep 期间虚拟线程卸载、只占停泊的协程栈与堆对象；
// 全部线程在第一个线程醒来之前都已开始执行（起跑时已有线程完成即计 lateStarts），即 10⁵ 个同时处于停泊。
public class TestVirtualThreadScale {
    public static void main(String[] args) throws Exception {
        final int n = 100_000;
        AtomicInteger finished = new AtomicInteger();
        AtomicInteger lateStarts = new AtomicInteger();
        AtomicLong sum = new AtomicLong();
        Thread[] ts = new Thread[n];
        for (int i = 0; i < n; i++) {
            final int id = i;
            ts[i] = Thread.ofVirtual().start(() -> {
                if (finished.get() > 0) lateStarts.incrementAndGet();
                try {
                    Thread.sleep(2000);
                } catch (InterruptedException e) {
                    throw new RuntimeException(e);
                }
                sum.addAndGet(id);
                finished.incrementAndGet();
            });
        }
        for (Thread t : ts) t.join();
        int alive = 0;
        for (Thread t : ts) if (t.isAlive()) alive++;
        System.out.println("finished: " + finished.get());
        System.out.println("sum: " + sum.get());
        System.out.println("all sleeping at once: " + (lateStarts.get() == 0));
        System.out.println("alive after join: " + alive);
    }
}
