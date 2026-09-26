import java.util.concurrent.locks.LockSupport;

/**
 * 协作调度的虚拟时钟（N12）：限时 park / wait / sleep 在无其他线程可推进时只能超时返回，
 * 返回时观测到的流逝时间必须 ≥ 时限（JVM 真实驻留；原生侧以时钟前移兑现，不空转）。
 * 覆盖：parkNanos（相对）、parkUntil（绝对）、泵内有就绪线程时先运行该线程、
 * Object.wait(ms) 无通知者超时、Thread.sleep、currentTimeMillis 与 nanoTime 同向推进。
 */
public class TestVirtualClockPark {
    static volatile boolean ran;

    public static void main(String[] args) throws Exception {
        long t0 = System.nanoTime();
        LockSupport.parkNanos(2_000_000_000L);
        long dt = System.nanoTime() - t0;
        System.out.println("parkNanos elapsed>=2s: " + (dt >= 2_000_000_000L));

        long m0 = System.currentTimeMillis();
        LockSupport.parkUntil(m0 + 1500);
        System.out.println("parkUntil reached deadline: " + (System.currentTimeMillis() >= m0 + 1500));

        Thread t = new Thread(() -> { System.out.println("worker ran"); ran = true; });
        t.start();
        while (!ran) {
            LockSupport.parkNanos(1_000_000_000L);
        }
        System.out.println("main saw worker: " + ran);
        t.join();

        Object lock = new Object();
        long w0 = System.nanoTime();
        synchronized (lock) {
            lock.wait(1200);
        }
        System.out.println("wait(1200) elapsed>=1200ms: " + (System.nanoTime() - w0 >= 1_200_000_000L));

        long s0 = System.nanoTime();
        long sm0 = System.currentTimeMillis();
        Thread.sleep(800);
        System.out.println("sleep(800) nano>=800ms: " + (System.nanoTime() - s0 >= 800_000_000L));
        System.out.println("sleep(800) millis>=800: " + (System.currentTimeMillis() - sm0 >= 800));
    }
}
