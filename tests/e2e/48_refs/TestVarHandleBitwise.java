import java.lang.invoke.MethodHandles;
import java.lang.invoke.VarHandle;

/**
 * VarHandle 的 getAndBitwise{Or,And,Xor}（含 Acquire / Release 档位）：签名多态方法，返回旧值。
 * 覆盖：实例字段（boolean / byte / short / char / int / long）、静态字段、数组元素（int[] / long[] / boolean[]）；
 * 相邻子字字段互不影响；浮点 / 引用族抛 UnsupportedOperationException。
 */
public class TestVarHandleBitwise {
    static class Bits {
        boolean z;
        byte b;
        short s;
        char c;
        int i;
        long l;
        byte guard = 0x55;
        double d;
        String r;
    }

    static int counter;

    public static void main(String[] args) throws Throwable {
        MethodHandles.Lookup lk = MethodHandles.lookup();
        VarHandle Z = lk.findVarHandle(Bits.class, "z", boolean.class);
        VarHandle B = lk.findVarHandle(Bits.class, "b", byte.class);
        VarHandle S = lk.findVarHandle(Bits.class, "s", short.class);
        VarHandle C = lk.findVarHandle(Bits.class, "c", char.class);
        VarHandle I = lk.findVarHandle(Bits.class, "i", int.class);
        VarHandle L = lk.findVarHandle(Bits.class, "l", long.class);
        VarHandle D = lk.findVarHandle(Bits.class, "d", double.class);
        VarHandle R = lk.findVarHandle(Bits.class, "r", String.class);
        VarHandle SI = lk.findStaticVarHandle(TestVarHandleBitwise.class, "counter", int.class);

        Bits f = new Bits();
        boolean oz = (boolean) Z.getAndBitwiseOr(f, true);
        boolean oz2 = (boolean) Z.getAndBitwiseXorAcquire(f, true);
        boolean oz3 = (boolean) Z.getAndBitwiseAndRelease(f, true);
        System.out.println("z: " + oz + " " + oz2 + " " + oz3 + " -> " + f.z);

        f.b = (byte) 0x0F;
        byte ob = (byte) B.getAndBitwiseOr(f, (byte) 0xF0);
        byte ob2 = (byte) B.getAndBitwiseAnd(f, (byte) 0x3C);
        byte ob3 = (byte) B.getAndBitwiseXorRelease(f, (byte) 0xFF);
        System.out.println("b: " + ob + " " + ob2 + " " + ob3 + " -> " + f.b + " guard=" + f.guard);

        f.s = (short) 0x1234;
        short os = (short) S.getAndBitwiseXor(f, (short) 0xFFFF);
        short os2 = (short) S.getAndBitwiseOrAcquire(f, (short) 0x8000);
        System.out.println("s: " + os + " " + os2 + " -> " + f.s);

        f.c = 'A';
        char oc = (char) C.getAndBitwiseOr(f, (char) 0x20);
        char oc2 = (char) C.getAndBitwiseAndAcquire(f, (char) 0xFFDF);
        System.out.println("c: " + oc + " " + oc2 + " -> " + f.c);

        f.i = 0b1010;
        int oi = (int) I.getAndBitwiseOr(f, 0b0101);
        int oi2 = (int) I.getAndBitwiseAnd(f, 0b0110);
        int oi3 = (int) I.getAndBitwiseXor(f, -1);
        int oi4 = (int) I.getAndBitwiseOrRelease(f, 1);
        System.out.println("i: " + oi + " " + oi2 + " " + oi3 + " " + oi4 + " -> " + f.i);

        f.l = 0x00000000FFFFFFFFL;
        long ol = (long) L.getAndBitwiseXor(f, 0xFFFFFFFF00000000L);
        long ol2 = (long) L.getAndBitwiseAndAcquire(f, 0x0F0F0F0F0F0F0F0FL);
        long ol3 = (long) L.getAndBitwiseOr(f, Long.MIN_VALUE);
        System.out.println("l: " + ol + " " + ol2 + " " + ol3 + " -> " + f.l);

        counter = 6;
        int oc1 = (int) SI.getAndBitwiseOr(1);
        int oc3 = (int) SI.getAndBitwiseXorRelease(3);
        System.out.println("static: " + oc1 + " " + oc3 + " -> " + counter);

        int[] ia = {1, 2, 4, 8};
        VarHandle IA = MethodHandles.arrayElementVarHandle(int[].class);
        int oa = (int) IA.getAndBitwiseOr(ia, 2, 3);
        int oa2 = (int) IA.getAndBitwiseXorAcquire(ia, 3, 0xFF);
        int oa3 = (int) IA.getAndBitwiseAndRelease(ia, 0, 0);
        System.out.println("int[]: " + oa + " " + oa2 + " " + oa3 + " -> " + ia[0] + "," + ia[1] + "," + ia[2] + "," + ia[3]);

        long[] la = {-1L, 5L};
        VarHandle LA = MethodHandles.arrayElementVarHandle(long[].class);
        long ola = (long) LA.getAndBitwiseAnd(la, 0, 0xFFL);
        long ola2 = (long) LA.getAndBitwiseXor(la, 1, 1L << 40);
        System.out.println("long[]: " + ola + " " + ola2 + " -> " + la[0] + "," + la[1]);

        boolean[] za = {false, true};
        VarHandle ZA = MethodHandles.arrayElementVarHandle(boolean[].class);
        boolean oza = (boolean) ZA.getAndBitwiseOr(za, 0, true);
        boolean oza2 = (boolean) ZA.getAndBitwiseXor(za, 1, true);
        System.out.println("boolean[]: " + oza + " " + oza2 + " -> " + za[0] + "," + za[1]);

        try {
            D.getAndBitwiseOr(f, 1.0);
            System.out.println("double: no exception");
        } catch (UnsupportedOperationException e) {
            System.out.println("double: UnsupportedOperationException");
        }
        try {
            R.getAndBitwiseXor(f, "x");
            System.out.println("ref: no exception");
        } catch (UnsupportedOperationException e) {
            System.out.println("ref: UnsupportedOperationException");
        }
    }
}
