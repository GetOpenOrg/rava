import java.util.concurrent.atomic.AtomicInteger;

/** JVMS §5.5：多线程并发首次触发类初始化——<clinit> 只执行一次，其余线程阻塞至完成后看到初始化后的值。 */
public class TestConcurrentClinit {
    static final AtomicInteger runs = new AtomicInteger();

    static class Slow {
        static final int VALUE;
        static {
            runs.incrementAndGet();
            try {
                Thread.sleep(150);
            } catch (InterruptedException e) {
                throw new RuntimeException(e);
            }
            VALUE = 42;
        }
        static int value() {
            return VALUE;
        }
    }

    static class Boom {
        static int X;
        static {
            if (runs.get() >= 0) {
                throw new IllegalStateException("boom");
            }
        }
    }

    public static void main(String[] args) throws Exception {
        final int n = 4;
        final int[] seen = new int[n];
        Thread[] ts = new Thread[n];
        for (int i = 0; i < n; i++) {
            final int k = i;
            ts[i] = new Thread(() -> seen[k] = Slow.value());
        }
        for (Thread t : ts) t.start();
        for (Thread t : ts) t.join();
        StringBuilder sb = new StringBuilder();
        for (int v : seen) sb.append(v).append(' ');
        System.out.println("seen=" + sb.toString().trim() + " clinit runs=" + runs.get());

        // 初始化失败：首次 ExceptionInInitializerError，他线程随后 NoClassDefFoundError
        try {
            Boom.X = 1;
        } catch (ExceptionInInitializerError e) {
            System.out.println("first: EIIE cause=" + e.getCause().getMessage());
        }
        final String[] other = new String[1];
        Thread t2 = new Thread(() -> {
            try {
                Boom.X = 2;
                other[0] = "no error";
            } catch (NoClassDefFoundError e) {
                other[0] = "NCDFE";
            }
        });
        t2.start();
        t2.join();
        System.out.println("other thread: " + other[0]);
    }
}
