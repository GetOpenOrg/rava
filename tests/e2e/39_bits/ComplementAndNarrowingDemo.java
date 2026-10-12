
import java.io.ByteArrayOutputStream;
import java.io.DataOutputStream;
import java.io.IOException;

/**
 * PART 04（上）：补码与窄化转换。
 * - 补码：byte 的模是 256，-56 与 +200 是同一串位模式（1100 1000）的两种解读
 * - 窄化转换：writeByte() 只保留 int 的低 8 位，高位直接丢弃
 *   writeByte(300) → 44（丢掉高位 1），writeByte(200) → -56（符号位变 1）
 */
public class ComplementAndNarrowingDemo {

    public static void main(String[] args) throws IOException {
        // ---------- 1. 补码 = 模运算的补数 ----------
        System.out.println("56 的二进制      = " + toBits8(56));        // 0011 1000
        System.out.println("取反            = " + toBits8(~56));       // 1100 0111
        System.out.println("+1 得补码       = " + toBits8(-56));       // 1100 1000
        System.out.println("256 - 56        = " + (256 - 56));          // 200：与补码位模式一致

        // ---------- 2. 同一串位模式的两种解读 ----------
        byte b = (byte) 200;
        System.out.println("位模式 1100 1000：有符号解读 = " + b
                + "，无符号解读 = " + (b & 0xFF));

        // ---------- 3. 减法变加法：100 - 56 == 100 + 200 (mod 256) ----------
        int sum = 100 + 200; // 300，超出 8 位
        System.out.println("100 + 200 = " + sum + "，溢出丢第 9 位后 = "
                + (sum & 0xFF) + "，与 100 - 56 = " + (100 - 56) + " 相等");

        // ---------- 4. 窄化转换：writeByte 只取低 8 位 ----------
        System.out.println("\nwriteByte(300): 300 = " + toBits32(300)
                + " → 截断低8位 = " + toBits8(300 & 0xFF) + " = " + (300 & 0xFF));
        System.out.println("writeByte(200): 200 = " + toBits32(200)
                + " → 截断低8位 = " + toBits8(200) + " = 有符号解读 " + (byte) 200);

        ByteArrayOutputStream buf = new ByteArrayOutputStream();
        DataOutputStream out = new DataOutputStream(buf);
        out.writeByte(300);
        out.writeByte(200);
        out.writeByte(5); // 高位本来就是 0，数值不变
        byte[] written = buf.toByteArray();
        System.out.println("落盘字节: " + hex(written)
                + " → 读回: " + written[0] + ", " + written[1] + ", " + written[2]);
    }

    private static String toBits8(int v) {
        return String.format("%8s", Integer.toBinaryString(v & 0xFF)).replace(' ', '0');
    }

    private static String toBits32(int v) {
        return String.format("%32s", Integer.toBinaryString(v)).replace(' ', '0');
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
