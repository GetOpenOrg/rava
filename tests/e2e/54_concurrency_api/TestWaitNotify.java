public class TestWaitNotify {
    static final Object lock = new Object();
    static int value = 0;

    public static void main(String[] args) throws Exception {
        Thread consumer = new Thread(() -> {
            synchronized (lock) {
                try {
                    while (value == 0) lock.wait();
                } catch (InterruptedException e) {
                    Thread.currentThread().interrupt();
                }
                System.out.println("consumed=" + value);
            }
        });
        Thread producer = new Thread(() -> {
            synchronized (lock) {
                value = 42;
                lock.notify();
            }
        });
        consumer.start();
        producer.start();
        consumer.join();
        producer.join();
        System.out.println("done=" + value);
    }
}
