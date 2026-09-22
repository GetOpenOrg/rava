import java.lang.reflect.Field;
import java.util.concurrent.atomic.AtomicInteger;

public class TestReflectProbe {
    public static void main(String[] args) throws Exception {
        // getDeclaredField not-found：真实 NoSuchFieldException、可捕获、消息=字段名
        try {
            Field f = String.class.getDeclaredField("no_such_field_xyz");
            System.out.println("unexpected=" + f);
        } catch (NoSuchFieldException ex) {
            System.out.println("caught=" + ex.getClass().getSimpleName() + "," + ex.getMessage());
        }
        // 命中：携带声明元数据（名 / 修饰位 / 类型描述符）
        Field value = String.class.getDeclaredField("value");
        System.out.println("hit=" + value.getName() + ",mods=" + value.getModifiers());
        Field serial = String.class.getDeclaredField("serialVersionUID");
        System.out.println("serialMods=" + serial.getModifiers());
        System.out.println("typeOf=" + value.getType().getName());

        // getComponentType：数组 / 基本数组 / 嵌套数组 / 非数组（经 getName 断言，
        // 规避 Class.toString 的既有缺口）
        System.out.println("arr=" + String[].class.getComponentType().getName());
        System.out.println("arrPrim=" + int[].class.getComponentType().getName());
        System.out.println("arrNested=" + int[][].class.getComponentType().getName());
        System.out.println("nonArr=" + (String.class.getComponentType() == null));
        System.out.println("clsName=" + String[].class.getName());
        System.out.println("same=" + (new String[0].getClass() == String[].class));
        System.out.println("viaGetClass=" + new String[0].getClass().getComponentType().getName());

        // 访问检查：私有字段无 setAccessible → IllegalAccessException（与 JDK 同型）
        AtomicInteger ai = new AtomicInteger(5);
        Field vf = AtomicInteger.class.getDeclaredField("value");
        try {
            System.out.println("getV=" + vf.get(ai));
        } catch (IllegalAccessException ex) {
            System.out.println("iaeV=" + ex.getClass().getSimpleName());
        }
        // 公有静态常量（ConstantValue，I / J 描述符）
        Field maxI = Integer.class.getDeclaredField("MAX_VALUE");
        System.out.println("maxI=" + maxI.get(null) + ",type=" + maxI.getType().getName());
        Field maxL = Long.class.getDeclaredField("MAX_VALUE");
        System.out.println("maxL=" + maxL.get(null));
    }
}
