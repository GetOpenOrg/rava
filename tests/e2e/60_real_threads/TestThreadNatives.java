// FS-T6：Thread 的 setPriority0 / setNativeName / getThreads / getStackTrace0 native。
import java.util.*;
import java.util.concurrent.CountDownLatch;

public class TestThreadNatives {
    public static void main(String[] args) throws Exception {
        Thread main = Thread.currentThread();
        main.setPriority(Thread.MAX_PRIORITY - 1);
        System.out.println("main priority " + main.getPriority());
        main.setName("renamed-main");
        System.out.println("main name " + main.getName());

        CountDownLatch started = new CountDownLatch(1), release = new CountDownLatch(1);
        Thread worker = new Thread(() -> {
            started.countDown();
            try { release.await(); } catch (InterruptedException e) { }
        }, "worker-1");
        worker.setPriority(Thread.MIN_PRIORITY);
        worker.start();
        started.await();
        worker.setName("worker-renamed");
        System.out.println("worker priority " + worker.getPriority() + " name " + worker.getName());

        Map<Thread, StackTraceElement[]> all = Thread.getAllStackTraces();
        System.out.println("all has main " + all.containsKey(main) + " has worker " + all.containsKey(worker));
        Thread[] arr = new Thread[16];
        int n = Thread.enumerate(arr);
        Set<String> names = new TreeSet<>();
        for (int i = 0; i < n; i++) names.add(arr[i].getName());
        System.out.println("enumerate contains " + names.contains("renamed-main") + " " + names.contains("worker-renamed"));
        System.out.println("activeCount>=2 " + (Thread.activeCount() >= 2));
        release.countDown();
        worker.join();
        System.out.println("after join has worker " + Thread.getAllStackTraces().containsKey(worker));
        System.out.println("terminated trace " + worker.getStackTrace().length);
        try {
            main.setPriority(11);
        } catch (IllegalArgumentException e) {
            System.out.println("IAE priority");
        }
    }
}
