import java.lang.reflect.Field;

/**
 * getDeclaringClass 按接收者求外层类镜像（vm_intrinsics reflect.declaring_of_receiver）：
 * 成员类 / 多层嵌套 → 外层类；匿名 / 局部 / 数组 / 基本类型 → null。
 * 只取镜像不触发外层类初始化；经返回镜像反射读 static 字段才触发——外层类的 <clinit>
 * 只能经 getDeclaringClass 的返回值到达，闭包须按接收者建模而非退回「任意类」。
 * 数组镜像 getComponentType 同理：元素类只经组件镜像反射初始化（手写 componentType 写入值来源）。
 */
public class TestDeclaringClassInit {

    static class Outer {
        static String tag = init("Outer");

        static class Inner {
            static class Deepest {
            }
        }
    }

    static class Elem {
        static String tag = init("Elem");
    }

    static class Plain {
        static String tag = init("Plain");
    }

    static String init(String who) {
        System.out.println("clinit " + who);
        return who + "-ready";
    }

    static String name(Class<?> c) {
        return c == null ? "null" : c.getSimpleName();
    }

    static Object readTag(Class<?> c) throws Exception {
        Field f = c.getDeclaredField("tag");
        return f.get(null);
    }

    public static void main(String[] args) throws Exception {
        Class<?> deepest = Outer.Inner.Deepest.class;
        Class<?> inner = deepest.getDeclaringClass();
        Class<?> outer = inner.getDeclaringClass();
        System.out.println("deepest->" + name(inner) + " inner->" + name(outer));
        System.out.println("outer->" + name(outer.getDeclaringClass()));
        System.out.println("after mirrors (no clinit yet)");
        System.out.println("tag " + readTag(outer));

        Object anon = new Object() {
        };
        class Local {
        }
        System.out.println("anon->" + name(anon.getClass().getDeclaringClass())
                + " enclosing=" + name(anon.getClass().getEnclosingClass()));
        System.out.println("local->" + name(Local.class.getDeclaringClass())
                + " enclosing=" + name(Local.class.getEnclosingClass()));
        System.out.println("array->" + name(Elem[].class.getDeclaringClass())
                + " prim->" + name(int.class.getDeclaringClass()));

        Class<?> arr = Class.forName("[LTestDeclaringClassInit$Elem;");
        Class<?> comp = arr.getComponentType();
        System.out.println("component " + name(comp) + " (no clinit yet)");
        System.out.println("tag " + readTag(comp));

        Class<?> plain = Class.forName("TestDeclaringClassInit$Plain", false,
                TestDeclaringClassInit.class.getClassLoader());
        System.out.println("plain declaring " + name(plain.getDeclaringClass()));
        System.out.println("tag " + readTag(plain));
    }
}
