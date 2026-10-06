// 边界用例：只建线程、不经系统类加载器链的程序读写 Thread.contextClassLoader。初始线程的上下文
// 加载器由 initPhase3 设置——构建期引导映像求值后随映像物化，启动序列把映像中的 main 线程对象绑定为
// 初始线程；新线程在构造时继承它（本程序字节码不直接调用 setContextClassLoader）。
public class TestThreadContextLoaderInit {
    public static void main(String[] args) throws Exception {
        StringBuilder sb = new StringBuilder();
        Thread t = new Thread(() -> sb.append("ran"));
        t.start();
        t.join();
        System.out.println(sb);
        ClassLoader main = Thread.currentThread().getContextClassLoader();
        ClassLoader child = t.getContextClassLoader();
        System.out.println(main != null);
        System.out.println(main == child);
        System.out.println(main.getClass().getName());
    }
}
