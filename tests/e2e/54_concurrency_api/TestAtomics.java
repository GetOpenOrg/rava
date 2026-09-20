import java.util.concurrent.atomic.*;
import java.util.concurrent.CountDownLatch;

public class TestAtomics {
    public static void main(String[] args) throws Exception {
        AtomicInteger ai = new AtomicInteger(0);
        for (int v : new int[]{1, 2, 3, 4, 5}) ai.addAndGet(v);
        System.out.println("ai=" + ai.get());

        AtomicReference<String> ar = new AtomicReference<>("init");
        ar.set("set");
        System.out.println("ar=" + ar.get());

        AtomicLong al = new AtomicLong(100);
        System.out.println("al=" + al.decrementAndGet());

        AtomicBoolean ab = new AtomicBoolean(true);
        System.out.println("ab=" + ab.getAndSet(false));
        System.out.println("ab2=" + ab.get());

        CountDownLatch latch = new CountDownLatch(3);
        for (int i = 0; i < 3; i++) latch.countDown();
        System.out.println("latch=" + latch.getCount());

        AtomicIntegerArray aia = new AtomicIntegerArray(3);
        aia.set(0, 10);
        aia.set(1, 20);
        System.out.println("aia=" + aia.get(0) + "," + aia.get(1));
    }
}
