import java.lang.reflect.Field;

/**
 * Field.get / Field.set 全类型（按名字段协议 __reflect_field）：实例字段 int/long/boolean/
 * short/byte/char/float/double/String/引用，静态字段读写，编译期常量读取，static final 写
 * → IllegalAccessException，类型不符 → IllegalArgumentException，继承字段经声明类访问。
 */
public class TestFieldReflectAll {
    static class Base { protected String label = "base"; }
    static class Holder extends Base {
        int i = 1; long l = 2L; boolean z = true; short s = 3; byte b = 4; char c = 'c';
        float f = 1.5f; double d = 2.25; String str = "s"; Object ref = null;
        static int counter = 7;
        static String name = "n";
        static final int CONST = 42;
        static final String CONST_S = "k";
    }

    static String show(Field f, Object o) throws Exception {
        return f.getName() + "=" + f.get(o);
    }

    public static void main(String[] args) throws Exception {
        Holder h = new Holder();
        Class<Holder> k = Holder.class;
        StringBuilder sb = new StringBuilder("get");
        for (String n : new String[] {"i", "l", "z", "s", "b", "c", "f", "d", "str", "ref"}) {
            sb.append(' ').append(show(k.getDeclaredField(n), h));
        }
        System.out.println(sb);

        k.getDeclaredField("i").set(h, 10);
        k.getDeclaredField("l").set(h, 20L);
        k.getDeclaredField("z").set(h, false);
        k.getDeclaredField("s").set(h, (short) 30);
        k.getDeclaredField("b").set(h, (byte) 40);
        k.getDeclaredField("c").set(h, 'x');
        k.getDeclaredField("f").set(h, 3.5f);
        k.getDeclaredField("d").set(h, 4.75);
        k.getDeclaredField("str").set(h, "t");
        k.getDeclaredField("ref").set(h, h.str);
        System.out.println("direct " + h.i + " " + h.l + " " + h.z + " " + h.s + " " + h.b + " " + h.c
                + " " + h.f + " " + h.d + " " + h.str + " " + h.ref);

        System.out.println("static " + show(k.getDeclaredField("counter"), null)
                + " " + show(k.getDeclaredField("name"), null)
                + " " + show(k.getDeclaredField("CONST"), null)
                + " " + show(k.getDeclaredField("CONST_S"), null));
        k.getDeclaredField("counter").set(null, 8);
        k.getDeclaredField("name").set(null, "m");
        System.out.println("static set " + Holder.counter + " " + Holder.name);

        try {
            k.getDeclaredField("CONST").set(null, 1);
            System.out.println("no exception?");
        } catch (IllegalAccessException e) {
            System.out.println("IllegalAccessException on static final");
        }
        try {
            k.getDeclaredField("str").set(h, 5);
            System.out.println("no exception?");
        } catch (IllegalArgumentException e) {
            System.out.println("IllegalArgumentException on type mismatch");
        }
        Field label = Base.class.getDeclaredField("label");
        label.set(h, "sub");
        System.out.println("inherited " + label.get(h) + " " + h.label);
    }
}
