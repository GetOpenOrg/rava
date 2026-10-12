
import java.io.ByteArrayInputStream;
import java.io.ByteArrayOutputStream;
import java.io.DataInputStream;
import java.io.DataOutputStream;
import java.io.IOException;
import java.nio.charset.StandardCharsets;
import java.io.DataInput;
import java.io.DataOutput;

/**
 * 综合实战：append-only 记录的两种编码对比（文章 PART 02 的 hello world 场景）。
 * 记录布局：type(1) + keyLen + key + valLen + value
 * - 固定版：keyLen/valLen 一律 writeInt，占 4 字节
 * - varint 版：keyLen/valLen 用 VariableLengthQuantity 编码，小长度只占 1 字节
 */
public class WalRecordDemo {

    private static final byte OP_INSERT = 1;

    public static void main(String[] args) throws IOException {
        String key = "hello";
        String value = "world";
        byte[] keyBytes = key.getBytes(StandardCharsets.UTF_8);
        byte[] valueBytes = value.getBytes(StandardCharsets.UTF_8);

        // ---------- 1. 固定 4 字节长度版 ----------
        ByteArrayOutputStream fixedBuf = new ByteArrayOutputStream();
        DataOutputStream fixedOut = new DataOutputStream(fixedBuf);
        writeFixedRecord(fixedOut, keyBytes, valueBytes);
        System.out.println("固定 int 版: " + hex(fixedBuf.toByteArray())
                + " 共 " + fixedBuf.size() + " 字节");

        // ---------- 2. varint 版 ----------
        ByteArrayOutputStream varBuf = new ByteArrayOutputStream();
        DataOutputStream varOut = new DataOutputStream(varBuf);
        writeVarintRecord(varOut, keyBytes, valueBytes);
        System.out.println("varint   版: " + hex(varBuf.toByteArray())
                + " 共 " + varBuf.size() + " 字节");
        System.out.println("节省: " + (fixedBuf.size() - varBuf.size()) + " 字节（"
                + Math.round((1 - varBuf.size() / (double) fixedBuf.size()) * 100) + "%）\n");

        // ---------- 3. 顺序读回 varint 记录 ----------
        DataInputStream in = new DataInputStream(new ByteArrayInputStream(varBuf.toByteArray()));
        byte op = in.readByte();
        int keyLen = VariableLengthQuantity.readVariableLengthInt(in);
        byte[] k = in.readNBytes(keyLen);
        int valLen = VariableLengthQuantity.readVariableLengthInt(in);
        byte[] v = in.readNBytes(valLen);
        System.out.println("读回: op=" + op + " keyLen=" + keyLen
                + " key=" + new String(k, StandardCharsets.UTF_8)
                + " valLen=" + valLen
                + " value=" + new String(v, StandardCharsets.UTF_8));
    }

    private static void writeFixedRecord(DataOutputStream out,
                                         byte[] key, byte[] value) throws IOException {
        out.writeByte(OP_INSERT);
        out.writeInt(key.length);
        out.write(key);
        out.writeInt(value.length);
        out.write(value);
    }

    private static void writeVarintRecord(DataOutputStream out,
                                          byte[] key, byte[] value) throws IOException {
        out.writeByte(OP_INSERT);
        VariableLengthQuantity.writeVariableLengthInt(out, key.length);
        out.write(key);
        VariableLengthQuantity.writeVariableLengthInt(out, value.length);
        out.write(value);
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
