import java.util.concurrent.atomic.AtomicInteger;
import java.util.concurrent.atomic.AtomicLong;
import java.util.concurrent.atomic.AtomicBoolean;

public class AtomicDemo {
    public static void main(String[] args) {
        AtomicInteger ai = new AtomicInteger(10);
        System.out.println("initial: " + ai.get());
        System.out.println("getAndIncrement: " + ai.getAndIncrement());
        System.out.println("after: " + ai.get());
        System.out.println("incrementAndGet: " + ai.incrementAndGet());
        System.out.println("getAndDecrement: " + ai.getAndDecrement());
        System.out.println("after decrement: " + ai.get());
        System.out.println("addAndGet(5): " + ai.addAndGet(5));
        System.out.println("getAndAdd(3): " + ai.getAndAdd(3));
        System.out.println("after getAndAdd: " + ai.get());

        boolean cas = ai.compareAndSet(19, 100);
        System.out.println("CAS(19->100): " + cas + " val=" + ai.get());
        boolean cas2 = ai.compareAndSet(0, 999);
        System.out.println("CAS(0->999) failed: " + cas2 + " val=" + ai.get());

        System.out.println("getAndSet(42): " + ai.getAndSet(42));
        System.out.println("after getAndSet: " + ai.get());

        AtomicLong al = new AtomicLong(1000000000000L);
        System.out.println("long initial: " + al.get());
        System.out.println("long incrementAndGet: " + al.incrementAndGet());

        AtomicBoolean ab = new AtomicBoolean(false);
        System.out.println("bool initial: " + ab.get());
        ab.set(true);
        System.out.println("bool after set(true): " + ab.get());
        System.out.println("bool CAS(true->false): " + ab.compareAndSet(true, false));
        System.out.println("bool after CAS: " + ab.get());
    }
}
