import java.time.Duration;
import java.util.ArrayList;
import java.util.List;
import java.util.concurrent.CountDownLatch;
import java.util.concurrent.atomic.AtomicInteger;
import java.util.concurrent.locks.LockSupport;

// 虚拟线程让出后（可能在另一载体上）恢复：currentThread 身份、ThreadLocal / InheritableThreadLocal 值、
// isVirtual、中断状态保持；join(Duration) 限时语义。
public class TestVirtualThreadCarrier {
    static final ThreadLocal<String> TL = new ThreadLocal<>();
    static final InheritableThreadLocal<String> ITL = new InheritableThreadLocal<>();

    public static void main(String[] args) throws Exception {
        ITL.set("inherited-from-main");
        int n = 16;
        AtomicInteger ok = new AtomicInteger();
        List<String> errors = new ArrayList<>();
        CountDownLatch start = new CountDownLatch(1);
        List<Thread> ts = new ArrayList<>();
        for (int i = 0; i < n; i++) {
            final int id = i;
            ts.add(Thread.ofVirtual().name("vt-" + id).start(() -> {
                try {
                    start.await();
                } catch (InterruptedException e) {
                    throw new RuntimeException(e);
                }
                Thread self = Thread.currentThread();
                TL.set("tl-" + id);
                String inherited = ITL.get();
                boolean good = true;
                for (int k = 0; k < 50; k++) {
                    if (k % 3 == 0) Thread.yield();
                    else if (k % 3 == 1) LockSupport.parkNanos(10_000);
                    else {
                        try {
                            Thread.sleep(1);
                        } catch (InterruptedException e) {
                            good = false;
                        }
                    }
                    good &= Thread.currentThread() == self;
                    good &= ("tl-" + id).equals(TL.get());
                    good &= "inherited-from-main".equals(ITL.get()) && inherited.equals(ITL.get());
                    good &= Thread.currentThread().isVirtual();
                    good &= Thread.currentThread().getName().equals("vt-" + id);
                }
                if (good) ok.incrementAndGet();
                else synchronized (errors) { errors.add("vt-" + id); }
            }));
        }
        start.countDown();
        for (Thread t : ts) t.join();
        System.out.println("identity/locals preserved: " + ok.get() + "/" + n + " errors=" + errors);
        System.out.println("main TL after: " + TL.get() + " ITL: " + ITL.get());

        // 中断状态跨让出保持；park 在已中断时立即返回且不清状态
        boolean[] f = new boolean[4];
        Thread it = Thread.ofVirtual().start(() -> {
            Thread.currentThread().interrupt();
            Thread.yield();
            f[0] = Thread.currentThread().isInterrupted();
            LockSupport.park();
            f[1] = Thread.currentThread().isInterrupted();
            try {
                Thread.sleep(10);
            } catch (InterruptedException e) {
                f[2] = true;
            }
            f[3] = Thread.currentThread().isInterrupted();
        });
        it.join();
        System.out.println("interrupt after yield=" + f[0] + " after park=" + f[1] + " sleep threw=" + f[2] + " after sleep=" + f[3]);

        // join(Duration)：超时返回 false，终结后返回 true
        Thread parked = Thread.ofVirtual().start(LockSupport::park);
        boolean early = parked.join(Duration.ofMillis(50));
        System.out.println("join(50ms) on parked: " + early + " alive=" + parked.isAlive());
        LockSupport.unpark(parked);
        boolean done = parked.join(Duration.ofSeconds(30));
        System.out.println("join after unpark: " + done + " alive=" + parked.isAlive() + " state=" + parked.getState());

        // 子虚拟线程继承父虚拟线程的 InheritableThreadLocal
        String[] child = new String[1];
        Thread parent = Thread.ofVirtual().start(() -> {
            ITL.set("from-virtual-parent");
            Thread c = Thread.ofVirtual().start(() -> child[0] = ITL.get());
            try {
                c.join();
            } catch (InterruptedException e) {
                throw new RuntimeException(e);
            }
        });
        parent.join();
        System.out.println("child ITL: " + child[0]);

        // 大量虚拟线程：每个都让出，全部完成
        int many = 10_000;
        AtomicInteger count = new AtomicInteger();
        List<Thread> lots = new ArrayList<>(many);
        for (int i = 0; i < many; i++) {
            lots.add(Thread.startVirtualThread(() -> {
                Thread.yield();
                count.incrementAndGet();
            }));
        }
        for (Thread t : lots) t.join();
        System.out.println("many: " + count.get());
    }
}
