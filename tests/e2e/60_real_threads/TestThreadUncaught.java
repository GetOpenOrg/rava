/** 线程未捕获异常只终结该线程（stderr 报告），join 正常返回，其余线程与 main 继续。 */
public class TestThreadUncaught {
    public static void main(String[] args) throws Exception {
        Thread bad = new Thread(() -> {
            throw new IllegalStateException("worker failed");
        }, "bad-worker");
        bad.start();
        bad.join();
        System.out.println("bad alive=" + bad.isAlive() + " state=" + bad.getState());

        final StringBuilder sb = new StringBuilder();
        Thread good = new Thread(() -> sb.append("good ran"));
        good.start();
        good.join();
        System.out.println(sb);

        Thread d = new Thread(() -> {
            try {
                Thread.sleep(10_000);
            } catch (InterruptedException e) {
                throw new RuntimeException(e);
            }
            System.out.println("daemon must not print");
        });
        d.setDaemon(true);
        d.start();
        System.out.println("daemon=" + d.isDaemon() + " main exits without waiting");
    }
}
