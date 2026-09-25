public class SynchronizedTest {
    private int counter = 0;

    public synchronized void increment() {
        counter++;
    }

    public synchronized int getCounter() {
        return counter;
    }

    // synchronized block
    public void blockIncrement() {
        synchronized (this) {
            counter += 10;
        }
    }

    public static void main(String[] args) {
        SynchronizedTest t = new SynchronizedTest();
        t.increment();
        t.increment();
        t.increment();
        System.out.println(t.getCounter()); // 3
        t.blockIncrement();
        System.out.println(t.getCounter()); // 13
        System.out.println("Done");
    }
}
