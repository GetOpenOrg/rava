// 边界用例：只建线程、不经系统类加载器链的程序，首次读写 Thread.contextClassLoader 时
// 由 initPhase3 段（ClassLoader.__vm_init_phase3）经手写体设置初始线程的上下文加载器——
// 手写体里经模块路径调用的自由 fn（super::thread_impl::__vm_initial_thread()）的返回类型
// 须推得出，其上的 setContextClassLoader 回调才入闭包（本程序字节码不直接调用它）。
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
