import java.lang.invoke.MethodHandles;
import java.lang.invoke.VarHandle;
import java.nio.ByteOrder;

/**
 * MethodHandles.byteArrayViewVarHandle（K-JCA：sun.security.provider.ByteArrayAccess 的 LE/BE
 * 字与字节块转换经它实现，MD5/SHA 揭出）。覆盖：short/char/int/long/float/double 六种视图 ×
 * 大小端、get/set、非对齐偏移、withInvokeExactBehavior 后仍可用、越界 → AIOOBE（消息与 JDK 一致）。
 */
public class TestByteArrayViewVarHandle {
    static String hex(byte[] b) {
        StringBuilder sb = new StringBuilder();
        for (byte x : b) sb.append(String.format("%02x", x & 0xff));
        return sb.toString();
    }

    public static void main(String[] args) {
        for (ByteOrder order : new ByteOrder[] {ByteOrder.BIG_ENDIAN, ByteOrder.LITTLE_ENDIAN}) {
            System.out.println("== " + order);
            VarHandle ints = MethodHandles.byteArrayViewVarHandle(int[].class, order).withInvokeExactBehavior();
            VarHandle longs = MethodHandles.byteArrayViewVarHandle(long[].class, order);
            VarHandle shorts = MethodHandles.byteArrayViewVarHandle(short[].class, order);
            VarHandle chars = MethodHandles.byteArrayViewVarHandle(char[].class, order);
            VarHandle floats = MethodHandles.byteArrayViewVarHandle(float[].class, order);
            VarHandle doubles = MethodHandles.byteArrayViewVarHandle(double[].class, order);

            byte[] b = new byte[20];
            ints.set(b, 1, 0x01020304);
            System.out.println("int set@1 " + hex(b));
            System.out.println("int get@1 " + Integer.toHexString((int) ints.get(b, 1)));
            System.out.println("int get@0 " + Integer.toHexString((int) ints.get(b, 0)));

            b = new byte[20];
            longs.set(b, 3, 0x1122334455667788L);
            System.out.println("long " + hex(b) + " " + Long.toHexString((long) longs.get(b, 3)));

            b = new byte[4];
            shorts.set(b, 0, (short) -2);
            chars.set(b, 2, 'A');
            System.out.println("short/char " + hex(b) + " " + (short) shorts.get(b, 0) + " " + (char) chars.get(b, 2));

            b = new byte[12];
            floats.set(b, 0, 1.5f);
            doubles.set(b, 4, -2.25);
            System.out.println("float/double " + hex(b) + " " + (float) floats.get(b, 0) + " " + (double) doubles.get(b, 4));

            try {
                int x = (int) ints.get(new byte[6], 3);
                System.out.println("no exception? " + x);
            } catch (ArrayIndexOutOfBoundsException e) {
                System.out.println("AIOOBE " + e.getMessage());
            }
            try {
                longs.set(new byte[8], -1, 1L);
            } catch (ArrayIndexOutOfBoundsException e) {
                System.out.println("AIOOBE " + e.getMessage());
            }
        }
    }
}
