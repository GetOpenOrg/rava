// L1（名字级）类：只作 instanceof / 字段类型出现、从未实例化的类发不透明形态；已实例化类的超类型至少 L2。
// - L1 属主上的虚调用：接收者恒 null（null_recv，抛 NPE）
// - 静态类型为 L1 类、成员在其 L2 祖先上（字段声明类 / 调用属主）：先上转再访问，null 照常抛 NPE
// - 折叠为 null 常量的接收者（只剩 Object 类型）：取字段声明类的 null 再访问
public class OpaqueLevels {
    interface Shape { String name(); }
    interface Marker {}
    static class Base { int tag = 7; String id() { return "b" + tag; } }
    static class Circle extends Base implements Shape { public String name() { return "c"; } }
    static abstract class Ghost implements Shape {}
    static abstract class Phantom extends Base {}
    static Ghost ghost;
    static Phantom phantom;
    static Base nobody;

    static Phantom phantom() { return phantom; }

    public static void main(String[] args) {
        Object o = new Circle();
        System.out.println(o instanceof Marker);
        Shape s = (Shape) o;
        System.out.println(s.name() + ((Base) o).id());
        try {
            System.out.println(ghost.name());
        } catch (NullPointerException e) {
            System.out.println("NPE ghost");
        }
        try {
            System.out.println(nobody.tag);
        } catch (NullPointerException e) {
            System.out.println("NPE field");
        }
        try {
            nobody.tag = 3;
        } catch (NullPointerException e) {
            System.out.println("NPE store");
        }
        // 接收者静态类型仍是 L1 类 Phantom（局部变量；checkcast 只要类型身份），经 javac 省略
        // checkcast 的上转访问 L2 的 Base 成员：字段 / 调用属主都是 Base
        Object[] cells = new Object[args.length + 1];
        Phantom p = (Phantom) cells[0];
        try {
            System.out.println(((Base) p).tag);
        } catch (NullPointerException e) {
            System.out.println("NPE view field");
        }
        try {
            ((Base) p).tag = 5;
        } catch (NullPointerException e) {
            System.out.println("NPE view store");
        }
        try {
            System.out.println(((Base) p).id());
        } catch (NullPointerException e) {
            System.out.println("NPE view call");
        }
        try {
            Base b = phantom();
            System.out.println(b.id());
        } catch (NullPointerException e) {
            System.out.println("NPE call");
        }
    }
}
