import java.lang.reflect.Field;

// 按名查字段的名字来源：局部变量 / 形参 / 静态字段 / 拼接 / getClass 接收者；名字不存在抛 NoSuchFieldException
public class TestReflectFieldNames {
    static String sizeName = "SIZE";

    static Object read(Class<?> c, String name) throws Exception {
        return c.getDeclaredField(name).get(null);
    }

    static String prefix(boolean max) {
        return max ? "MAX" : "MIN";
    }

    public static void main(String[] args) throws Exception {
        // 局部变量
        String local = "MIN_VALUE";
        System.out.println("local=" + Integer.class.getDeclaredField(local).get(null));
        // 形参（两个调用点各带类与名）
        System.out.println("param1=" + read(Long.class, "MIN_VALUE"));
        System.out.println("param2=" + read(Short.class, "MAX_VALUE"));
        // 静态字段
        Field size = Byte.class.getDeclaredField(sizeName);
        System.out.println("field=" + size.getName() + "," + size.get(null));
        // 拼接出的名字
        Field max = Character.class.getDeclaredField(prefix(args.length == 0) + "_VALUE");
        System.out.println("concat=" + max.getName() + "," + (int) (Character) max.get(null));
        Field radix = Character.class.getDeclaredField(prefix(args.length != 0) + "_RADIX");
        System.out.println("concat2=" + radix.getName() + "," + radix.get(null));
        // 公有字段（getField）
        System.out.println("public=" + Double.class.getField("MAX_EXPONENT").get(null));
        // 接收者来自 getClass 的值流
        Object boxed = 7L;
        System.out.println("recv=" + boxed.getClass().getDeclaredField("BYTES").get(null));
        // 名字不存在：真实 NoSuchFieldException、消息=字段名
        try {
            Integer.class.getDeclaredField("NO_SUCH_" + args.length);
            System.out.println("unexpected");
        } catch (NoSuchFieldException ex) {
            System.out.println("missing=" + ex.getClass().getSimpleName() + "," + ex.getMessage());
        }
        try {
            read(Long.class, "nope");
            System.out.println("unexpected");
        } catch (NoSuchFieldException ex) {
            System.out.println("missing2=" + ex.getMessage());
        }
    }
}
