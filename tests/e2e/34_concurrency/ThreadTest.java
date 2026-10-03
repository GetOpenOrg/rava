public class ThreadTest {
    public static void main(String[] args) throws InterruptedException {
        // Test 1: Thread.sleep
        System.out.println("=== Thread.sleep ===");
        Thread.sleep(50);
        System.out.println("Sleep completed");

        // Test 2: Thread(Runnable) + start + join
        System.out.println("=== Thread(Runnable) ===");
        Thread t1 = new Thread(() -> {
            System.out.println("Thread t1 running");
        });
        t1.start();
        t1.join();
        System.out.println("Thread t1 finished");

        // Test 3: Thread(Runnable, name) + getName
        System.out.println("=== Thread with name ===");
        Thread t2 = new Thread(() -> {
            System.out.println("Named thread running");
        }, "MyThread");
        System.out.println("Thread name: " + t2.getName());
        t2.start();
        t2.join();
        System.out.println("Named thread finished");

        // Test 4: Multiple threads, sequential join ensures order
        System.out.println("=== Multiple threads ===");
        for (int i = 0; i < 3; i++) {
            final int id = i;
            Thread t = new Thread(() -> {
                System.out.println("Worker " + id);
            });
            t.start();
            t.join();
        }
        System.out.println("All workers done");

        // Test 5: Thread.currentThread
        System.out.println("=== currentThread ===");
        Thread current = Thread.currentThread();
        System.out.println("Current thread: " + current.getName());
    }
}
