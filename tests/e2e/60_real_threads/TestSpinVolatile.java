/** 真多线程：自旋等待他线程写入的 volatile 字段 / 数组元素（安全点让出）；Thread.yield。 */
public class TestSpinVolatile {
    static volatile boolean ready = false;
    static volatile int stage = 0;
    static final int[] slots = new int[4];

    public static void main(String[] args) throws Exception {
        Thread writer = new Thread(() -> {
            try {
                Thread.sleep(50);
            } catch (InterruptedException e) {
                throw new RuntimeException(e);
            }
            stage = 1;
            ready = true;
        });
        writer.start();
        long spins = 0;
        while (!ready) {
            spins++;
        }
        System.out.println("saw ready, stage=" + stage + " spun=" + (spins > 0));
        writer.join();

        // 乒乓：两线程交替推进 stage（纯自旋，无阻塞原语）
        Thread pong = new Thread(() -> {
            for (int i = 0; i < 5; i++) {
                while (stage % 2 == 1) {
                    Thread.yield();
                }
                stage++;
            }
        });
        stage = 1;
        pong.start();
        for (int i = 0; i < 5; i++) {
            while (stage % 2 == 0) {
                // 忙等（无 yield）：依赖安全点让出
            }
            stage++;
        }
        pong.join();
        System.out.println("pingpong stage=" + stage);

        Thread arr = new Thread(() -> {
            for (int i = 0; i < slots.length; i++) {
                slots[i] = i + 1;
            }
        });
        arr.start();
        while (slots[slots.length - 1] == 0) {
        }
        arr.join();
        System.out.println("slots last=" + slots[3]);
    }
}
