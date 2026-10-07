// 按对象常量（计划 c1d §30.9 的 P2–P4）：两个 Box 的 tag 一个非空、一个为 null。
// - use(b1)：接收者来自形参，b.tag() 按接收者对象取返回值（P3），构造器必然写 tag、不并入初值 null（P2），
//   null 分支死，Rare.go 不入链；
// - Holder.run：接收者来自字段读（站点），按站点值集取返回值（P4），Rare2.go 不入链。
// 按成员汇合时 tag() 的返回含 b2 的 null，两条 null 分支都活。
public class ObjFacts {
    static class Box<T> {
        final T item;
        final String tag;

        Box(T item, String tag) {
            this.item = item;
            this.tag = tag;
        }

        String tag() { return tag; }
    }

    static class Rare {
        static void go() { System.out.println("rare"); }
    }

    static class Rare2 {
        static void go() { System.out.println("rare2"); }
    }

    static class Holder {
        final Box<String> box;

        Holder(Box<String> box) { this.box = box; }

        void run() {
            if (box.tag() == null) Rare2.go();
            System.out.println(box.item);
        }
    }

    static void use(Box<String> b) {
        if (b.tag() == null) Rare.go();
        System.out.println(b.item);
    }

    public static void main(String[] args) {
        Box<String> b1 = new Box<>("x", "a");
        Box<String> b2 = new Box<>("y", null);
        use(b1);
        new Holder(b1).run();
        System.out.println(b2.tag());
    }
}
