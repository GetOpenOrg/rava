import java.lang.reflect.Method;
import java.lang.reflect.RecordComponent;
import java.util.List;

/**
 * record 分量反射（Class.getRecordComponents → native getRecordComponents0）：分量名 / 类型
 * （基本、引用、数组、泛型擦除、嵌套 record）、声明序、toString、访问器（名字 / 返回类型 /
 * invoke）、getDeclaringRecord；空 record → 长度 0；非 record（普通类、数组、接口）→ null；
 * static 字段不计入分量；紧凑构造器校验后的分量值经访问器读出。
 */
public class TestRecordComponents {
    record Point(int x, int y) {}
    record Mixed(long id, double w, boolean flag, char c, String name, int[] data, List<String> tags, Point origin) {
        static int COUNT = 7;
        Mixed {
            if (name == null) name = "anon";
        }
    }
    record Empty() {}
    interface Shape { double area(); }
    record Circle(double r) implements Shape {
        public double area() { return 3.0 * r * r; }
    }
    static class Plain { int x; }

    static void dump(Class<?> c, Object instance) throws Exception {
        System.out.println(c.getSimpleName() + " isRecord=" + c.isRecord());
        RecordComponent[] rcs = c.getRecordComponents();
        if (rcs == null) {
            System.out.println("  components=null");
            return;
        }
        System.out.println("  components=" + rcs.length);
        for (RecordComponent rc : rcs) {
            Method acc = rc.getAccessor();
            Object v = instance == null ? null : acc.invoke(instance);
            String shown = v instanceof int[] arr ? "int[" + arr.length + "]" : String.valueOf(v);
            System.out.println("  " + rc + " | name=" + rc.getName() + " type=" + rc.getType().getName()
                    + " accessor=" + acc.getName() + ":" + acc.getReturnType().getName()
                    + " declaring=" + rc.getDeclaringRecord().getSimpleName() + " value=" + shown);
        }
    }

    public static void main(String[] args) throws Exception {
        dump(Point.class, new Point(3, 4));
        dump(Mixed.class, new Mixed(9L, 1.5, true, 'z', null, new int[] {1, 2}, List.of("a", "b"), new Point(0, 1)));
        dump(Empty.class, new Empty());
        dump(Circle.class, new Circle(2.0));
        dump(Plain.class, null);
        dump(String.class, null);
        dump(int[].class, null);
        dump(Shape.class, null);
        System.out.println(Mixed.COUNT);
    }
}
