import java.lang.reflect.Field;

/**
 * Class.forName 的类名经字符串拼接得到（indy makeConcatWithConstants / StringBuilder.append 链）：
 * 各段是字面量或类镜像的 getName / getSimpleName 结果时，所指类可在编译期确定。
 * 覆盖 initialize=true / false、嵌套类（`$` 拼接）、数组类描述符（`[L…;`）、跨方法传递的拼接名，
 * 以及经数组组件镜像反射读 static 字段触发元素类初始化。字面量版本见 TestDeclaringClassInit。
 */
public class TestForNameComputedName {

    static class Plain {
        static String tag = init("Plain");
    }

    static class Elem {
        static String tag = init("Elem");
    }

    static class Eager {
        static String tag = init("Eager");
    }

    static class Lazy {
        static String tag = init("Lazy");

        static class Leaf {
            static String tag = init("Lazy.Leaf");
        }
    }

    static class Built {
        static String tag = init("Built");
    }

    static class Passed {
        static String tag = init("Passed");
    }

    static String init(String who) {
        System.out.println("clinit " + who);
        return who + "-ready";
    }

    static Object readTag(Class<?> c) throws Exception {
        Field f = c.getDeclaredField("tag");
        return f.get(null);
    }

    static ClassLoader loader() {
        return TestForNameComputedName.class.getClassLoader();
    }

    /** 拼接名经形参跨方法传递 */
    static Class<?> load(String name, boolean initialize) throws Exception {
        return Class.forName(name, initialize, loader());
    }

    /** 拼接名经返回值跨方法传递 */
    static String nestedName(Class<?> outer, String simple) {
        return outer.getName() + "$" + simple;
    }

    public static void main(String[] args) throws Exception {
        String host = TestForNameComputedName.class.getName();

        // initialize=false：只加载，经反射读 static 字段才初始化
        Class<?> plain = Class.forName(host + "$Plain", false, loader());
        System.out.println("loaded " + plain.getSimpleName() + " (no clinit yet)");
        System.out.println("tag " + readTag(plain));

        // 数组类描述符：数组类不初始化；组件镜像反射读字段初始化元素类
        Class<?> arr = Class.forName("[L" + Elem.class.getName() + ";");
        System.out.println("array " + arr.getSimpleName() + " component " + arr.getComponentType().getSimpleName()
                + " (no clinit yet)");
        System.out.println("tag " + readTag(arr.getComponentType()));
        Class<?> arr2 = Class.forName("[[L" + host + "$Elem;");
        System.out.println("array2 " + arr2.getSimpleName() + " same-elem "
                + (arr2.getComponentType().getComponentType() == Elem.class));

        // initialize=true：forName 当场初始化
        Class<?> eager = Class.forName(TestForNameComputedName.class.getName() + "$" + "Eager");
        System.out.println("eager " + eager.getSimpleName());

        // 跨方法传递的拼接名；多层嵌套
        Class<?> lazy = load(nestedName(TestForNameComputedName.class, "Lazy"), false);
        System.out.println("lazy " + lazy.getSimpleName() + " (no clinit yet)");
        Class<?> leaf = load(nestedName(lazy, Leaf.class.getSimpleName()), true);
        System.out.println("leaf " + leaf.getName());
        System.out.println("tag " + readTag(lazy));

        // StringBuilder 链
        StringBuilder sb = new StringBuilder();
        sb.append(host).append('$').append("Built");
        Class<?> built = Class.forName(sb.toString());
        System.out.println("built " + built.getSimpleName());

        // 拼接名经形参传入、在被调方内初始化
        Class<?> passed = load(host + "$" + "Passed", true);
        System.out.println("passed " + passed.getSimpleName());
    }

    static class Leaf {
    }
}
