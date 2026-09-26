import java.util.concurrent.locks.LockSupport;

/** 真实时间语义：sleep / parkNanos / parkUntil 挂钟下界；unpark 先于 park 的许可；nanoTime 单调。 */
public class TestSleepParkClock {
    public static void main(String[] args) throws Exception {
        long t0 = System.nanoTime();
        long w0 = System.currentTimeMillis();
        Thread.sleep(200);
        long dn = (System.nanoTime() - t0) / 1_000_000;
        long dw = System.currentTimeMillis() - w0;
        System.out.println("sleep nano>=200 " + (dn >= 200) + " wall>=199 " + (dw >= 199) + " sane " + (dn < 5000));

        t0 = System.nanoTime();
        LockSupport.parkNanos(100_000_000L);
        long dp = (System.nanoTime() - t0) / 1_000_000;
        System.out.println("parkNanos>=100 " + (dp >= 100));

        t0 = System.nanoTime();
        LockSupport.parkUntil(System.currentTimeMillis() + 100);
        long du = (System.nanoTime() - t0) / 1_000_000;
        System.out.println("parkUntil>=90 " + (du >= 90));

        // 许可先授予：随后的 park 立即返回
        LockSupport.unpark(Thread.currentThread());
        t0 = System.nanoTime();
        LockSupport.parkNanos(2_000_000_000L);
        long dq = (System.nanoTime() - t0) / 1_000_000;
        System.out.println("pre-unparked park fast " + (dq < 1000));

        // 他线程 unpark 唤醒无限期 park
        final Thread main = Thread.currentThread();
        Thread waker = new Thread(() -> {
            try {
                Thread.sleep(100);
            } catch (InterruptedException e) {
                throw new RuntimeException(e);
            }
            LockSupport.unpark(main);
        });
        t0 = System.nanoTime();
        waker.start();
        LockSupport.park();
        long dr = (System.nanoTime() - t0) / 1_000_000;
        System.out.println("unparked by other >=90 " + (dr >= 90) + " fast " + (dr < 5000));
        waker.join();

        long prev = System.nanoTime();
        boolean mono = true;
        for (int i = 0; i < 1000; i++) {
            long now = System.nanoTime();
            if (now < prev) mono = false;
            prev = now;
        }
        System.out.println("nanoTime monotonic " + mono);
    }
}
