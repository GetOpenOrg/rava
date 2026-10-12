
/**
 * PART 04（下）：符号扩展、零扩展与 & 0xFF 掩码。
 * - 规则：有符号类型转宽类型做符号扩展（高位填符号位），char 做零扩展（高位填 0）
 * - Multicast 谜题：(int)(char)(byte)-1 == 65535
 * - BigDelight 谜题：b == 0x90 永远不成立，需用 (b & 0xFF) == 0x90 抑制符号扩展
 */
public class SignExtensionMaskDemo {

    public static void main(String[] args) {
        // ---------- 1. Multicast 谜题 ----------
        int result = (int) (char) (byte) -1;
        System.out.println("(int)(char)(byte)-1 = " + result); // 65535，不是 -1
        System.out.println("byte -1 → char：byte 有符号 → 符号扩展成 16 个 1 = " + (int) (char) (byte) -1);
        System.out.println("char → int：char 无符号 → 零扩展，高位补 0，仍是 " + (int) (char) (byte) -1);

        // ---------- 2. 符号扩展 vs & 0xFF 零扩展 ----------
        byte negative = (byte) 200; // 位模式 1100 1000
        System.out.println("\n(byte)200 直接提升为 int（符号扩展）: " + negative);              // -56
        System.out.println("(byte)200 & 0xFF（掩码抑制符号扩展）: " + (negative & 0xFF));        // 200
        System.out.println("-56 的 32 位补码: "
                + String.format("%32s", Integer.toBinaryString(negative)).replace(' ', '0'));
        System.out.println("200 的 32 位表示: "
                + String.format("%32s", Integer.toBinaryString(negative & 0xFF)).replace(' ', '0'));

        // ---------- 3. BigDelight 谜题：byte b == 0x90 ----------
        System.out.println("\n0x90 = " + 0x90 + "（超出 byte 范围的 int 常量）");
        int wrongHits = 0;
        int rightHits = 0;
        for (byte b = Byte.MIN_VALUE; b < Byte.MAX_VALUE; b++) {
            if (b == 0x90) {            // 混合类型比较：b 符号扩展成 -112，永远不等于 144
                wrongHits++;
            }
            if ((b & 0xFF) == 0x90) {   // 苹果比苹果：掩码把 b 变成无符号 int 再比较
                rightHits++;
            }
        }
        System.out.println("b == 0x90     命中次数: " + wrongHits + "（什么都找不到）");
        System.out.println("(b & 0xFF) == 0x90 命中次数: " + rightHits);
        System.out.println("((byte)0x90 == 0x90) = " + ((byte) 0x90 == 0x90)
                + "，因为 (byte)0x90 符号扩展后是 " + ((byte) 0x90));
    }
}
