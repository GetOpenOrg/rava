import java.lang.invoke.MethodHandles;
import java.lang.invoke.VarHandle;

/**
 * VarHandle 的 volatile 访问模式落到 Unsafe.get/put{Boolean,Byte,Short,Char,Float,Double}Volatile
 * （HotSpot native）。覆盖：实例字段、静态字段、数组元素三种基址；跨线程发布（volatile 写 → 读可见）。
 */
public class TestVolatilePrimitiveAccess {
    boolean z; byte b; short s; char c; float f; double d;
    static boolean sz; static byte sb; static short ss; static char sc; static float sf; static double sd;

    static final VarHandle Z, B, S, C, F, D, SZ, SB, SS, SC, SF, SD;
    static {
        try {
            MethodHandles.Lookup l = MethodHandles.lookup();
            Class<?> k = TestVolatilePrimitiveAccess.class;
            Z = l.findVarHandle(k, "z", boolean.class);
            B = l.findVarHandle(k, "b", byte.class);
            S = l.findVarHandle(k, "s", short.class);
            C = l.findVarHandle(k, "c", char.class);
            F = l.findVarHandle(k, "f", float.class);
            D = l.findVarHandle(k, "d", double.class);
            SZ = l.findStaticVarHandle(k, "sz", boolean.class);
            SB = l.findStaticVarHandle(k, "sb", byte.class);
            SS = l.findStaticVarHandle(k, "ss", short.class);
            SC = l.findStaticVarHandle(k, "sc", char.class);
            SF = l.findStaticVarHandle(k, "sf", float.class);
            SD = l.findStaticVarHandle(k, "sd", double.class);
        } catch (ReflectiveOperationException e) {
            throw new ExceptionInInitializerError(e);
        }
    }

    public static void main(String[] args) throws Exception {
        TestVolatilePrimitiveAccess o = new TestVolatilePrimitiveAccess();
        Z.setVolatile(o, true);
        B.setVolatile(o, (byte) -7);
        S.setVolatile(o, (short) 31000);
        C.setVolatile(o, 'Q');
        F.setVolatile(o, 1.5f);
        D.setVolatile(o, -2.25);
        System.out.println("instance: " + (boolean) Z.getVolatile(o) + " " + (byte) B.getVolatile(o) + " "
                + (short) S.getVolatile(o) + " " + (char) C.getVolatile(o) + " "
                + (float) F.getVolatile(o) + " " + (double) D.getVolatile(o));
        System.out.println("plain view: " + o.z + " " + o.b + " " + o.s + " " + o.c + " " + o.f + " " + o.d);

        SZ.setVolatile(true);
        SB.setVolatile((byte) 120);
        SS.setVolatile((short) -1);
        SC.setVolatile('é');
        SF.setVolatile(Float.NaN);
        SD.setVolatile(Double.MAX_VALUE);
        System.out.println("static: " + (boolean) SZ.getVolatile() + " " + (byte) SB.getVolatile() + " "
                + (short) SS.getVolatile() + " " + (int) (char) SC.getVolatile() + " "
                + (float) SF.getVolatile() + " " + (double) SD.getVolatile());

        boolean[] za = new boolean[3]; byte[] ba = new byte[3]; short[] sa = new short[3];
        char[] ca = new char[3]; float[] fa = new float[3]; double[] da = new double[3];
        VarHandle zh = MethodHandles.arrayElementVarHandle(boolean[].class);
        VarHandle bh = MethodHandles.arrayElementVarHandle(byte[].class);
        VarHandle sh = MethodHandles.arrayElementVarHandle(short[].class);
        VarHandle ch = MethodHandles.arrayElementVarHandle(char[].class);
        VarHandle fh = MethodHandles.arrayElementVarHandle(float[].class);
        VarHandle dh = MethodHandles.arrayElementVarHandle(double[].class);
        for (int i = 0; i < 3; i++) {
            zh.setVolatile(za, i, i % 2 == 0);
            bh.setVolatile(ba, i, (byte) (i * 50));
            sh.setVolatile(sa, i, (short) (i * -1000));
            ch.setVolatile(ca, i, (char) ('x' + i));
            fh.setVolatile(fa, i, i / 4f);
            dh.setVolatile(da, i, i * 1e100);
        }
        StringBuilder sbuf = new StringBuilder("array:");
        for (int i = 0; i < 3; i++) {
            sbuf.append(' ').append((boolean) zh.getVolatile(za, i)).append('/').append((byte) bh.getVolatile(ba, i))
                .append('/').append((short) sh.getVolatile(sa, i)).append('/').append((char) ch.getVolatile(ca, i))
                .append('/').append((float) fh.getVolatile(fa, i)).append('/').append((double) dh.getVolatile(da, i));
        }
        System.out.println(sbuf);
        System.out.println("plain array: " + za[2] + " " + ba[2] + " " + sa[2] + " " + ca[2] + " " + fa[2] + " " + da[2]);

        // 跨线程发布：写线程按序 volatile 写数据与标志，读线程见标志后数据必可见
        TestVolatilePrimitiveAccess shared = new TestVolatilePrimitiveAccess();
        Thread w = new Thread(() -> {
            D.setVolatile(shared, 42.0);
            C.setVolatile(shared, 'R');
            Z.setVolatile(shared, true);
        });
        w.start();
        while (!(boolean) Z.getVolatile(shared)) {
            Thread.onSpinWait();
        }
        System.out.println("published: " + (double) D.getVolatile(shared) + " " + (char) C.getVolatile(shared));
        w.join();
    }
}
