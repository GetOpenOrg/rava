import java.util.concurrent.atomic.AtomicInteger;
import java.util.concurrent.atomic.AtomicBoolean;
import java.util.concurrent.atomic.AtomicReference;

public class ConcurrentBasicTest {
    public static void main(String[] args) {
        // AtomicInteger
        AtomicInteger ai = new AtomicInteger(10);
        System.out.println("get: " + ai.get());
        System.out.println("getAndIncrement: " + ai.getAndIncrement());
        System.out.println("after increment: " + ai.get());
        System.out.println("incrementAndGet: " + ai.incrementAndGet());
        System.out.println("getAndDecrement: " + ai.getAndDecrement());
        System.out.println("decrementAndGet: " + ai.decrementAndGet());
        System.out.println("getAndAdd(5): " + ai.getAndAdd(5));
        System.out.println("addAndGet(3): " + ai.addAndGet(3));
        System.out.println("getAndSet(100): " + ai.getAndSet(100));
        System.out.println("after set: " + ai.get());
        System.out.println("compareAndSet(100,200): " + ai.compareAndSet(100, 200));
        System.out.println("after CAS: " + ai.get());
        System.out.println("compareAndSet(0,999): " + ai.compareAndSet(0, 999));
        System.out.println("after failed CAS: " + ai.get());
        System.out.println("intValue: " + ai.intValue());

        // AtomicBoolean
        AtomicBoolean ab = new AtomicBoolean(false);
        System.out.println("ab.get: " + ab.get());
        ab.set(true);
        System.out.println("ab after set: " + ab.get());
        System.out.println("ab.getAndSet(false): " + ab.getAndSet(false));
        System.out.println("ab after getAndSet: " + ab.get());
        System.out.println("ab.compareAndSet(false,true): " + ab.compareAndSet(false, true));
        System.out.println("ab after CAS: " + ab.get());

        // AtomicReference
        AtomicReference<String> ar = new AtomicReference<>("hello");
        System.out.println("ar.get: " + ar.get());
        ar.set("world");
        System.out.println("ar after set: " + ar.get());
        System.out.println("ar.getAndSet: " + ar.getAndSet("foo"));
        System.out.println("ar after getAndSet: " + ar.get());
    }
}
