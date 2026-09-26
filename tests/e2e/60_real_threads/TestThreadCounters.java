import java.util.concurrent.atomic.AtomicInteger;

/** 真多线程：synchronized 计数 / AtomicInteger 计数 / 非守护线程在 main 结束后继续运行。 */
public class TestThreadCounters {
    static int syncCount = 0;
    static final Object LOCK = new Object();
    static final AtomicInteger atomic = new AtomicInteger();

    static synchronized void bumpStatic(int[] box) {
        box[0]++;
    }

    public static void main(String[] args) throws Exception {
        final int threads = 4, rounds = 2000;
        Thread[] ts = new Thread[threads];
        final int[] box = new int[1];
        for (int t = 0; t < threads; t++) {
            ts[t] = new Thread(() -> {
                for (int i = 0; i < rounds; i++) {
                    synchronized (LOCK) {
                        syncCount++;
                    }
                    atomic.incrementAndGet();
                    bumpStatic(box);
                    if (i % 500 == 0) {
                        Thread.yield();
                    }
                }
            }, "counter-" + t);
            ts[t].start();
        }
        for (Thread t : ts) {
            t.join();
        }
        System.out.println("sync=" + syncCount + " atomic=" + atomic.get() + " static=" + box[0]);
        for (Thread t : ts) {
            System.out.println(t.getName() + " alive=" + t.isAlive() + " state=" + t.getState());
        }

        // 非守护线程：main 返回后 JVM 等待其结束，其输出必须出现
        Thread tail = new Thread(() -> {
            try {
                Thread.sleep(150);
            } catch (InterruptedException e) {
                throw new RuntimeException(e);
            }
            System.out.println("tail thread done after main");
        });
        tail.start();
        System.out.println("main done");
    }
}
