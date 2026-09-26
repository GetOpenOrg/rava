/** main 正常返回后 JVM 运行 shutdown hook（DestroyJavaVM → Shutdown.shutdown）；hook 在新线程上执行。 */
public class TestShutdownHooks {
    public static void main(String[] args) {
        // 多个 hook 由 JDK 并发启动、输出序不确定：只保留一个打印的 hook
        Thread b = new Thread(() -> System.out.println("hook B ran on " + (Thread.currentThread().getName().isEmpty() ? "?" : "thread")));
        Runtime.getRuntime().addShutdownHook(b);
        Thread c = new Thread(() -> System.out.println("hook C must not run"));
        Runtime.getRuntime().addShutdownHook(c);
        System.out.println("removed C: " + Runtime.getRuntime().removeShutdownHook(c));
        try {
            Runtime.getRuntime().addShutdownHook(b);
        } catch (IllegalArgumentException e) {
            System.out.println("duplicate: " + e.getMessage());
        }
        System.out.println("main returns");
    }
}
