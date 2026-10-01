// 恒 null 接收者：字段类型的类从未实例化（字段读作 null），调用其方法按 invokevirtual 语义
// 抛 NullPointerException（闭包分析导出 null_recv），不得静默返回默认值。
public class NullView {
    static abstract class Handler {
        abstract String name();
    }

    static class Holder {
        Handler h;
        String go() { return h.name(); }
    }

    public static void main(String[] args) {
        try {
            System.out.println(new Holder().go());
        } catch (NullPointerException e) {
            System.out.println("NPE");
        }
    }
}
