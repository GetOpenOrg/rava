import java.lang.reflect.Field;
import java.lang.reflect.Method;
import java.util.HashSet;
import java.util.Set;

/**
 * 反射产出的装箱值（Field.get / Method.invoke 返回）按包装类值语义：hashCode 与
 * Integer/Long/.../Double.hashCode 一致；equals 同类型同值为 true、不同值 / 跨类型 / null
 * 为 false；Double NaN 自等、0.0 与 -0.0 不等；装箱值在 HashSet 中按值去重。
 */
public class TestBoxedValueSemantics {
    int i = 7;
    long l = -123456789012345L;
    boolean z = true;
    static final short S = -3;
    static final byte B = 100;
    static final char C = 'Z';

    public double dbl(double v) { return v; }
    public float flt(float v) { return v; }

    public static void main(String[] args) throws Exception {
        TestBoxedValueSemantics o = new TestBoxedValueSemantics();
        Class<?> k = TestBoxedValueSemantics.class;
        Object bi = k.getDeclaredField("i").get(o);
        Object bl = k.getDeclaredField("l").get(o);
        Object bz = k.getDeclaredField("z").get(o);
        Object bs = k.getDeclaredField("S").get(null);
        Object bb = k.getDeclaredField("B").get(null);
        Object bc = k.getDeclaredField("C").get(null);
        System.out.println("hash i=" + bi.hashCode() + " l=" + bl.hashCode() + " z=" + bz.hashCode()
                + " S=" + bs.hashCode() + " B=" + bb.hashCode() + " C=" + bc.hashCode());

        System.out.println("i eq same " + bi.equals(k.getDeclaredField("i").get(o))
                + " eq Integer " + bi.equals(Integer.valueOf(7))
                + " eq 8 " + bi.equals(Integer.valueOf(8))
                + " eq Long " + bi.equals(Long.valueOf(7))
                + " eq null " + bi.equals(null));
        System.out.println("l eq " + bl.equals(Long.valueOf(-123456789012345L)) + " z eq " + bz.equals(Boolean.TRUE)
                + " z eq false " + bz.equals(Boolean.FALSE));
        System.out.println("S eq " + bs.equals(Short.valueOf((short) -3)) + " B eq " + bb.equals(Byte.valueOf((byte) 100))
                + " C eq " + bc.equals(Character.valueOf('Z')) + " C eq Integer " + bc.equals(Integer.valueOf(90)));

        Method md = k.getMethod("dbl", double.class);
        Method mf = k.getMethod("flt", float.class);
        Object d1 = md.invoke(o, 3.25);
        Object nan1 = md.invoke(o, Double.NaN);
        Object nan2 = md.invoke(o, 0.0 / 0.0);
        Object pz = md.invoke(o, 0.0);
        Object nz = md.invoke(o, -0.0);
        Object f1 = mf.invoke(o, 1.5f);
        Object fnan = mf.invoke(o, Float.NaN);
        System.out.println("hash d=" + d1.hashCode() + " nan=" + nan1.hashCode() + " -0.0=" + nz.hashCode()
                + " f=" + f1.hashCode() + " fnan=" + fnan.hashCode());
        System.out.println("nan eq nan " + nan1.equals(nan2) + " 0.0 eq -0.0 " + pz.equals(nz)
                + " d eq Double " + d1.equals(Double.valueOf(3.25)) + " f eq Double " + f1.equals(Double.valueOf(1.5)));

        Set<Object> set = new HashSet<>();
        set.add(bi);
        set.add(k.getDeclaredField("i").get(o));
        set.add(nan1);
        set.add(nan2);
        System.out.println("set size " + set.size());
    }
}
