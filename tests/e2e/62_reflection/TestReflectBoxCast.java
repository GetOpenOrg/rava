import java.lang.reflect.Constructor;
import java.lang.reflect.Field;
import java.lang.reflect.Method;

/**
 * 反射装箱值的强转见证（Method.invoke 返回值 / Field.get / Constructor.newInstance）。
 *
 * 基本类型返回值与字段值经反射访问器装箱为包装类实例：可强转为包装类本身及其超类型
 * （Number / Comparable / Object），instanceof 成立，缓存范围内与 valueOf 同一身份。
 * 引用返回值与构造结果强转为声明类型、超类与接口。
 */
public class TestReflectBoxCast {
    interface Shape {
        String name();
    }

    static class Base {
        int base() { return 1; }
    }

    static class Box extends Base implements Shape {
        int iv = 42;
        long lv = 1234567890123L;
        boolean zv = true;
        char cv = 'q';
        byte bv = -5;
        short sv = 300;
        float fv = 1.5f;
        double dv = 2.25;
        String text = "field-text";
        static int counter = 99;
        static long stamp = -7L;

        Box() {}

        Box(int iv) { this.iv = iv; }

        public String name() { return "box" + iv; }

        int getInt(int x) { return x + iv; }
        long getLong(long x) { return x * 3; }
        boolean isOdd(int x) { return (x & 1) == 1; }
        char letter() { return cv; }
        byte small() { return bv; }
        short medium() { return sv; }
        float half(float x) { return x / 2; }
        double twice(double x) { return x * 2; }
        String label(String p) { return p + ":" + iv; }
        Shape self() { return this; }
        static int sq(int x) { return x * x; }
    }

    public static void main(String[] args) throws Exception {
        Box b = new Box();
        Class<Box> c = Box.class;

        // ── Method.invoke：基本类型返回值 ──
        Method mi = c.getDeclaredMethod("getInt", int.class);
        Integer i1 = (Integer) mi.invoke(b, 8);
        int i2 = (Integer) mi.invoke(b, 9);
        Number n1 = (Number) mi.invoke(b, 10);
        @SuppressWarnings("unchecked")
        Comparable<Integer> cmp = (Comparable<Integer>) mi.invoke(b, 11);
        Object o1 = (Object) mi.invoke(b, 12);
        System.out.println("int " + i1 + " " + i2 + " " + n1.longValue() + " " + cmp.compareTo(53) + " " + o1);
        System.out.println("int-inst " + (o1 instanceof Integer) + " " + (o1 instanceof Number)
                + " " + o1.getClass().getName() + " " + o1.equals(54));
        Integer cached = (Integer) mi.invoke(b, -40);
        System.out.println("int-cache " + (cached == Integer.valueOf(2)) + " " + cached.intValue());

        Method ml = c.getDeclaredMethod("getLong", long.class);
        Long l1 = (Long) ml.invoke(b, 5L);
        long l2 = (Long) ml.invoke(b, 7);
        Number n2 = (Number) ml.invoke(b, 9L);
        System.out.println("long " + l1 + " " + l2 + " " + n2.intValue() + " " + l1.getClass().getName());

        Method mz = c.getDeclaredMethod("isOdd", int.class);
        Boolean z1 = (Boolean) mz.invoke(b, 3);
        boolean z2 = (Boolean) mz.invoke(b, 4);
        System.out.println("bool " + z1 + " " + z2 + " " + (z1 == Boolean.TRUE) + " " + z1.getClass().getName());

        Character ch = (Character) c.getDeclaredMethod("letter").invoke(b);
        Byte by = (Byte) c.getDeclaredMethod("small").invoke(b);
        Short sh = (Short) c.getDeclaredMethod("medium").invoke(b);
        Float fl = (Float) c.getDeclaredMethod("half", float.class).invoke(b, 3.0f);
        Double db = (Double) c.getDeclaredMethod("twice", double.class).invoke(b, 1.25);
        System.out.println("narrow " + ch + " " + by + " " + sh + " " + fl + " " + db
                + " " + ((Number) (Object) db).floatValue() + " " + Character.isLetter(ch));

        Integer st = (Integer) c.getDeclaredMethod("sq", int.class).invoke(null, 12);
        System.out.println("static " + st + " " + (st + 1));

        // ── Method.invoke：引用返回值 ──
        String s1 = (String) c.getDeclaredMethod("label", String.class).invoke(b, "p");
        Object s2 = (Object) c.getDeclaredMethod("label", String.class).invoke(b, "q");
        Shape sh1 = (Shape) c.getDeclaredMethod("self").invoke(b);
        Box bx = (Box) c.getDeclaredMethod("self").invoke(b);
        Base bs = (Base) c.getDeclaredMethod("self").invoke(b);
        System.out.println("ref " + s1 + " " + s2 + " " + sh1.name() + " " + (bx == b) + " " + bs.base());
        String viaIface = (String) Shape.class.getMethod("name").invoke(b);
        System.out.println("iface " + viaIface + " " + viaIface.length());

        // ── Field.get：实例 / 静态字段装箱 ──
        Field fi = c.getDeclaredField("iv");
        Field fl2 = c.getDeclaredField("lv");
        Field fz = c.getDeclaredField("zv");
        Field fc = c.getDeclaredField("cv");
        Field fd = c.getDeclaredField("dv");
        Field ft = c.getDeclaredField("text");
        Integer gi = (Integer) fi.get(b);
        Number gn = (Number) fi.get(b);
        Object go = (Object) fi.get(b);
        Long gl = (Long) fl2.get(b);
        Boolean gz = (Boolean) fz.get(b);
        Character gc = (Character) fc.get(b);
        Double gd = (Double) fd.get(b);
        String gt = (String) ft.get(b);
        System.out.println("field " + gi + " " + gn.doubleValue() + " " + go + " " + gl + " " + gz
                + " " + gc + " " + gd + " " + gt);
        System.out.println("field-inst " + (go instanceof Integer) + " " + go.getClass().getName()
                + " " + (gi == Integer.valueOf(42)) + " " + fi.getInt(b));
        Integer sc = (Integer) c.getDeclaredField("counter").get(null);
        Long ss = (Long) c.getDeclaredField("stamp").get(null);
        System.out.println("static-field " + sc + " " + ss + " " + (sc + ss));
        fi.set(b, 7);
        fi.set(b, Integer.valueOf((Integer) fi.get(b) + 1));
        System.out.println("field-rw " + b.iv + " " + (Integer) fi.get(b));

        // ── Constructor.newInstance ──
        Constructor<Box> k0 = c.getDeclaredConstructor();
        Constructor<Box> k1 = c.getDeclaredConstructor(int.class);
        Box nb0 = k0.newInstance();
        Object raw = k1.newInstance(5);
        Box nb1 = (Box) raw;
        Shape ns = (Shape) raw;
        Base nbs = (Base) raw;
        Constructor<?> wild = c.getDeclaredConstructor(int.class);
        Box nb2 = (Box) wild.newInstance(6);
        StringBuilder sbd = (StringBuilder) StringBuilder.class.getConstructor(String.class).newInstance("sb");
        CharSequence csq = (CharSequence) StringBuilder.class.getConstructor(String.class).newInstance("cs");
        System.out.println("ctor " + nb0.name() + " " + nb1.name() + " " + ns.name() + " " + nbs.base()
                + " " + nb2.iv + " " + sbd.append('!') + " " + csq.length());
        Integer boxedCtor = (Integer) Integer.class.getConstructor(int.class).newInstance(77);
        System.out.println("ctor-wrapper " + boxedCtor + " " + (boxedCtor + 1));
    }
}
