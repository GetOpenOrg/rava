import java.lang.invoke.MethodHandles;
import java.lang.invoke.VarHandle;
import java.lang.reflect.Field;
import sun.misc.Unsafe;

/**
 * Unsafe 在基本类型数组元素上的访问（arrayBaseOffset + i × arrayIndexScale）：各元素类型的
 * get / put、int / long 元素的 CAS 与 getAndAdd / getAndSet；以 int 宽度按字对齐访问 byte[] /
 * short[]（一个字覆盖多个子字元素，CAS 只改该字覆盖的元素，字外相邻元素不变）；float / double
 * 元素按原始位读；实例 float / double 字段经 Unsafe 读写、经 VarHandle 的 CAS / getAndAdd /
 * getAndSet（按原始位比较：-0.0 与 0.0 不同、NaN 与自身相同）。
 */
public class TestUnsafePrimitiveArray {
    static class Holder {
        float f = 1.25f;
        double d = 2.5;
        int n = 3;
    }

    public static void main(String[] args) throws Throwable {
        Field tf = Unsafe.class.getDeclaredField("theUnsafe");
        tf.setAccessible(true);
        Unsafe u = (Unsafe) tf.get(null);

        int[] ia = {1, 2, 3, 4};
        long ib = u.arrayBaseOffset(int[].class);
        long is = u.arrayIndexScale(int[].class);
        System.out.println("int scale=" + is);
        System.out.println("cas[2] 3->30: " + u.compareAndSwapInt(ia, ib + 2 * is, 3, 30));
        System.out.println("cas[2] 3->31: " + u.compareAndSwapInt(ia, ib + 2 * is, 3, 31));
        System.out.println("getAndAdd[0] +10: " + u.getAndAddInt(ia, ib, 10));
        System.out.println("getAndSet[3] 99: " + u.getAndSetInt(ia, ib + 3 * is, 99));
        u.putInt(ia, ib + is, -7);
        System.out.println("get[1]=" + u.getInt(ia, ib + is) + " ia=" + java.util.Arrays.toString(ia));

        long[] la = {5L, 6L};
        long lb = u.arrayBaseOffset(long[].class);
        long ls = u.arrayIndexScale(long[].class);
        System.out.println("long scale=" + ls);
        System.out.println("cas[1] 6->60: " + u.compareAndSwapLong(la, lb + ls, 6L, 60L));
        System.out.println("getAndAdd[0] +1<<33: " + u.getAndAddLong(la, lb, 1L << 33));
        System.out.println("getAndSet[1]: " + u.getAndSetLong(la, lb + ls, -1L));
        System.out.println("la=" + java.util.Arrays.toString(la));

        byte[] ba = new byte[8];
        long bb = u.arrayBaseOffset(byte[].class);
        for (int k = 0; k < 8; k++) u.putByte(ba, bb + k, (byte) (k + 1));
        int w = u.getInt(ba, bb + 4);
        System.out.println("word@4=" + Integer.toHexString(w));
        System.out.println("byte word cas: " + u.compareAndSwapInt(ba, bb + 4, w, w ^ 0x0000ff00));
        System.out.println("byte word cas stale: " + u.compareAndSwapInt(ba, bb + 4, w, 0));
        System.out.println("ba=" + java.util.Arrays.toString(ba));

        short[] sa = {1, 2, 3, 4};
        long sb = u.arrayBaseOffset(short[].class);
        long ss = u.arrayIndexScale(short[].class);
        System.out.println("short scale=" + ss);
        int sw = u.getInt(sa, sb);
        System.out.println("short word cas: " + u.compareAndSwapInt(sa, sb, sw, (sw & 0xffff0000) | 0x7fff));
        u.putShort(sa, sb + 3 * ss, (short) -9);
        System.out.println("sa=" + java.util.Arrays.toString(sa));

        boolean[] za = new boolean[3];
        long zb = u.arrayBaseOffset(boolean[].class);
        u.putBoolean(za, zb + 1, true);
        System.out.println("za=" + java.util.Arrays.toString(za) + " get1=" + u.getBoolean(za, zb + 1));

        char[] ca = {'a', 'b'};
        long cb = u.arrayBaseOffset(char[].class);
        u.putChar(ca, cb + u.arrayIndexScale(char[].class), 'Q');
        System.out.println("ca=" + new String(ca) + " get0=" + u.getChar(ca, cb));

        float[] fa = {1f, 2f};
        long fb = u.arrayBaseOffset(float[].class);
        u.putFloat(fa, fb + 4, -3.5f);
        System.out.println("fa=" + java.util.Arrays.toString(fa) + " bits0=" + Integer.toHexString(u.getInt(fa, fb)));

        double[] da = {1.0, 2.0};
        long db = u.arrayBaseOffset(double[].class);
        u.putDouble(da, db, 0.125);
        System.out.println("da=" + java.util.Arrays.toString(da) + " bits1=" + Long.toHexString(u.getLong(da, db + 8)));

        Holder h = new Holder();
        long fo = u.objectFieldOffset(Holder.class.getDeclaredField("f"));
        long dO = u.objectFieldOffset(Holder.class.getDeclaredField("d"));
        u.putFloat(h, fo, u.getFloat(h, fo) * 2);
        u.putDouble(h, dO, u.getDouble(h, dO) + 0.5);
        System.out.println("holder: f=" + h.f + " d=" + h.d + " n=" + h.n);

        MethodHandles.Lookup lk = MethodHandles.lookup();
        VarHandle HF = lk.findVarHandle(Holder.class, "f", float.class);
        VarHandle HD = lk.findVarHandle(Holder.class, "d", double.class);
        h.f = Float.NaN;
        h.d = 0.0;
        System.out.println("HF.cas(NaN,3.5)=" + HF.compareAndSet(h, Float.NaN, 3.5f));
        System.out.println("HF.getAndAdd(1)=" + (float) HF.getAndAdd(h, 1f));
        System.out.println("HD.cas(-0.0,2)=" + HD.compareAndSet(h, -0.0, 2.0));
        System.out.println("HD.cae(0.0,2.5)=" + (double) HD.compareAndExchange(h, 0.0, 2.5));
        System.out.println("HD.getAndSet(-1)=" + (double) HD.getAndSet(h, -1.0));
        System.out.println("holder: f=" + h.f + " d=" + h.d + " n=" + h.n);
    }
}
