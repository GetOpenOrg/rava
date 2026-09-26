import java.util.concurrent.ArrayBlockingQueue;
import java.util.concurrent.BlockingQueue;
import java.util.concurrent.CountDownLatch;
import java.util.concurrent.CyclicBarrier;
import java.util.concurrent.TimeUnit;
import java.util.concurrent.atomic.AtomicInteger;
import java.util.concurrent.locks.Condition;
import java.util.concurrent.locks.ReentrantLock;

/** j.u.c 同步器（AQS / park 路径）：CountDownLatch、CyclicBarrier、ArrayBlockingQueue、ReentrantLock + Condition。 */
public class TestJucSync {
    public static void main(String[] args) throws Exception {
        // CountDownLatch：3 个工作线程各自计数后 countDown，main await
        CountDownLatch latch = new CountDownLatch(3);
        AtomicInteger work = new AtomicInteger();
        for (int i = 0; i < 3; i++) {
            final int k = i;
            new Thread(() -> {
                work.addAndGet(k + 1);
                latch.countDown();
            }).start();
        }
        latch.await();
        System.out.println("latch work=" + work.get() + " count=" + latch.getCount());
        System.out.println("latch timed await on zero: " + latch.await(10, TimeUnit.MILLISECONDS));
        CountDownLatch never = new CountDownLatch(1);
        System.out.println("latch timed await timeout: " + never.await(50, TimeUnit.MILLISECONDS));

        // CyclicBarrier：3 方两轮，屏障动作计数
        AtomicInteger trips = new AtomicInteger();
        CyclicBarrier barrier = new CyclicBarrier(3, trips::incrementAndGet);
        Thread[] ps = new Thread[2];
        for (int i = 0; i < 2; i++) {
            ps[i] = new Thread(() -> {
                try {
                    barrier.await();
                    barrier.await();
                } catch (Exception e) {
                    throw new RuntimeException(e);
                }
            });
            ps[i].start();
        }
        barrier.await();
        barrier.await();
        for (Thread t : ps) t.join();
        System.out.println("barrier trips=" + trips.get());

        // ArrayBlockingQueue：生产者-消费者
        BlockingQueue<Integer> q = new ArrayBlockingQueue<>(2);
        Thread producer = new Thread(() -> {
            try {
                for (int i = 1; i <= 100; i++) q.put(i);
                q.put(-1);
            } catch (InterruptedException e) {
                throw new RuntimeException(e);
            }
        });
        producer.start();
        long sum = 0;
        while (true) {
            int v = q.take();
            if (v < 0) break;
            sum += v;
        }
        producer.join();
        System.out.println("queue sum=" + sum + " poll timeout=" + q.poll(30, TimeUnit.MILLISECONDS));

        // ReentrantLock + Condition：信号交接
        ReentrantLock lock = new ReentrantLock();
        Condition cond = lock.newCondition();
        final int[] state = new int[1];
        Thread signaller = new Thread(() -> {
            lock.lock();
            try {
                state[0] = 7;
                cond.signalAll();
            } finally {
                lock.unlock();
            }
        });
        lock.lock();
        try {
            signaller.start();
            while (state[0] == 0) {
                cond.await();
            }
            System.out.println("condition state=" + state[0] + " held=" + lock.isHeldByCurrentThread());
        } finally {
            lock.unlock();
        }
        signaller.join();
    }
}
