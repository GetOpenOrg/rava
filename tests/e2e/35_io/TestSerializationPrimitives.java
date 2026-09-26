import java.io.ByteArrayInputStream;
import java.io.ByteArrayOutputStream;
import java.io.ObjectInputStream;
import java.io.ObjectOutputStream;
import java.io.Serializable;

/**
 * 反序列化实例化语义（N2，ReflectionFactory.newConstructorForSerialization）与全基本类型字段
 * 往返：首个不可序列化超类的无参构造器运行（Base() 打印、baseVal 回到构造值）；可序列化类
 * 自身的构造器与字段初始化器不运行（transient 字段为默认值）；boolean / byte / short / char /
 * int / long / float / double / String / int[] 字段按值还原；嵌套可序列化对象与共享引用。
 */
public class TestSerializationPrimitives {
    static class Base {
        int baseVal = 5;
        Base() { System.out.println("Base() ran"); }
    }

    static class Inner implements Serializable {
        private static final long serialVersionUID = 3L;
        String tag;
        Inner(String tag) { this.tag = tag; }
    }

    static class All extends Base implements Serializable {
        private static final long serialVersionUID = 1L;
        boolean z; byte b; short s; char c; int i; long l; float f; double d;
        String str; int[] arr; Inner in1; Inner in2;
        transient int t = 9;
        All() { System.out.println("All() ran"); }
    }

    public static void main(String[] args) throws Exception {
        All a = new All();
        a.baseVal = 77;
        a.z = true; a.b = -8; a.s = 1234; a.c = 'Q'; a.i = -99999; a.l = 1L << 40;
        a.f = 2.5f; a.d = -0.125; a.str = "text"; a.arr = new int[] {3, 1, 4};
        a.in1 = new Inner("shared"); a.in2 = a.in1;

        ByteArrayOutputStream bo = new ByteArrayOutputStream();
        try (ObjectOutputStream oos = new ObjectOutputStream(bo)) {
            oos.writeObject(a);
        }
        System.out.println("written " + (bo.size() > 0));

        All r;
        try (ObjectInputStream ois = new ObjectInputStream(new ByteArrayInputStream(bo.toByteArray()))) {
            r = (All) ois.readObject();
        }
        System.out.println("baseVal " + r.baseVal + " t " + r.t);
        System.out.println("prims " + r.z + " " + r.b + " " + r.s + " " + r.c + " " + r.i + " " + r.l
                + " " + r.f + " " + r.d);
        System.out.println("refs " + r.str + " " + r.arr.length + ":" + r.arr[0] + r.arr[1] + r.arr[2]
                + " " + r.in1.tag + " shared " + (r.in1 == r.in2) + " copy " + (r != a));
    }
}
