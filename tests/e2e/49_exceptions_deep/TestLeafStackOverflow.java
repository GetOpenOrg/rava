// 叶子方法省略入口栈界检查（a3-T1b-2）的边界：无界递归经过叶子方法、
// 或经「叶子声明的虚方法」分派到递归覆盖体时，仍须抛出可捕获的 StackOverflowError。
public class TestLeafStackOverflow {
    static int depth;

    // static 叶子：无调用指令
    static int inc(int x) { return x + 1; }

    static void viaStaticLeaf() {
        depth = inc(depth);
        viaStaticLeaf();
    }

    // private 叶子
    private int count;
    private void bump() { count++; }

    void viaPrivateLeaf() {
        bump();
        viaPrivateLeaf();
    }

    // final 类的叶子 getter
    static final class Box {
        int v;
        int get() { return v; }
        void set(int x) { v = x; }
    }

    static void viaFinalClassLeaf(Box b) {
        b.set(b.get() + 1);
        viaFinalClassLeaf(b);
    }

    // 基类方法体是叶子，子类覆盖体递归：递归环只经过基类声明的虚方法入口
    static class Node {
        int next() { return 0; }
    }

    static class Deep extends Node {
        Node self = this;
        int n;
        @Override
        int next() {
            n++;
            return 1 + self.next();
        }
    }

    // 接口 default 方法是叶子，实现类覆盖体经接口引用递归
    interface Step {
        default int step() { return 0; }
    }

    static class Walker implements Step {
        Step self = this;
        int n;
        @Override
        public int step() {
            n++;
            return 1 + self.step();
        }
    }

    static String run(String name, Runnable r) {
        try {
            r.run();
            return name + ": returned";
        } catch (StackOverflowError e) {
            return name + ": StackOverflowError";
        }
    }

    public static void main(String[] args) {
        for (int round = 0; round < 2; round++) {
            depth = 0;
            System.out.println(run("static-leaf", TestLeafStackOverflow::viaStaticLeaf) + " deep=" + (depth > 500));

            TestLeafStackOverflow t = new TestLeafStackOverflow();
            System.out.println(run("private-leaf", t::viaPrivateLeaf) + " deep=" + (t.count > 500));

            Box b = new Box();
            System.out.println(run("final-class-leaf", () -> viaFinalClassLeaf(b)) + " deep=" + (b.v > 500));

            Deep d = new Deep();
            System.out.println(run("virtual-override", d::next) + " deep=" + (d.n > 500));

            Walker w = new Walker();
            System.out.println(run("interface-default", w::step) + " deep=" + (w.n > 500));
        }
        // 溢出恢复后正常调用叶子方法
        System.out.println("after: " + inc(41) + " " + new Node().next());
    }
}
