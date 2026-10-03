// 引导期主线程上下文加载器：用户代码从不调用 setContextClassLoader，
// 只读取引导（initPhase3）设置的值与子线程继承的值
public class TestBootContextLoader {
    public static void main(String[] args) throws Exception {
        ClassLoader scl = ClassLoader.getSystemClassLoader();
        ClassLoader ctx = Thread.currentThread().getContextClassLoader();
        System.out.println("main ctx non-null: " + (ctx != null));
        System.out.println("main ctx == scl: " + (ctx == scl));
        ClassLoader[] seen = new ClassLoader[1];
        Thread t = new Thread(() -> seen[0] = Thread.currentThread().getContextClassLoader());
        t.start();
        t.join();
        System.out.println("child inherits: " + (seen[0] == ctx));
        System.out.println("own loader == scl: " + (TestBootContextLoader.class.getClassLoader() == scl));
    }
}
