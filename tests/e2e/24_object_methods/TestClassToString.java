import java.util.ArrayList;
import java.util.List;
import java.util.Objects;

/**
 * Class 对象经 Object 视图的虚分派（Rosetta SumDataType 揭出：Class 由手写层构造、未记为
 * 已实例化，`"" + obj.getClass()` 落 vtable 默认体输出 java/lang/Class）。覆盖：普通类 /
 * 装箱类 / 数组 / 嵌套类 / 接口 / 基本类型 / 枚举 / 记录的 Class.toString，
 * 入口含字符串拼接、String.valueOf、println(Object)、Objects.toString、集合 toString、
 * 以及 getName / getSimpleName / equals / hashCode 一致性。
 */
public class TestClassToString {
    interface Shape {}
    enum Color { RED }
    record Pt(int x) {}
    static class Inner implements Shape {}

    public static void main(String[] args) {
        Object[] samples = {"s", 1, 2.5f, new int[0], new String[0][0], new Inner(), Color.RED, new Pt(1), new ArrayList<>()};
        for (Object o : samples) {
            Class<?> c = o.getClass();
            System.out.println("concat=" + c);
            System.out.println("valueOf=" + String.valueOf(c));
            Object asObj = c;
            System.out.println(asObj);
            System.out.println("objects=" + Objects.toString(c) + " name=" + c.getName());
        }
        System.out.println(Shape.class + " | " + int.class + " | " + void.class + " | " + Color.class);
        List<Class<?>> list = List.of(String.class, Integer.class, Shape.class);
        System.out.println(list);
        System.out.println(String.class.equals("x".getClass()) + " " + (String.class.hashCode() == "y".getClass().hashCode()));
    }
}
