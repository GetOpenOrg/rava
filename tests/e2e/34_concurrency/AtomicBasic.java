import java.util.concurrent.atomic.AtomicInteger;
import java.util.concurrent.atomic.AtomicLong;
import java.util.concurrent.atomic.AtomicBoolean;

public class AtomicBasic {
    public static void main(String[] args) {
        // AtomicInteger
        AtomicInteger ai = new AtomicInteger(10);
        System.out.println(ai.get());
        System.out.println(ai.incrementAndGet());
        System.out.println(ai.decrementAndGet());
        System.out.println(ai.addAndGet(5));
        System.out.println(ai.getAndSet(100));
        System.out.println(ai.get());

        // compareAndSet
        System.out.println(ai.compareAndSet(100, 200));
        System.out.println(ai.compareAndSet(100, 300)); // should fail
        System.out.println(ai.get());

        // getAndAdd / getAndIncrement
        AtomicInteger ai2 = new AtomicInteger(0);
        System.out.println(ai2.getAndAdd(5));
        System.out.println(ai2.get());
        System.out.println(ai2.getAndIncrement());
        System.out.println(ai2.get());

        // AtomicLong
        AtomicLong al = new AtomicLong(1000000000L);
        System.out.println(al.get());
        System.out.println(al.incrementAndGet());
        System.out.println(al.addAndGet(-1));

        // AtomicBoolean
        AtomicBoolean ab = new AtomicBoolean(false);
        System.out.println(ab.get());
        ab.set(true);
        System.out.println(ab.get());
        System.out.println(ab.compareAndSet(true, false));
        System.out.println(ab.get());
        System.out.println(ab.getAndSet(true));
        System.out.println(ab.get());

        System.out.println("done");
    }
}
