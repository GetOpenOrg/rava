import java.util.ArrayList;
import java.util.List;

// 闭包精度边界：instanceof 的否定分支只保留不是该类型子类型的接收者（a5-4b）。
// 覆盖：类 / 接口过滤、子类在否定侧被排除、经父类实现接口、null 落入否定侧、否定后再分派、取反条件
public class TestInstanceofElseDispatch {
    interface Shape { String name(); }
    interface Marked { }
    static class Circle implements Shape { public String name() { return "circle"; } }
    static class BigCircle extends Circle { public String name() { return "big-circle"; } }
    static class Square implements Shape, Marked { public String name() { return "square"; } }
    static class TinySquare extends Square { public String name() { return "tiny-square"; } }
    static class Tri implements Shape { public String name() { return "tri"; } public String toString() { return "Tri"; } }

    static String classify(Shape s) {
        if (s instanceof Circle) {
            return "round:" + s.name();
        } else {
            return "other:" + s.name();
        }
    }

    static String marked(Object o) {
        if (!(o instanceof Marked)) {
            return "unmarked:" + o.getClass().getSimpleName();
        }
        return "marked:" + ((Shape) o).name();
    }

    static String nullSide(Shape s) {
        if (s instanceof Circle c) {
            return "circle " + c.name();
        }
        try {
            return "else " + s.name();
        } catch (NullPointerException e) {
            return "else NPE";
        }
    }

    static String chain(Object o) {
        if (o instanceof Circle) return "C";
        if (o instanceof Square) return "S";
        if (o instanceof CharSequence cs) return "CS" + cs.length();
        return "X:" + o;
    }

    public static void main(String[] args) {
        List<Shape> shapes = new ArrayList<>();
        shapes.add(new Circle());
        shapes.add(new BigCircle());
        shapes.add(new Square());
        shapes.add(new TinySquare());
        shapes.add(new Tri());
        for (Shape s : shapes) {
            System.out.println(classify(s) + " | " + marked(s) + " | " + nullSide(s) + " | " + chain(s));
        }
        System.out.println(nullSide(null));
        System.out.println(marked("str") + " | " + marked(42));
        System.out.println(chain(new StringBuilder("abc")) + " | " + chain("hello") + " | " + chain(7L));
    }
}
