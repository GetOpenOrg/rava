import java.lang.reflect.Field;
import java.lang.reflect.Method;

/**
 * 反射成员对象的物性：Method/Field 的 equals 与 hashCode（两次获取等值等哈希——
 * OGNL/spring 方法缓存的键语义）、toString 固定形态、任一实例可 invoke。
 */
public class TestMethodObjectIdentity {

    static class Widget {
        public int size;

        public int area(int scale) {
            return size * scale;
        }
    }

    public static void main(String[] args) throws Exception {
        Method m1 = Widget.class.getMethod("area", int.class);
        Method m2 = Widget.class.getMethod("area", int.class);
        System.out.println("method-not-same=" + (m1 != m2));
        System.out.println("method-eq=" + m1.equals(m2));
        System.out.println("method-hash-eq=" + (m1.hashCode() == m2.hashCode()));
        System.out.println("method-to-string=" + m1);

        Field f1 = Widget.class.getField("size");
        Field f2 = Widget.class.getField("size");
        System.out.println("field-eq=" + f1.equals(f2));
        System.out.println("field-hash-eq=" + (f1.hashCode() == f2.hashCode()));
        System.out.println("field-to-string=" + f1);

        // getDeclaredMethod 与 getMethod 取到同一方法的等价性
        Method viaDeclared = Widget.class.getDeclaredMethod("area", int.class);
        System.out.println("declared-eq=" + m1.equals(viaDeclared));

        // 任一 Method 实例都能 invoke
        Widget w = new Widget();
        w.size = 6;
        System.out.println("invoke-alt=" + m2.invoke(w, 7));
    }
}
