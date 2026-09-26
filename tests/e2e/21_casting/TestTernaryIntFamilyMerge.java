/**
 * 三元表达式两臂同属 JVM int 族但源类型不同（byte / short / char 局部 vs int 运算）：
 * 汇合值是 int，不得按首臂收窄（DataEncryptionStandard 的 `b >= 0 ? b : b + 256` 揭出）。
 * 覆盖：byte/int、short/int、char/int 两臂，结果流入 int 局部、byte/char 局部存储、
 * 方法实参（char 形参）、返回值、数组存储、字符串拼接。
 */
public class TestTernaryIntFamilyMerge {
    static int unsignedByte(byte b) { return (b >= 0) ? b : b + 256; }
    static int shortOffset(short s, boolean up) { return up ? s : s + 70000; }
    static char pick(char c, boolean upper) { return upper ? (char) (c - 32) : c; }
    static String show(char c) { return "[" + c + "]"; }

    public static void main(String[] args) {
        byte[] bytes = {0, 5, 127, -1, -128, -87};
        StringBuilder sb = new StringBuilder();
        for (byte b : bytes) {
            int bb = (b >= 0) ? b : b + 256;
            sb.append(Integer.toString(bb, 16)).append(' ');
        }
        System.out.println("hex " + sb);
        System.out.println("unsigned " + unsignedByte((byte) -2) + " " + unsignedByte((byte) 9));
        System.out.println("short " + shortOffset((short) -5, false) + " " + shortOffset((short) 12, true));

        char c = 'q';
        boolean flag = args.length == 0;
        char stored = flag ? (char) (c + 1) : c;
        byte narrowed = (byte) (flag ? c + 200 : c);
        System.out.println("stored " + stored + " narrowed " + narrowed);
        System.out.println("arg " + show(flag ? 'x' : c) + " ret " + pick('m', true) + pick('m', false));
        System.out.println("concat " + (flag ? c : 'z') + (flag ? 'A' : c));

        char[] cs = new char[2];
        cs[0] = flag ? c : 'b';
        cs[1] = flag ? (char) (c + 2) : 'b';
        int[] is = new int[1];
        byte bv = -3;
        is[0] = flag ? bv + 1000 : bv;
        System.out.println("arrays " + new String(cs) + " " + is[0]);
    }
}
