/**
 * 异常链与抑制栈：initCause 一次性/自环、addSuppressed 顺序、getStackTrace 非空、
 * fillInStackTrace、cause 穿透（spring/mybatis 异常翻译与 try-with-resources 的地基）。
 */
public class TestThrowableChains {

    public static void main(String[] args) {
        IllegalStateException base = new IllegalStateException("base");
        System.out.println("cause-null=" + (base.getCause() == null));

        // initCause 一次性
        base.initCause(new IllegalArgumentException("root"));
        System.out.println("cause=" + base.getCause().getClass().getSimpleName()
                + " " + base.getCause().getMessage());
        try {
            base.initCause(new RuntimeException("second"));
        } catch (IllegalStateException e) {
            System.out.println("init-twice-ex=" + e.getClass().getSimpleName());
        }

        // 自环拒绝
        RuntimeException r = new RuntimeException("r");
        try {
            r.initCause(r);
        } catch (IllegalArgumentException e) {
            System.out.println("self-cause-ex=" + e.getClass().getSimpleName());
        }

        // 构造即带 cause 的形态
        Exception wrapped = new Exception("wrapped", base);
        System.out.println("chain-depth2=" + wrapped.getCause().getCause().getMessage());

        // 抑制异常顺序
        Exception sup = new Exception("sup");
        sup.addSuppressed(new ArithmeticException("s1"));
        sup.addSuppressed(new NullPointerException("s2"));
        System.out.println("suppressed=" + sup.getSuppressed().length
                + " first=" + sup.getSuppressed()[0].getMessage());

        // try-with-resources 的抑制语义（资源关闭异常被抑制）
        try {
            try (BadResource br = new BadResource()) {
                throw new IllegalStateException("body");
            }
        } catch (IllegalStateException e) {
            System.out.println("twr-cause=" + e.getMessage()
                    + " twr-suppressed=" + e.getSuppressed()[0].getMessage());
        }

        // 栈轨迹
        StackTraceElement[] st = base.getStackTrace();
        System.out.println("stack-nonempty=" + (st.length > 0)
                + " top=" + st[0].getClassName());

        Exception filled = new Exception("f");
        filled.fillInStackTrace();
        System.out.println("refill-ok=" + (filled.getStackTrace().length > 0));

        // toString 形态（类名: 消息 / 嵌套 cause）
        System.out.println("to-string=" + wrapped.toString());
    }

    static class BadResource implements AutoCloseable {
        @Override
        public void close() {
            throw new IllegalStateException("close-failed");
        }
    }
}
