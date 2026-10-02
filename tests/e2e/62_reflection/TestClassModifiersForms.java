import java.lang.annotation.ElementType;
import java.lang.annotation.Retention;
import java.lang.annotation.RetentionPolicy;
import java.lang.annotation.Target;
import java.lang.reflect.Modifier;

/**
 * 类元数据修饰符位与三种类名形态：interface/annotation/enum/record/普通类、
 * abstract/final/static 组合、数组与内部类的 getName/getSimpleName/getCanonicalName
 * （mybatis 按类名路由 TypeHandler、m3 曾发现 getModifiers 缺 INTERFACE 位）。
 */
public class TestClassModifiersForms {

    interface Iface {
    }

    @Retention(RetentionPolicy.RUNTIME)
    @Target(ElementType.TYPE)
    @interface Marker {
    }

    enum Color {
        RED
    }

    record Point(int x, int y) {
    }

    static abstract class AbstractBase {
    }

    static final class FinalLeaf extends AbstractBase {
    }

    static class Inner {
    }

    public static void main(String[] args) {
        print("iface", Iface.class);
        print("anno", Marker.class);
        print("enum", Color.class);
        print("record", Point.class);
        print("abstract", AbstractBase.class);
        print("final", FinalLeaf.class);
        print("inner", Inner.class);

        // 数组类名三形态
        Class<?> arr = String[].class;
        System.out.println("arr-name=" + arr.getName());
        System.out.println("arr-simple=" + arr.getSimpleName());
        System.out.println("arr-canonical=" + arr.getCanonicalName());

        int[][] deep = new int[0][];
        System.out.println("deep-name=" + deep.getClass().getName());

        // 基本类型与 void 镜像名
        System.out.println("int=" + int.class.getName() + " void=" + void.class.getName());

        // 匿名类 canonical null 的经典边界
        Object anon = new Object() {
        };
        System.out.println("anon-canonical-null=" + (anon.getClass().getCanonicalName() == null));
        System.out.println("anon-simple=" + anon.getClass().getSimpleName());
    }

    static void print(String tag, Class<?> c) {
        int mod = c.getModifiers();
        System.out.println(tag + ": interface=" + Modifier.isInterface(mod)
                + " abstract=" + Modifier.isAbstract(mod)
                + " final=" + Modifier.isFinal(mod)
                + " static=" + Modifier.isStatic(mod)
                + " annotation=" + c.isAnnotation()
                + " enum=" + c.isEnum()
                + " record=" + c.isRecord()
                + " name=" + c.getName()
                + " canonical=" + c.getCanonicalName());
    }
}
