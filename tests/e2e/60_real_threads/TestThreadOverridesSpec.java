// FS-H0：Thread.interrupt / isTerminated / getThreadGroup 与 Character.digit、
// AtomicInteger(int) 改走字节码翻译后的规格行为。
import java.util.concurrent.atomic.AtomicInteger;

public class TestThreadOverridesSpec {
    static volatile boolean mainSawInterrupt;

    public static void main(String[] args) throws Exception {
        // 1. Character.digit：ASCII / 全角 / 阿拉伯-印度数字 / 越界 radix / 非数字
        int[] cps = {'7', 'f', 'Z', 0xFF13, 0x0663, 0x0966, 0xFF21, '$', 0x10FFFF};
        int[] radices = {10, 16, 36, 2, 37};
        for (int cp : cps) {
            StringBuilder sb = new StringBuilder(Integer.toHexString(cp)).append(':');
            for (int r : radices) sb.append(' ').append(Character.digit(cp, r));
            System.out.println(sb);
        }
        System.out.println("digit(char) " + Character.digit('５', 10) + " parse " + Integer.parseInt("١٢٣"));

        // 2. getThreadGroup：存活时为构造组，终结后为 null；isAlive 翻转
        Thread t = new Thread(() -> {}, "short");
        System.out.println("before start group=" + (t.getThreadGroup() != null) + " state=" + t.getState());
        t.start();
        t.join();
        System.out.println("after join group=" + t.getThreadGroup() + " alive=" + t.isAlive() + " state=" + t.getState());

        // 3. 他线程中断主线程：主线程 sleep 中收到 InterruptedException
        Thread main = Thread.currentThread();
        Thread killer = new Thread(() -> {
            try { Thread.sleep(50); } catch (InterruptedException e) { }
            main.interrupt();
        });
        killer.start();
        try {
            Thread.sleep(10_000);
            System.out.println("sleep not interrupted");
        } catch (InterruptedException e) {
            System.out.println("main interrupted: " + e.getMessage() + " flag=" + Thread.currentThread().isInterrupted());
        }
        killer.join();

        // 4. 自中断：标志置位 → interrupted() 清除
        Thread.currentThread().interrupt();
        System.out.println("self flag=" + Thread.currentThread().isInterrupted() + " cleared=" + Thread.interrupted() + " now=" + Thread.currentThread().isInterrupted());

        // 5. 中断未启动 / 已终结线程：只置位，无异常
        Thread idle = new Thread(() -> {});
        idle.interrupt();
        System.out.println("unstarted interrupted=" + idle.isInterrupted());
        t.interrupt();
        System.out.println("terminated interrupted=" + t.isInterrupted());

        // 6. AtomicInteger(int) 构造器
        AtomicInteger ai = new AtomicInteger(41);
        System.out.println("atomic " + ai.incrementAndGet() + " " + new AtomicInteger().get());
    }
}
