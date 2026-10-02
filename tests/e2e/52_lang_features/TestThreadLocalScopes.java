/**
 * ThreadLocal 语义：set/get/remove/withInitial、同线程多实例隔离、
 * InheritableThreadLocal 父子传递（slf4j MDC 与框架上下文传递的地基，
 * 全库此前零覆盖；子线程固定执行序列并 join，输出确定）。
 */
public class TestThreadLocalScopes {

    static final ThreadLocal<String> CTX = ThreadLocal.withInitial(() -> "init-value");
    static final InheritableThreadLocal<String> PARENT_TO_CHILD = new InheritableThreadLocal<>();

    public static void main(String[] args) throws Exception {
        // withInitial 初始值 / set 覆盖 / remove 回落
        System.out.println("initial=" + CTX.get());
        CTX.set("ctx-a");
        System.out.println("after-set=" + CTX.get());
        CTX.remove();
        System.out.println("after-remove=" + CTX.get());

        // 未给初始值的构造形态：null
        ThreadLocal<Object> plain = new ThreadLocal<>();
        System.out.println("plain-null=" + (plain.get() == null));
        plain.set("x");
        System.out.println("plain-set=" + plain.get());

        // 同线程多 ThreadLocal 实例互不干扰
        ThreadLocal<Integer> a = ThreadLocal.withInitial(() -> 1);
        ThreadLocal<Integer> b = ThreadLocal.withInitial(() -> 2);
        a.set(10);
        System.out.println("iso=" + a.get() + "," + b.get());

        // 子线程不继承普通 ThreadLocal
        ThreadLocal<String> noInherit = new ThreadLocal<>();
        noInherit.set("parent-only");
        final StringBuilder out = new StringBuilder();
        Thread t1 = new Thread(() -> {
            out.append("child-plain=").append(noInherit.get()).append("\n");
        });
        t1.start();
        t1.join();
        System.out.print(out);

        // InheritableThreadLocal：子线程在构造时快照父值
        PARENT_TO_CHILD.set("from-parent");
        StringBuilder out2 = new StringBuilder();
        Thread t2 = new Thread(() -> {
            out2.append("child-inherit=").append(PARENT_TO_CHILD.get()).append("\n");
            PARENT_TO_CHILD.set("child-local");
            out2.append("child-after-set=").append(PARENT_TO_CHILD.get()).append("\n");
        });
        t2.start();
        t2.join();
        System.out.print(out2);
        System.out.println("parent-untouched=" + PARENT_TO_CHILD.get());
    }
}
