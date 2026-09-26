import java.util.ArrayDeque;

/** 真多线程：wait / notifyAll 有界缓冲区（2 生产者 × 2 消费者）+ 限时 wait 超时。 */
public class TestWaitNotifyQueue {
    static final ArrayDeque<Integer> buf = new ArrayDeque<>();
    static final int CAP = 3;
    static long consumedSum = 0;
    static int consumedCount = 0;

    static void put(int v) throws InterruptedException {
        synchronized (buf) {
            while (buf.size() == CAP) {
                buf.wait();
            }
            buf.addLast(v);
            buf.notifyAll();
        }
    }

    static int take() throws InterruptedException {
        synchronized (buf) {
            while (buf.isEmpty()) {
                buf.wait();
            }
            int v = buf.removeFirst();
            buf.notifyAll();
            return v;
        }
    }

    public static void main(String[] args) throws Exception {
        final int perProducer = 500;
        Thread[] producers = new Thread[2];
        Thread[] consumers = new Thread[2];
        for (int p = 0; p < 2; p++) {
            final int base = p * perProducer;
            producers[p] = new Thread(() -> {
                try {
                    for (int i = 1; i <= perProducer; i++) {
                        put(base + i);
                    }
                } catch (InterruptedException e) {
                    throw new RuntimeException(e);
                }
            });
        }
        for (int c = 0; c < 2; c++) {
            consumers[c] = new Thread(() -> {
                try {
                    for (int i = 0; i < perProducer; i++) {
                        int v = take();
                        synchronized (TestWaitNotifyQueue.class) {
                            consumedSum += v;
                            consumedCount++;
                        }
                    }
                } catch (InterruptedException e) {
                    throw new RuntimeException(e);
                }
            });
        }
        for (Thread t : consumers) t.start();
        for (Thread t : producers) t.start();
        for (Thread t : producers) t.join();
        for (Thread t : consumers) t.join();
        System.out.println("count=" + consumedCount + " sum=" + consumedSum + " left=" + buf.size());

        // 限时 wait：无 notifier，按挂钟超时返回
        Object o = new Object();
        long t0 = System.nanoTime();
        synchronized (o) {
            o.wait(120);
        }
        long ms = (System.nanoTime() - t0) / 1_000_000;
        System.out.println("timed wait >= 120ms: " + (ms >= 120));

        // notify 唤醒单个等待者
        final boolean[] flag = new boolean[1];
        Thread waiter = new Thread(() -> {
            synchronized (o) {
                while (!flag[0]) {
                    try {
                        o.wait();
                    } catch (InterruptedException e) {
                        throw new RuntimeException(e);
                    }
                }
                System.out.println("waiter woke");
            }
        });
        waiter.start();
        Thread.sleep(50);
        synchronized (o) {
            flag[0] = true;
            o.notify();
        }
        waiter.join();
        System.out.println("done");
    }
}
