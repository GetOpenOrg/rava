import java.util.concurrent.atomic.AtomicInteger;
import java.util.concurrent.Executors;

public class TestVirtualThread {
    public static void main(String[] args) throws Exception {
        AtomicInteger counter = new AtomicInteger(0);
        int n = 10;
        Thread[] ts = new Thread[n];
        for (int i = 0; i < n; i++) {
            ts[i] = Thread.startVirtualThread(() -> counter.incrementAndGet());
        }
        for (Thread t : ts) t.join();
        System.out.println("count=" + counter.get());

        try (var ex = Executors.newVirtualThreadPerTaskExecutor()) {
            int sum = 0;
            for (int i = 0; i < 5; i++) sum += (Integer) ex.submit(() -> 1).get();
            System.out.println("execSum=" + sum);
        }
    }
}
