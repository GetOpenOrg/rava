import java.util.concurrent.CountDownLatch;
import java.util.concurrent.TimeUnit;
import java.util.concurrent.locks.LockSupport;

// 虚拟线程被 pin 时的阻塞：synchronized 内 sleep / park（持有监视器），<clinit> 内 sleep（类初始化）。
// 被 pin 的虚拟线程不卸载，在原载体上阻塞；状态、唤醒、中断与限时语义同未 pin 时。
public class TestContinuationPinned {
    static final Object LOCK = new Object();

    /** 当前载体线程名（VirtualThread.toString 的 "@" 之后部分） */
    static String carrier() {
        String s = Thread.currentThread().toString();
        int at = s.indexOf('@');
        return at < 0 ? "" : s.substring(at + 1);
    }

    static class Slow {
        static final int VALUE;
        static String before, after;
        static {
            before = carrier();
            try {
                Thread.sleep(40);
            } catch (InterruptedException e) {
                throw new RuntimeException(e);
            }
            after = carrier();
            VALUE = 42;
        }
    }

    public static void main(String[] args) throws Exception {
        // 1a. synchronized 内 sleep：不卸载，载体不变
        String[] r = new String[2];
        Thread t = Thread.ofVirtual().start(() -> {
            synchronized (LOCK) {
                r[0] = carrier();
                try {
                    Thread.sleep(30);
                } catch (InterruptedException e) {
                    throw new RuntimeException(e);
                }
                r[1] = carrier();
            }
        });
        t.join();
        System.out.println("sync sleep: sameCarrier=" + r[0].equals(r[1]) + " carrierNonEmpty=" + !r[0].isEmpty());

        // 1b. synchronized 内 park：状态 WAITING，unpark 唤醒，载体不变
        CountDownLatch entered = new CountDownLatch(1);
        Thread p = Thread.ofVirtual().start(() -> {
            synchronized (LOCK) {
                r[0] = carrier();
                entered.countDown();
                LockSupport.park();
                r[1] = carrier();
            }
        });
        entered.await();
        while (p.getState() != Thread.State.WAITING) Thread.onSpinWait();
        System.out.println("sync park: state=" + p.getState());
        LockSupport.unpark(p);
        p.join();
        System.out.println("sync park: resumed sameCarrier=" + r[0].equals(r[1]));

        // 1c. synchronized 内限时 sleep：TIMED_WAITING，被中断时抛 InterruptedException 并清中断状态
        CountDownLatch entered2 = new CountDownLatch(1);
        boolean[] flags = new boolean[2];
        Thread s = Thread.ofVirtual().start(() -> {
            synchronized (LOCK) {
                entered2.countDown();
                try {
                    Thread.sleep(60_000);
                } catch (InterruptedException e) {
                    flags[0] = true;
                    flags[1] = Thread.currentThread().isInterrupted();
                }
            }
        });
        entered2.await();
        while (s.getState() != Thread.State.TIMED_WAITING) Thread.onSpinWait();
        System.out.println("sync sleep: state=" + s.getState());
        s.interrupt();
        s.join();
        System.out.println("sync sleep: interrupted=" + flags[0] + " stillInterrupted=" + flags[1]);

        // 1d. synchronized 内限时 parkNanos 超时返回
        Thread n = Thread.ofVirtual().start(() -> {
            synchronized (LOCK) {
                long t0 = System.nanoTime();
                LockSupport.parkNanos(TimeUnit.MILLISECONDS.toNanos(30));
                flags[0] = System.nanoTime() - t0 >= TimeUnit.MILLISECONDS.toNanos(25);
            }
        });
        n.join();
        System.out.println("sync parkNanos: elapsed>=25ms " + flags[0]);

        // 2. <clinit> 内 sleep：两个虚拟线程同时触发初始化，后到者等待初始化完成
        int[] vals = new int[2];
        Thread c1 = Thread.ofVirtual().start(() -> vals[0] = Slow.VALUE);
        Thread c2 = Thread.ofVirtual().start(() -> vals[1] = Slow.VALUE);
        c1.join();
        c2.join();
        System.out.println("clinit sleep: values=" + vals[0] + "," + vals[1]
                + " sameCarrier=" + Slow.before.equals(Slow.after) + " carrierNonEmpty=" + !Slow.before.isEmpty());

        // 被 pin 的线程都已结束后，未 pin 的虚拟线程照常运行
        Thread last = Thread.startVirtualThread(() -> System.out.println("after: virtual=" + Thread.currentThread().isVirtual()));
        last.join();
    }
}
