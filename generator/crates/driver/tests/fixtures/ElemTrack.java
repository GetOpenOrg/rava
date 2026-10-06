// 容器元素按对象跟踪（计划 c1d §30 B2）：两个 ConcurrentHashMap 各存一种元素，从 m1 取出再 cast 的值只含 m1 的元素。
// m1.get 的结果若退化为全部 map 的值并集，Square.name 会经 Shape.name 派发入链。
import java.util.concurrent.ConcurrentHashMap;

public class ElemTrack {
    interface Shape {
        String name();
    }

    static class Circle implements Shape {
        public String name() { return "circle"; }
    }

    static class Square implements Shape {
        public String name() { return "square"; }
    }

    public static void main(String[] args) {
        ConcurrentHashMap<String, Object> m1 = new ConcurrentHashMap<>();
        ConcurrentHashMap<String, Object> m2 = new ConcurrentHashMap<>();
        m1.put("a", new Circle());
        m2.put("b", new Square());
        Object o = m1.get(args.length > 0 ? args[0] : "a");
        System.out.println(((Shape) o).name());
        System.out.println(m2.size());
    }
}
