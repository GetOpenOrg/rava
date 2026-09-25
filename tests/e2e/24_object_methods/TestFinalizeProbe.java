/**
 * equiv 探针②：finalize。GC 触发的 finalize 调用时机不确定（JVM 也不保证），
 * 这里只测确定性部分：finalize 作为普通 protected 方法的覆盖、显式调用、
 * super.finalize 链、异常传播，以及 Object.finalize 的默认空实现。
 */
public class TestFinalizeProbe {
    static final StringBuilder LOG = new StringBuilder();

    static class Resource {
        final String name;
        Resource(String name) { this.name = name; }
        @SuppressWarnings({"deprecation", "removal"})
        @Override protected void finalize() throws Throwable {
            LOG.append("R(").append(name).append(")");
            super.finalize();                         // Object.finalize：空实现
        }
        void close() throws Throwable { finalize(); }
    }

    static class Pooled extends Resource {
        Pooled(String name) { super(name); }
        @SuppressWarnings({"deprecation", "removal"})
        @Override protected void finalize() throws Throwable {
            LOG.append("P(").append(name).append(")");
            super.finalize();                         // 链到 Resource.finalize
        }
    }

    static class Failing {
        @SuppressWarnings({"deprecation", "removal"})
        @Override protected void finalize() throws Throwable {
            throw new IllegalStateException("finalize failed");
        }
        void run() throws Throwable { finalize(); }
    }

    static class NoOverride {
        @SuppressWarnings({"deprecation", "removal"})
        void run() throws Throwable { finalize(); }  // 直接走 Object.finalize
    }

    public static void main(String[] args) throws Throwable {
        new Resource("a").close();
        System.out.println("single=" + LOG);
        LOG.setLength(0);
        new Pooled("b").close();                      // 虚分派到 Pooled，再链回 Resource
        System.out.println("chain=" + LOG);
        LOG.setLength(0);
        Resource r = new Pooled("c");
        r.close();
        System.out.println("virtual=" + LOG);
        try {
            new Failing().run();
            System.out.println("failing=unexpected");
        } catch (IllegalStateException e) {
            System.out.println("failing=" + e.getMessage());
        }
        new NoOverride().run();
        System.out.println("objectDefault=ok");
    }
}
