import java.lang.reflect.Field;
import java.util.Arrays;

/**
 * 基本类型类镜像与数组元素类型镜像的边界（C1d-b b1 / ②）：`Class.getPrimitiveClass` 的结果（int.class、
 * Integer.TYPE）不是字节码类——无超类、无字段、无类初始化；`getComponentType` / `componentType` 按数组描述符
 * 去一维给元素类型镜像（基本类型元素给基本类型类镜像，非数组为 null），经元素类型镜像的字段枚举只放开该类的字段。
 */
public class TestPrimitiveClassMirror {
    static class Point {
        int x;
        int y;
        String label;
    }

    static class Other {
        long unused;
    }

    static String describe(Class<?> c) {
        if (c == null) {
            return "null";
        }
        return c.getName() + " primitive=" + c.isPrimitive() + " array=" + c.isArray()
                + " super=" + (c.getSuperclass() == null ? "null" : c.getSuperclass().getName())
                + " fields=" + c.getDeclaredFields().length;
    }

    public static void main(String[] args) {
        System.out.println(describe(int.class));
        System.out.println(describe(Integer.TYPE));
        System.out.println(describe(void.class));
        System.out.println("TYPE same: " + (Integer.TYPE == int.class) + " " + (Character.TYPE == char.class));

        Class<?>[] arrays = { int[].class, double[][].class, Other[].class, Point[].class, Point[][].class, Point.class };
        for (Class<?> a : arrays) {
            Class<?> c = a.getComponentType();
            System.out.println(a.getSimpleName() + " -> " + describe(c) + " / " + (c == a.componentType()));
        }

        // 经元素类型镜像枚举字段：只有 Point 的字段
        Point[] pts = { new Point() };
        Class<?> elem = pts.getClass().getComponentType();
        String[] names = Arrays.stream(elem.getDeclaredFields()).map(Field::getName).sorted().toArray(String[]::new);
        System.out.println("elem fields: " + String.join(",", names));
        System.out.println("elem is Point: " + (elem == Point.class) + ", int elem: " + (new int[0].getClass().getComponentType() == int.class));
        Object o = new Other();
        System.out.println("other: " + o.getClass().getDeclaredFields().length);
    }
}
