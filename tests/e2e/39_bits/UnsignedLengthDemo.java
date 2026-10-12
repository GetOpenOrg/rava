
import java.io.ByteArrayInputStream;
import java.io.ByteArrayOutputStream;
import java.io.DataInputStream;
import java.io.DataOutputStream;
import java.io.IOException;

/**
 * PART 05 思路二：用 & 0xFF / & 0xFFFF 把 byte/short 当“无符号”用，扩大取值范围。
 * 思路一按有符号范围切档（127/32767），思路二按无符号范围切档（255/65535），
 * 同样的 1 字节能表达 0~255，让更多长度值落进更小的档位。
 */
public class UnsignedLengthDemo {

    private static final byte TYPE_UBYTE = 1;
    private static final byte TYPE_USHORT = 2;
    private static final byte TYPE_INT = 4;

    public static void main(String[] args) throws IOException {
        // 先看不用掩码的坑：200 落进 byte 后符号扩展成负数
        System.out.println("(byte)200 直接读 = " + (byte) 200
                + "，& 0xFF 后 = " + ((byte) 200 & 0xFF) + "\n");

        int[] samples = {5, 127, 200, 255, 256, 65535, 65536, 70000};
        for (int len : samples) {
            ByteArrayOutputStream buf = new ByteArrayOutputStream();
            DataOutputStream out = new DataOutputStream(buf);
            writeLength(out, len);

            int read = readLength(new DataInputStream(new ByteArrayInputStream(buf.toByteArray())));
            System.out.printf("len=%-8d 字节=%-16s 占用=%d 字节, 读回=%d %s%n",
                    len, hex(buf.toByteArray()), buf.size(), read, read == len ? "✓" : "✗");
        }
    }

    /** 思路二：按无符号范围 255 / 65535 动态写入。 */
    static void writeLength(DataOutputStream out, int len) throws IOException {
        if (len <= 0xFF) {
            out.writeByte(TYPE_UBYTE);
            out.writeByte(len);          // 0~255 窄化进 1 字节（低 8 位无损）
        } else if (len <= 0xFFFF) {
            out.writeByte(TYPE_USHORT);
            out.writeShort(len);         // 0~65535 窄化进 2 字节（低 16 位无损）
        } else {
            out.writeByte(TYPE_INT);
            out.writeInt(len);
        }
    }

    /** 读取时用 & 0xFF / & 0xFFFF 抑制符号扩展，还原无符号数值。 */
    static int readLength(DataInputStream in) throws IOException {
        byte type = in.readByte();
        if (type == TYPE_UBYTE) {
            return in.readByte() & 0xFF;
        } else if (type == TYPE_USHORT) {
            return in.readShort() & 0xFFFF;
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
