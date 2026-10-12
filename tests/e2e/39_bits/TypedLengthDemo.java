
import java.io.ByteArrayOutputStream;
import java.io.DataInputStream;
import java.io.DataOutputStream;
import java.io.IOException;

/**
 * PART 03 思路一：增加 keyType 类型标记，动态写入 keyLen。
 * 固定 int 类型的 keyLen 存 5 → 00 00 00 05，前 3 字节全是 0，浪费 75%；
 * 思路一：写 1 字节 keyType（1/2/4 表示占用字节数）+ 动态长度的 keyLen，
 * 读的时候先读 type 再决定 readByte()/readShort()/readInt()。
 */
public class TypedLengthDemo {

    private static final byte TYPE_BYTE = 1;
    private static final byte TYPE_SHORT = 2;
    private static final byte TYPE_INT = 4;

    public static void main(String[] args) throws IOException {
        // 先看固定 4 字节的浪费
        ByteArrayOutputStream fixed = new ByteArrayOutputStream();
        new DataOutputStream(fixed).writeInt(5);
        System.out.println("固定 int 存 keyLen=5: " + hex(fixed.toByteArray())
                + "（有效信息只有最后 1 个字节）\n");

        int[] samples = {5, 127, 300, 32767, 70000};
        for (int len : samples) {
            ByteArrayOutputStream buf = new ByteArrayOutputStream();
            DataOutputStream out = new DataOutputStream(buf);
            writeLength(out, len);

            int read = readLength(new DataInputStream(
                    new java.io.ByteArrayInputStream(buf.toByteArray())));
            System.out.printf("len=%-8d 字节=%-14s 实际占用=%d 字节, 读回=%d %s%n",
                    len, hex(buf.toByteArray()), buf.size(), read, read == len ? "✓" : "✗");
        }
    }

    /** 思路一（有符号版）：按 byte/short/int 的有符号范围动态写入。 */
    static void writeLength(DataOutputStream out, int len) throws IOException {
        if (len <= 127) {                       // byte: -128 ~ 127
            out.writeByte(TYPE_BYTE);
            out.writeByte(len);
        } else if (len <= 32767) {              // short: -32768 ~ 32767
            out.writeByte(TYPE_SHORT);
            out.writeShort(len);
        } else {                                // int: 最多 2147483647
            out.writeByte(TYPE_INT);
            out.writeInt(len);
        }
    }

    /** 对应的读取：先读 1 字节类型，再按类型读取长度。 */
    static int readLength(DataInputStream in) throws IOException {
        byte type = in.readByte();
        if (type == TYPE_BYTE) {
            return in.readByte();
        } else if (type == TYPE_SHORT) {
            return in.readShort();
        } else {
            return in.readInt();
        }
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
