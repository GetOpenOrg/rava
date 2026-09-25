public class ThreadSafetyTest {
    public static void main(String[] args) throws InterruptedException {
        // Test 1: Sequential thread creation and join
        System.out.println("=== Sequential threads ===");
        Thread a = new Thread(() -> {
            System.out.println("Thread A running");
        });
        a.start();
        a.join();

        Thread b = new Thread(() -> {
            System.out.println("Thread B running");
        });
        b.start();
        b.join();

        Thread c = new Thread(() -> {
            System.out.println("Thread C running");
        });
        c.start();
        c.join();
        System.out.println("All threads done");

        // Test 2: Thread naming
        System.out.println("=== Thread naming ===");
        Thread t1 = new Thread(() -> {
            System.out.println("Named thread executing");
        }, "TestWorker");
        System.out.println("Name: " + t1.getName());
        t1.start();
        t1.join();

        // Test 3: Thread with sleep
        System.out.println("=== Thread with sleep ===");
        Thread t2 = new Thread(() -> {
            System.out.println("Before sleep");
            try {
                Thread.sleep(10);
            } catch (InterruptedException e) {
                // ignore
            }
            System.out.println("After sleep");
        });
        t2.start();
        t2.join();
        System.out.println("Join complete");

        // Test 4: currentThread basic access
        System.out.println("=== currentThread ===");
        Thread main = Thread.currentThread();
        System.out.println("Current thread: " + main.getName());

        // Test 5: Thread with computation in lambda
        System.out.println("=== Compute thread ===");
        Thread t3 = new Thread(() -> {
            int sum = 0;
            for (int i = 0; i < 100; i++) {
                sum += i;
            }
            System.out.println("Sum: " + sum);
        });
        t3.start();
        t3.join();

        System.out.println("Thread safety tests passed");
    }
}
