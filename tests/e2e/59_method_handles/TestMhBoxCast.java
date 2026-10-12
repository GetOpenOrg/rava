import java.lang.invoke.MethodHandle;
import java.lang.invoke.MethodHandles;
import java.lang.invoke.MethodType;
import java.lang.invoke.WrongMethodTypeException;

/**
 * 方法句柄 invoke 路径的装箱失配见证（fix-invoke-box-cast 的同类风险面核查，
 * docs/tasks.md「剩余工作」第 10 条）。
 *
 * 基本类型返回值在引用上下文（(Integer) / (Object) / (Number) 强转、instanceof、
 * getClass）经 asType 装箱；基本类型实参（含 char/byte/short/boolean 子字）入参与
 * 子字窄化；invokeWithArguments 的装箱链；错误包装类强转的 ClassCastException
 * 与 invokeExact 类型不符的 WrongMethodTypeException（拒收面）。
 */
public class TestMhBoxCast {
    static int retInt(int x) { return x + 1; }
    static long retLong(long x) { return x * 2; }
    static boolean retBool(boolean x) { return !x; }
    static char retChar(int x) { return (char) ('A' + x); }
    static byte retByte(byte x) { return (byte) (x - 1); }
    static short retShort(short x) { return (short) (x + 10); }
    static float retFloat(float x) { return x / 2; }
    static double retDouble(double x) { return x * 3; }
    static String kinds(char c, byte b, short s, boolean z, int i, long l, float f, double d) {
        return c + "/" + b + "/" + s + "/" + z + "/" + i + "/" + l + "/" + f + "/" + d;
    }
    static Integer retBoxed(int x) { return x * 10; }

    static class Cell {
        int v = 5;
    }

    public static void main(String[] args) throws Throwable {
        MethodHandles.Lookup lookup = MethodHandles.lookup();

        // ── 基本类型返回值 × 引用上下文：asType 装箱 + checkcast / instanceof / getClass ──
        MethodHandle hi = lookup.findStatic(TestMhBoxCast.class, "retInt",
                MethodType.methodType(int.class, int.class));
        Integer bi = (Integer) hi.invoke(41);
        Object oi = (Object) hi.invoke(42);
        Number nmi = (Number) hi.invoke(43);
        System.out.println("int-box " + bi + " " + oi + " " + nmi.intValue());
        System.out.println("int-inst " + (oi instanceof Integer) + " " + (oi instanceof Number)
                + " " + (oi instanceof Long) + " " + oi.getClass().getName());

        MethodHandle hl = lookup.findStatic(TestMhBoxCast.class, "retLong",
                MethodType.methodType(long.class, long.class));
        Long bl = (Long) hl.invoke(21L);
        Object ol = (Object) hl.invoke(22L);
        System.out.println("long-box " + bl + " " + ol + " " + ol.getClass().getName());

        MethodHandle hz = lookup.findStatic(TestMhBoxCast.class, "retBool",
                MethodType.methodType(boolean.class, boolean.class));
        Boolean bz = (Boolean) hz.invoke(false);
        Object oz = (Object) hz.invoke(true);
        System.out.println("bool-box " + bz + " " + oz + " " + oz.getClass().getName());

        MethodHandle hc = lookup.findStatic(TestMhBoxCast.class, "retChar",
                MethodType.methodType(char.class, int.class));
        Character bc = (Character) hc.invoke(2);
        Object oc = (Object) hc.invoke(3);
        Comparable<Character> cc = (Comparable<Character>) hc.invoke(1);
        System.out.println("char-box " + bc + " " + oc + " " + cc.compareTo('B'));

        MethodHandle hb = lookup.findStatic(TestMhBoxCast.class, "retByte",
                MethodType.methodType(byte.class, byte.class));
        Byte bb = (Byte) hb.invoke((byte) 9);
        Number nb = (Number) hb.invoke((byte) 7);
        System.out.println("byte-box " + bb + " " + nb.byteValue());

        MethodHandle hs = lookup.findStatic(TestMhBoxCast.class, "retShort",
                MethodType.methodType(short.class, short.class));
        Short bs = (Short) hs.invoke((short) 100);
        Number ns = (Number) hs.invoke((short) 200);
        System.out.println("short-box " + bs + " " + ns.shortValue());

        MethodHandle hf = lookup.findStatic(TestMhBoxCast.class, "retFloat",
                MethodType.methodType(float.class, float.class));
        Float bf = (Float) hf.invoke(5.0f);
        Number nf = (Number) hf.invoke(7.0f);
        System.out.println("float-box " + bf + " " + nf.floatValue() + " " + bf.getClass().getName());

        MethodHandle hd = lookup.findStatic(TestMhBoxCast.class, "retDouble",
                MethodType.methodType(double.class, double.class));
        Double bd = (Double) hd.invoke(1.5);
        Number nd = (Number) hd.invoke(2.5);
        System.out.println("double-box " + bd + " " + nd.doubleValue() + " " + bd.getClass().getName());

        // ── 基本类型上下文（拆箱还原，既有路径的对照）──
        int pi = (int) hi.invoke(19);
        char pc = (char) hc.invoke(4);
        double pd = (double) hd.invoke(3.5);
        System.out.println("prim-ctx " + pi + " " + pc + " " + pd);

        // ── 子字实参：invoke 的适配窄化（Integer 流动 → char/byte/short/boolean 形参）──
        MethodHandle hk = lookup.findStatic(TestMhBoxCast.class, "kinds", MethodType.methodType(
                String.class, char.class, byte.class, short.class, boolean.class,
                int.class, long.class, float.class, double.class));
        System.out.println("subword-invoke " + hk.invoke('x', (byte) 1, (short) 2, true, 3, 4L, 5.5f, 6.25));
        System.out.println("subword-exact " + (String) hk.invokeExact('y', (byte) 8, (short) 9, false, 7, 6L, 5.0f, 4.0));
        System.out.println("subword-with " + hk.invokeWithArguments('z', (byte) 0, (short) 5, true, 2, 1L, 0.5f, 0.25));

        // ── invokeWithArguments：泛型适配链的装箱（Object 返回经强转 / instanceof / getClass）──
        Object wi = hi.invokeWithArguments(50);
        Integer wbi = (Integer) hi.invokeWithArguments(51);
        System.out.println("with-int " + wi + " " + wbi + " " + (wi instanceof Number)
                + " " + wi.getClass().getName());
        Object wl = hl.invokeWithArguments(5L);
        Object wc = hc.invokeWithArguments(6);
        Object wb = hz.invokeWithArguments(false);
        System.out.println("with-kinds " + wl + " " + wc + " " + wb + " "
                + wl.getClass().getName() + " " + wc.getClass().getName());

        // ── 字段 / identity / constant 句柄的装箱边界 ──
        MethodHandle gv = lookup.findGetter(Cell.class, "v", int.class);
        Object og = (Object) gv.invoke(new Cell());
        Integer ig = (Integer) gv.invoke(new Cell());
        System.out.println("getter-box " + og + " " + ig + " " + og.getClass().getName());

        Object oid = (Object) MethodHandles.identity(int.class).invoke(8);
        Integer iid = (Integer) MethodHandles.identity(int.class).invoke(9);
        System.out.println("identity-box " + oid + " " + iid + " " + oid.getClass().getName());

        // ── 引用返回句柄（invokeExact 的引用上下文合法面）──
        MethodHandle hbox = lookup.findStatic(TestMhBoxCast.class, "retBoxed",
                MethodType.methodType(Integer.class, int.class));
        Integer eb = (Integer) hbox.invokeExact(3);
        Integer eob = (Integer) hbox.invokeExact(4);
        System.out.println("ref-exact " + eb + " " + eob + " " + eob.getClass().getName());
        Integer ai = (Integer) hbox.invoke(5);
        Number an = (Number) hbox.invoke(6);
        System.out.println("ref-invoke " + ai + " " + an.intValue());

        // ── 拒收面：invoke 引用上下文不可转换（int 不可转 Long/Integer 的错误包装）→
        //    asType 适配失败抛 WrongMethodTypeException（JVM 在适配时拒绝，非 CCE）──
        try {
            Long bad = (Long) hi.invoke(1);
            System.out.println("no-wmte " + bad);
        } catch (WrongMethodTypeException e) {
            System.out.println("mh-wmte-int-to-long");
        }
        try {
            Integer bad2 = (Integer) hc.invoke(0);
            System.out.println("no-wmte " + bad2);
        } catch (WrongMethodTypeException e) {
            System.out.println("mh-wmte-char-to-int");
        }
        // ── 拒收面：invokeWithArguments 返回 Object 后强转错误包装类 → ClassCastException ──
        try {
            Double bad3 = (Double) hi.invokeWithArguments(2);
            System.out.println("no-cce " + bad3);
        } catch (ClassCastException e) {
            System.out.println("mh-with-cce-int-to-double " + e.getClass().getSimpleName());
        }

        // ── 拒收面：invokeExact 引用上下文 ≠ 句柄类型 → WrongMethodTypeException ──
        try {
            Integer bad4 = (Integer) hi.invokeExact(3);
            System.out.println("no-wmte " + bad4);
        } catch (WrongMethodTypeException e) {
            System.out.println("wmte-int-ref");
        }
        try {
            Object bad5 = (Object) hi.invokeExact(4);
            System.out.println("no-wmte " + bad5);
        } catch (WrongMethodTypeException e) {
            System.out.println("wmte-int-obj");
        }
    }
}
