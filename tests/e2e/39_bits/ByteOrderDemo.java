
import java.io.ByteArrayOutputStream;
import java.io.DataOutputStream;
import java.io.IOException;
import java.nio.ByteBuffer;
import java.nio.ByteOrder;

/**
 * PART 01 引言：ByteOrder 大端序与小端序。
 * 同一个 int 0x12345678（305419896）按不同字节序落盘，字节顺序相反；
 * 只要“按写的顺序读”，就能无损还原。
 */
public class ByteOrderDemo {

    private static final int VALUE = 0x12345678;

    public static void main(String[] args) throws IOException {
        System.out.println("int 值: " + VALUE + " (0x" + Integer.toHexString(VALUE) + ")");

        // ---------- 1. ByteBuffer 指定字节序 ----------
        ByteBuffer big = ByteBuffer.allocate(4).order(ByteOrder.BIG_ENDIAN);
        big.putInt(VALUE);
        System.out.println("大端序(高位先写): " + hex(big.array()));    // 12 34 56 78

        ByteBuffer little = ByteBuffer.allocate(4).order(ByteOrder.LITTLE_ENDIAN);
        little.putInt(VALUE);
        System.out.println("小端序(低位先写): " + hex(little.array()));  // 78 56 34 12

        // ---------- 2. Java 的 DataOutputStream 默认就是大端序 ----------
        ByteArrayOutputStream buf = new ByteArrayOutputStream();
        new DataOutputStream(buf).writeInt(VALUE);
        System.out.println("DataOutputStream.writeInt: " + hex(buf.toByteArray()));

        // ---------- 3. 按写的顺序读，即可无损还原 ----------
        System.out.println("大端序读回: " + ByteBuffer.wrap(big.array())
                .order(ByteOrder.BIG_ENDIAN).getInt());
        System.out.println("小端序读回: " + ByteBuffer.wrap(little.array())
                .order(ByteOrder.LITTLE_ENDIAN).getInt());
    }

    private static String hex(byte[] bytes) {
        StringBuilder sb = new StringBuilder();
        for (byte b : bytes) {
            if (sb.length() > 0) {
                sb.append(' ');
            }
            sb.append(String.format("%02X", b));
        }
        return sb.toString();
    }
}
