import java.util.concurrent.locks.LockSupport;

/** 中断语义：sleep / wait / join 抛 InterruptedException 并清状态；park 返回并保留状态；预先中断的立即生效。 */
public class TestThreadInterrupt {
    static final Object lock = new Object();

    public static void main(String[] args) throws Exception {
        // 1. 他线程中断 sleep
        final String[] r = new String[1];
        Thread sleeper = new Thread(() -> {
            try {
                Thread.sleep(10_000);
                r[0] = "slept fully";
            } catch (InterruptedException e) {
                r[0] = "sleep interrupted: " + e.getMessage() + " flag=" + Thread.currentThread().isInterrupted();
            }
        });
        long t0 = System.nanoTime();
        sleeper.start();
        Thread.sleep(50);
        sleeper.interrupt();
        sleeper.join();
        System.out.println(r[0] + " fast=" + ((System.nanoTime() - t0) / 1_000_000 < 5000));

        // 2. 预先中断的 sleep 立即抛出
        Thread.currentThread().interrupt();
        try {
            Thread.sleep(10_000);
            System.out.println("no throw");
        } catch (InterruptedException e) {
            System.out.println("pre-interrupted sleep: " + e.getMessage() + " flag=" + Thread.currentThread().isInterrupted());
        }

        // 3. 他线程中断 wait
        Thread waiter = new Thread(() -> {
            synchronized (lock) {
                try {
                    lock.wait();
                    r[0] = "woke normally";
                } catch (InterruptedException e) {
                    r[0] = "wait interrupted msg=" + e.getMessage() + " holds=" + Thread.holdsLock(lock)
                            + " flag=" + Thread.currentThread().isInterrupted();
                }
            }
        });
        waiter.start();
        Thread.sleep(50);
        waiter.interrupt();
        waiter.join();
        System.out.println(r[0]);

        // 4. 预先中断的 wait
        Thread.currentThread().interrupt();
        synchronized (lock) {
            try {
                lock.wait(10_000);
                System.out.println("no throw");
            } catch (InterruptedException e) {
                System.out.println("pre-interrupted wait flag=" + Thread.currentThread().isInterrupted());
            }
        }

        // 5. park 被中断：返回且保留中断状态；随后 park 立即返回
        Thread parker = new Thread(() -> {
            LockSupport.park();
            boolean f1 = Thread.currentThread().isInterrupted();
            long s = System.nanoTime();
            LockSupport.parkNanos(5_000_000_000L);
            boolean fast = (System.nanoTime() - s) / 1_000_000 < 1000;
            boolean cleared = Thread.interrupted();
            r[0] = "park returned flag=" + f1 + " reparkFast=" + fast + " interrupted()=" + cleared
                    + " after=" + Thread.currentThread().isInterrupted();
        });
        parker.start();
        Thread.sleep(50);
        parker.interrupt();
        parker.join();
        System.out.println(r[0]);

        // 6. join 被中断
        Thread longRunner = new Thread(() -> {
            try {
                Thread.sleep(300);
            } catch (InterruptedException e) {
                throw new RuntimeException(e);
            }
        });
        longRunner.start();
        Thread.currentThread().interrupt();
        try {
            longRunner.join();
            System.out.println("join no throw");
        } catch (InterruptedException e) {
            System.out.println("join interrupted flag=" + Thread.currentThread().isInterrupted());
        }
        longRunner.join();

        // 7. Thread.interrupted() 清状态
        Thread.currentThread().interrupt();
        System.out.println("interrupted()=" + Thread.interrupted() + " then=" + Thread.interrupted());
    }
}
