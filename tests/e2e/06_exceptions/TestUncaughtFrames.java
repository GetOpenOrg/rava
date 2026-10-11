// 线程未捕获异常经 Thread.dispatchUncaughtException → 处理器：缺省处理器在 System.err 上打
// `Exception in thread "名" ` + printStackTrace（含 at 帧、Caused by 与 "... n more"）；自定义处理器 / 默认处理器生效。
public class TestUncaughtFrames {
    static void fail(int depth) {
        if (depth == 0) {
            throw new IllegalStateException("inner failure");
        }
        fail(depth - 1);
    }

    static class Worker extends Thread {
        Worker(String name) { super(name); }

        @Override
        public void run() {
            try {
                fail(2);
            } catch (IllegalStateException e) {
                throw new RuntimeException("worker failed", e);
            }
        }
    }

    public static void main(String[] args) throws Exception {
        System.setErr(System.out);
        Worker w = new Worker("worker-a");
        w.start();
        w.join();
        System.out.println("state after: " + w.getState());

        Thread custom = new Thread(() -> { throw new UnsupportedOperationException("custom"); }, "worker-b");
        custom.setUncaughtExceptionHandler((t, e) ->
            System.out.println("handler: " + t.getName() + " " + e));
        custom.start();
        custom.join();

        Thread.setDefaultUncaughtExceptionHandler((t, e) ->
            System.out.println("default handler: " + t.getName() + " " + e.getMessage()));
        Thread dflt = new Thread(() -> { throw new ArithmeticException("by default"); }, "worker-c");
        dflt.start();
        dflt.join();
        System.out.println("done");
    }
}
