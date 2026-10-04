import java.util.concurrent.CountDownLatch;
import java.util.concurrent.TimeUnit;
import java.util.concurrent.locks.LockSupport;

// a3-T pinned 诊断：重复 TestContinuationPinned 的 1c（pin 中 sleep 被中断）+ 1d（pin 中 parkNanos 30ms），
// 统计 1d 提前返回的次数，及其与「n 落在 s 的载体上」的相关性；mode=nointr 去掉 1c 的中断（s 改为 parkNanos 10ms 自然超时）。
// 用法：PinnedRace [rounds=200] [mode=intr|nointr]
public class PinnedRace {
    static final Object LOCK = new Object();

    static String carrier() {
        String s = Thread.currentThread().toString();
        int at = s.indexOf('@');
        return at < 0 ? "" : s.substring(at + 1);
    }

    public static void main(String[] args) throws Exception {
        int rounds = args.length > 0 ? Integer.parseInt(args[0]) : 200;
        boolean intr = !(args.length > 1 && args[1].equals("nointr"));
        int early = 0, earlySame = 0, same = 0, stateFlips = 0;
        for (int i = 0; i < rounds; i++) {
            String[] cs = new String[2];
            CountDownLatch entered = new CountDownLatch(1);
            Thread s = Thread.ofVirtual().start(() -> {
                synchronized (LOCK) {
                    cs[0] = carrier();
                    entered.countDown();
                    if (intr) {
                        try {
                            Thread.sleep(60_000);
                        } catch (InterruptedException e) {
                        }
                    } else {
                        LockSupport.parkNanos(TimeUnit.MILLISECONDS.toNanos(10));
                    }
                }
            });
            entered.await();
            if (intr) {
                while (s.getState() != Thread.State.TIMED_WAITING) Thread.onSpinWait();
                if (s.getState() != Thread.State.TIMED_WAITING) stateFlips++;
                s.interrupt();
            }
            s.join();
            long[] el = new long[1];
            Thread n = Thread.ofVirtual().start(() -> {
                synchronized (LOCK) {
                    cs[1] = carrier();
                    long t0 = System.nanoTime();
                    LockSupport.parkNanos(TimeUnit.MILLISECONDS.toNanos(30));
                    el[0] = System.nanoTime() - t0;
                }
            });
            n.join();
            boolean sm = cs[0].equals(cs[1]);
            if (sm) same++;
            if (el[0] < TimeUnit.MILLISECONDS.toNanos(25)) {
                early++;
                if (sm) earlySame++;
            }
        }
        System.out.println("mode=" + (intr ? "intr" : "nointr") + " rounds=" + rounds + " early=" + early
                + " earlySameCarrier=" + earlySame + " sameCarrier=" + same + " stateFlips=" + stateFlips);
    }
}
