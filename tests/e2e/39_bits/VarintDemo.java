
import java.io.ByteArrayInputStream;
import java.io.ByteArrayOutputStream;
import java.io.DataInputStream;
import java.io.DataOutputStream;
import java.io.IOException;
import java.io.DataInput;
import java.io.DataOutput;

/**
 * PART 05 思路三：varint 编码演示。
 * 以 300 为例：300 = 1 0010 1100（9 位）
 *   第一字节 (300 & 0x7F) | 0x80 = 0xAC（还有后续）
 *   第二字节 300 >>> 7 = 0x02（最后一字节）
 * 与思路一/二相比，varint 让“存储格式按实际数据密度付费”：
 * 0~127 占 1 字节，128~16383 占 2 字节……而不是按类型上限付 4 字节。
 */
public class VarintDemo {

    public static void main(String[] args) throws IOException {
        // ---------- 1. 文章手推的 300 编码过程 ----------
        System.out.println("300 的二进制补码: "
                + String.format("%32s", Integer.toBinaryString(300)).replace(' ', '0'));

        ByteArrayOutputStream buf = new ByteArrayOutputStream();
        DataOutputStream out = new DataOutputStream(buf);
        VariableLengthQuantity.writeVariableLengthInt(out, 300);
        System.out.println("300 的 varint 编码: " + hex(buf.toByteArray()) + "（期望 AC 02）");

        int decoded = VariableLengthQuantity.readVariableLengthInt(
                new DataInputStream(new ByteArrayInputStream(buf.toByteArray())));
        System.out.println("解码回: " + decoded + "\n");

        // ---------- 2. 各档位边界的占用字节数 ----------
        int[] boundaries = {0, 127, 128, 16383, 16384, 2097151, 2097152,
                268435455, 268435456, Integer.MAX_VALUE};
        System.out.println("值              varint字节数   固定int字节数");
        for (int v : boundaries) {
            System.out.printf("%-15d %-13d %d%n",
                    v, VariableLengthQuantity.variableLengthSize(v), 4);
        }

        // ---------- 3. 随机值往返校验 ----------
        java.util.Random random = new java.util.Random(42);
        int passed = 0;
        for (int i = 0; i < 100_000; i++) {
            int value = random.nextInt();
            ByteArrayOutputStream b = new ByteArrayOutputStream(5);
            VariableLengthQuantity.writeVariableLengthInt(
                    new DataOutputStream(b), value);
            int back = VariableLengthQuantity.readVariableLengthInt(
                    new DataInputStream(new ByteArrayInputStream(b.toByteArray())));
            if (back == value) {
                passed++;
            }
        }
        System.out.println("\n10 万个随机 int varint 往返校验通过: " + passed);
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



/**
 * PART 05 思路三：varint（Variable Byte）工具类。
 * 参考《信息检索导论》5.3.1 与 Presto Airlift 的 VariableLengthQuantity 实现，
 * 把 SliceOutput/SliceInput 换成 JDK 的 DataOutput/DataInput。
 *
 * 编码规则（Protobuf/SQLite 同款约定）：
 * - 每个字节的低 7 位存有效数值
 * - 最高位为 1：后面还有字节要接着读（continuation bit）
 * - 最高位为 0：这是最后一个字节
 */class VariableLengthQuantity {

    private VariableLengthQuantity() {
    }

    /** 计算 varint 编码占用的字节数（1~5 字节）。 */
    public static int variableLengthSize(int value) {
        int size = 1;
        while ((value & ~0x7f) != 0) {  // 还有超出低 7 位的部分
            value >>>= 7;
            size++;
        }
        return size;
    }

    /** varint 编码 int：循环版，比按档位分支的写法更通用。 */
    public static void writeVariableLengthInt(DataOutput out, int value) throws IOException {
        while ((value & ~0x7f) != 0) {
            // 低 7 位 + 续位标志 0x80；writeByte 窄化只保留低 8 位
            out.writeByte((value & 0x7f) | 0x80);
            value >>>= 7;               // 无符号右移，处理下一组 7 位
        }
        out.writeByte(value);           // 最后一字节，最高位自然是 0
    }

    /** varint 编码 long（最多 10 字节）。 */
    public static void writeVariableLengthLong(DataOutput out, long value) throws IOException {
        while ((value & ~0x7fL) != 0) {
            out.writeByte((int) ((value & 0x7fL) | 0x80L));
            value >>>= 7;
        }
        out.writeByte((int) value);
    }

    /** varint 解码 int：逐字节拼接低 7 位，直到最高位为 0。 */
    public static int readVariableLengthInt(DataInput in) throws IOException {
        int result = 0;
        for (int shift = 0; shift <= 28; shift += 7) {
            int b = in.readUnsignedByte();
            result |= (b & 0x7f) << shift;   // 当前字节的低 7 位放到对应位置
            if ((b & 0x80) == 0) {
                return result;               // 续位为 0，编码结束
            }
        }
        throw new NumberFormatException("varint 的最后一个字节最高位仍为 1");
    }

    /** varint 解码 long。 */
    public static long readVariableLengthLong(DataInput in) throws IOException {
        long result = 0;
        for (int shift = 0; shift <= 63; shift += 7) {
            long b = in.readUnsignedByte();
            result |= (b & 0x7f) << shift;
            if ((b & 0x80) == 0) {
                return result;
            }
        }
        throw new NumberFormatException("varlong 的最后一个字节最高位仍为 1");
    }
}
