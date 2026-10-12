
import java.util.concurrent.CountDownLatch;
import java.util.concurrent.locks.Condition;
import java.util.concurrent.locks.Lock;
import java.util.concurrent.locks.ReentrantLock;

/**
 * 知识点5：手写简化版屏障，还原 CyclicBarrier 的核心原理。
 * 结构 = ReentrantLock + Condition + 计数器：
 * 先到的线程 condition.await() 挂起，最后一个到达的线程重置计数并 signalAll()。
 */
public class SimpleBarrier {

    private int count;
    private final int parties;
    private final Lock lock = new ReentrantLock();
    private final Condition condition = lock.newCondition();

    public SimpleBarrier(int parties) {
        this.parties = parties;
        this.count = parties;
    }

    public void await() throws InterruptedException {
        lock.lock();
        try {
            count--;
            if (count == 0) {
                count = parties;      // 重置计数，支持复用
                condition.signalAll(); // 最后一个线程唤醒所有人
            } else {
                condition.await();     // 未到齐则挂起等待
            }
        } finally {
            lock.unlock();
        }
    }

    public static void main(String[] args) throws InterruptedException {
        SimpleBarrier barrier = new SimpleBarrier(3);
        CountDownLatch done = new CountDownLatch(3);
        for (int i = 1; i <= 3; i++) {
            final int id = i;
            new Thread(() -> {
                try {
                    Thread.sleep(id * 50L); // 错峰：固定到达顺序
                    System.out.println("线程" + id + " 到达（简化版屏障）");
                    barrier.await();
                    Thread.sleep(id * 50L); // 错峰：固定释放顺序
                    System.out.println("线程" + id + " 被释放");
                } catch (InterruptedException e) {
                    Thread.currentThread().interrupt();
                } finally {
                    done.countDown();
                }
            }).start();
        }
        done.await(); // 主线程等所有线程结束后退出，输出完整
    }
}
