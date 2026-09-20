public class TestSynchronized {

    static int sharedCounter = 0;
    static final Object lock = new Object();
    static StringBuilder trace = new StringBuilder();

    static synchronized void bumpSyncMethod() {
        sharedCounter++;
        trace.append("m");
    }

    static void bumpSyncBlock() {
        // 与静态同步方法使用同一个监视器（Class 对象），保证互斥语义一致
        synchronized (TestSynchronized.class) {
            sharedCounter++;
            trace.append("b");
        }
    }

    static void bumpOtherMonitor() {
        synchronized (lock) {
            trace.append("o");
        }
    }

    static void bumpSyncThis(Object monitor) {
        synchronized (monitor) {
            sharedCounter++;
            trace.append("t");
        }
    }

    static synchronized int readCounter() {
        return sharedCounter;
    }

    // 可重入：同步方法里再进同一个锁
    static synchronized void reentrant(int depth) {
        if (depth == 0) {
            trace.append("!");
            return;
        }
        reentrant(depth - 1);
        trace.append("r");
    }

    static class CounterHolder {
        int value = 0;

        synchronized void inc() {
            value++;
        }

        int get() {
            synchronized (this) {
                return value;
            }
        }
    }

    static class Worker extends Thread {
        final int times;
        final boolean useMethod;

        Worker(String name, int times, boolean useMethod) {
            super(name);
            this.times = times;
            this.useMethod = useMethod;
        }

        @Override
        public void run() {
            for (int i = 0; i < times; i++) {
                if (useMethod) {
                    bumpSyncMethod();
                } else {
                    bumpSyncBlock();
                }
            }
        }
    }

    public static void main(String[] args) throws InterruptedException {
        // 单线程下同步语义
        bumpSyncMethod();
        bumpSyncBlock();
        bumpSyncThis(lock);
        bumpOtherMonitor();
        System.out.println("counter=" + readCounter());
        System.out.println("trace=" + trace);

        // 可重入
        trace.setLength(0);
        reentrant(3);
        System.out.println("reentrant trace=" + trace);

        // 实例级同步
        CounterHolder holder = new CounterHolder();
        for (int i = 0; i < 5; i++) {
            holder.inc();
        }
        System.out.println("holder=" + holder.get());

        // 多线程 + join：join 之后读取结果，输出确定
        sharedCounter = 0;
        Thread t1 = new Worker("w1", 1000, true);
        Thread t2 = new Worker("w2", 1000, false);
        t1.start();
        t2.start();
        t1.join();
        t2.join();
        System.out.println("multithread counter=" + sharedCounter);

        System.out.println("done");
    }
}
