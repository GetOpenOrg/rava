import java.lang.reflect.Method;

// rd::unbox_* 的包装类取值回归（P-B 结构化取值改动的行为见证，reflect_dispatch.rs）：
// 翻译包装类实例经 Method.invoke 传入基本类型形参——Integer 全值域（valueOf 缓存内 / 外与
// 边界）、Short / Byte / Character 拓宽到 int、Boolean / Character / Float 各自路径、
// Long / Double 文本路径；并钉住拒收面（Boolean → int、String → int、Integer → char
// 均为 IllegalArgumentException，不窄化不解析陌生文本）。
public class TestUnboxWrapperFaces {
    static int takeInt(int x) { return x + 1; }
    static long takeLong(long x) { return x - 1; }
    static boolean takeBool(boolean z) { return !z; }
    static char takeChar(char c) { return (char) (c + 1); }
    static float takeFloat(float f) { return f * 2; }
    static double takeDouble(double d) { return d / 2; }

    public static void main(String[] args) throws Exception {
        Class<?> k = TestUnboxWrapperFaces.class;
        Method mi = k.getDeclaredMethod("takeInt", int.class);
        Method ml = k.getDeclaredMethod("takeLong", long.class);
        Method mz = k.getDeclaredMethod("takeBool", boolean.class);
        Method mc = k.getDeclaredMethod("takeChar", char.class);
        Method mf = k.getDeclaredMethod("takeFloat", float.class);
        Method md = k.getDeclaredMethod("takeDouble", double.class);
        for (Method m : new Method[] { mi, ml, mz, mc, mf, md }) m.setAccessible(true);

        // Integer 全值域（-128..127 缓存内、缓存外、边界）
        int[] ints = { 0, 1, -1, 127, 128, -128, -129, 255, 65535, Integer.MIN_VALUE, Integer.MAX_VALUE };
        long acc = 0;
        for (int x : ints) acc += (Integer) mi.invoke(null, Integer.valueOf(x)) + (long) x;
        System.out.println("ints " + acc);

        // 现场装箱（i & 255 值域跨缓存边界，热路径同形态）
        int round = 0;
        for (int i = 0; i < 600; i++) round += (Integer) mi.invoke(null, i & 255);
        System.out.println("round " + round);

        // Short / Byte / Character 拓宽到 int（JLS §5.1.2）
        System.out.println("widen " + (Integer) mi.invoke(null, Short.valueOf((short) -300))
                + " " + (Integer) mi.invoke(null, Byte.valueOf((byte) 100))
                + " " + (Integer) mi.invoke(null, Character.valueOf('A')));

        // Long / Double（hashCode 折叠不可逆，仍走文本路径）
        System.out.println("long " + (Long) ml.invoke(null, Long.valueOf(Long.MIN_VALUE)));
        System.out.println("double " + (Double) md.invoke(null, Double.valueOf(1.5e300)));
        // Boolean / Character / Float（Float.hashCode = floatToIntBits 按位还原）
        System.out.println("bool " + (Boolean) mz.invoke(null, Boolean.valueOf(false)));
        System.out.println("char " + (int) (char) (Character) mc.invoke(null, Character.valueOf('a')));
        float f1 = (Float) mf.invoke(null, Float.valueOf(2.5f));
        float f2 = (Float) mf.invoke(null, Float.valueOf(-0.0f));
        float f3 = (Float) mf.invoke(null, Float.valueOf(Float.NaN));
        float f4 = (Float) mf.invoke(null, Float.valueOf(Float.NEGATIVE_INFINITY));
        System.out.println("floats " + f1 + " " + (f2 == 0.0f && 1 / f2 < 0) + " " + f3 + " " + f4);

        // 拒收面：不窄化（Integer → char）、不跨种类（Boolean → int）、不解析陌生文本（"123" → int）
        try {
            mi.invoke(null, Boolean.TRUE);
            System.out.println("reject bool-int no-throw");
        } catch (IllegalArgumentException e) {
            System.out.println("reject bool-int IAE");
        } catch (Exception e) {
            System.out.println("reject bool-int " + e.getClass().getSimpleName());
        }
        try {
            mi.invoke(null, "123");
            System.out.println("reject str-int no-throw");
        } catch (IllegalArgumentException e) {
            System.out.println("reject str-int IAE");
        } catch (Exception e) {
            System.out.println("reject str-int " + e.getClass().getSimpleName());
        }
        try {
            mc.invoke(null, Integer.valueOf(65));
            System.out.println("reject int-char no-throw");
        } catch (IllegalArgumentException e) {
            System.out.println("reject int-char IAE");
        } catch (Exception e) {
            System.out.println("reject int-char " + e.getClass().getSimpleName());
        }
    }
}
