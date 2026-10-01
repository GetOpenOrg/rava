// L1（名字级）类：只作 instanceof / checkcast / 字段类型出现、从未实例化的类发不透明形态；
// 已实例化类的超接口至少 L2；L1 属主上的虚调用接收者恒 null（null_recv，抛 NPE）
public class OpaqueLevels {
    interface Shape { String name(); }
    interface Marker {}
    static class Circle implements Shape { public String name() { return "c"; } }
    static abstract class Ghost implements Shape {}
    static Ghost ghost;

    public static void main(String[] args) {
        Object o = new Circle();
        System.out.println(o instanceof Marker);
        Shape s = (Shape) o;
        System.out.println(s.name());
        try {
            System.out.println(ghost.name());
        } catch (NullPointerException e) {
            System.out.println("NPE");
        }
    }
}
