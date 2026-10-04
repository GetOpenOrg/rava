import java.util.concurrent.atomic.AtomicInteger;
import java.util.concurrent.atomic.AtomicLong;
import java.util.concurrent.locks.LockSupport;

// 虚拟线程规模作业（计划 §21.8.4 a3-T6，服务器单独跑，不进 e2e）。
// 用法：VirtualThreadScale [n=1000000] [sleepMs=5000] [mode=sleep|park|unstarted]
//   sleep     ：n 个虚拟线程各 sleep(sleepMs)（经延时调度器 UNPARKER 唤醒）
//   park      ：n 个虚拟线程各 park，全部停泊后主线程逐个 unpark（无延时调度器开销）
//   unstarted ：只构造 n 个未启动的虚拟线程并持有 sleepMs（VirtualThread + Continuation 对象开销）
// stdout 只打印与平台无关的结果；阶段耗时（ms，相对起点）打印到 stderr，带墙钟毫秒供 RSS 采样对齐。
public class VirtualThreadScale {
    static long t0;

    static void phase(String name) {
        System.err.println("phase " + name + " +" + (System.nanoTime() - t0) / 1_000_000 + "ms wall=" + System.currentTimeMillis());
    }

    public static void main(String[] args) throws Exception {
        final int n = args.length > 0 ? Integer.parseInt(args[0]) : 1_000_000;
        final long sleepMs = args.length > 1 ? Long.parseLong(args[1]) : 5000;
        final String mode = args.length > 2 ? args[2] : "sleep";
        AtomicInteger started = new AtomicInteger();
        AtomicInteger finished = new AtomicInteger();
        AtomicInteger lateStarts = new AtomicInteger();
        AtomicLong sum = new AtomicLong();
        Thread[] ts = new Thread[n];
        t0 = System.nanoTime();
        phase("begin");
        for (int i = 0; i < n; i++) {
            final int id = i;
            Runnable body = () -> {
                if (finished.get() > 0) lateStarts.incrementAndGet();
                started.incrementAndGet();
                if (mode.equals("park")) {
                    LockSupport.park();
                } else {
                    try {
                        Thread.sleep(sleepMs);
                    } catch (InterruptedException e) {
                        throw new RuntimeException(e);
                    }
                }
                sum.addAndGet(id);
                finished.incrementAndGet();
            };
            ts[i] = mode.equals("unstarted") ? Thread.ofVirtual().unstarted(body) : Thread.ofVirtual().start(body);
        }
        phase("created");
        if (mode.equals("unstarted")) {
            Thread.sleep(sleepMs);
            phase("held");
            System.out.println("unstarted: " + n);
            return;
        }
        while (started.get() < n) Thread.sleep(10);
        phase("all-started");
        if (mode.equals("park")) {
            Thread.sleep(sleepMs);
            phase("unpark");
            for (Thread t : ts) LockSupport.unpark(t);
        }
        for (Thread t : ts) t.join();
        phase("joined");
        int alive = 0;
        for (Thread t : ts) if (t.isAlive()) alive++;
        System.out.println("finished: " + finished.get());
        System.out.println("sum: " + sum.get());
        System.out.println("all sleeping at once: " + (lateStarts.get() == 0));
        System.out.println("alive after join: " + alive);
    }
}
