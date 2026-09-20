import java.util.ArrayList;
import java.util.Collections;
import java.util.List;

public class TestThreadJoin {

    static final List<String> log = Collections.synchronizedList(new ArrayList<>());

    static class Worker extends Thread {
        final String tag;
        final int times;

        Worker(String tag, int times) {
            super("worker-" + tag);
            this.tag = tag;
            this.times = times;
        }

        @Override
        public void run() {
            for (int i = 0; i < times; i++) {
                log.add(tag + i);
            }
        }
    }

    static class Rename extends Thread {
        Rename() {
            super("renamed-thread");
        }

        @Override
        public void run() {
            log.add("from-" + getName());
        }
    }

    public static void main(String[] args) throws InterruptedException {
        System.out.println("isAlive before=" + Thread.currentThread().isAlive());

        Worker w1 = new Worker("A", 3);
        Worker w2 = new Worker("B", 2);

        // join 之后读取结果，输出确定
        w1.start();
        w1.join();
        System.out.println("after join A, log=" + log);

        w2.start();
        w2.join();
        System.out.println("after join B, log=" + log);

        Rename r = new Rename();
        r.start();
        r.join();
        System.out.println("after join rename, log=" + log);

        // 两个线程同时 start，最后统一 join：结果数量确定（顺序不定，排序后打印）
        log.clear();
        Worker w3 = new Worker("C", 4);
        Worker w4 = new Worker("D", 4);
        w3.start();
        w4.start();
        w3.join();
        w4.join();
        List<String> sorted = new ArrayList<>(log);
        Collections.sort(sorted);
        System.out.println("sorted combined=" + sorted);
        System.out.println("combined size=" + log.size());

        // Thread 基本信息
        Thread current = Thread.currentThread();
        System.out.println("main alive=" + current.isAlive());
        System.out.println("workers joined alive=" + w3.isAlive() + " " + w4.isAlive());

        // sleep 后仍可 join（sleep 时长固定，不打印时间）
        Thread sleeper = new Thread(() -> {
            try {
                Thread.sleep(1);
                log.add("slept");
            } catch (InterruptedException e) {
                log.add("interrupted");
            }
        }, "sleeper");
        sleeper.start();
        sleeper.join();
        System.out.println("sleep log contains slept=" + log.contains("slept"));

        // 匿名 Runnable 线程
        Thread anon = new Thread(new Runnable() {
            @Override
            public void run() {
                log.add("runnable");
            }
        });
        anon.start();
        anon.join();
        System.out.println("after anon size=" + log.size() + " hasRunnable=" + log.contains("runnable"));

        System.out.println("done");
    }
}
