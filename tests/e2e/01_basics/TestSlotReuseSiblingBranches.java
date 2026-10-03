// 边界：if / else-if / else 兄弟分支各自声明同名变量（byte2 / cc），两段 LVT 落在不同槽，
// 外层变量 byte1 在第一个分支内被改写；整体包在 try-finally 里、循环内含多处 return
// （同 EUC_TW.Decoder.decodeArrayLoop / decodeBufferLoop）。兄弟分支的同名声明各自独立，不得互相提升或省略
public class TestSlotReuseSiblingBranches {
    static final int SS2 = 0x8e;
    static final int[] PLANE = new int[256];
    static {
        for (int i = 0; i < 256; i++) {
            PLANE[i] = (i >= 0xa1 && i <= 0xa7) ? i - 0xa1 : -1;
        }
    }

    static int srcPos;
    static int dstPos;

    static boolean isLegalDB(int b) {
        return b >= 0xa1 && b <= 0xfe;
    }

    static char[] toUnicode(int b1, int b2, int p) {
        if (!isLegalDB(b1) || !isLegalDB(b2)) {
            return null;
        }
        int c = 0x4e00 + p * 8836 + (b1 - 0xa1) * 94 + (b2 - 0xa1);
        if (p == 2) {
            return new char[] {(char) c, '+'};
        }
        return new char[] {(char) c};
    }

    static String decodeArrayLoop(byte[] sa, char[] da) {
        int sp = 0;
        int sl = sa.length;
        int dp = 0;
        int dl = da.length;
        try {
            while (sp < sl) {
                int byte1 = sa[sp] & 0xff;
                if (byte1 == SS2) {
                    if (sl - sp < 4)
                        return "UNDERFLOW";
                    int cnsPlane = PLANE[sa[sp + 1] & 0xff];
                    if (cnsPlane < 0)
                        return "MALFORMED[2]";
                    byte1 = sa[sp + 2] & 0xff;
                    int byte2 = sa[sp + 3] & 0xff;
                    char[] cc = toUnicode(byte1, byte2, cnsPlane);
                    if (cc == null) {
                        if (!isLegalDB(byte1) || !isLegalDB(byte2))
                            return "MALFORMED[4]";
                        return "UNMAPPABLE[4]";
                    }
                    if (dl - dp < cc.length)
                        return "OVERFLOW";
                    if (cc.length == 1) {
                        da[dp++] = cc[0];
                    } else {
                        da[dp++] = cc[0];
                        da[dp++] = cc[1];
                    }
                    sp += 4;
                } else if (byte1 < 0x80) {
                    if (dl - dp < 1)
                        return "OVERFLOW";
                    da[dp++] = (char) byte1;
                    sp++;
                } else {
                    if (sl - sp < 2)
                        return "UNDERFLOW";
                    int byte2 = sa[sp + 1] & 0xff;
                    char[] cc = toUnicode(byte1, byte2, 0);
                    if (cc == null) {
                        if (!isLegalDB(byte1) || !isLegalDB(byte2))
                            return "MALFORMED[1]";
                        return "UNMAPPABLE[2]";
                    }
                    if (dl - dp < 1)
                        return "OVERFLOW";
                    da[dp++] = cc[0];
                    sp += 2;
                }
            }
            return "UNDERFLOW";
        } finally {
            srcPos = sp;
            dstPos = dp;
        }
    }

    static String run(int[] in, int cap) {
        byte[] sa = new byte[in.length];
        for (int i = 0; i < in.length; i++) {
            sa[i] = (byte) in[i];
        }
        char[] da = new char[cap];
        String r = decodeArrayLoop(sa, da);
        StringBuilder sb = new StringBuilder();
        for (int i = 0; i < dstPos; i++) {
            sb.append(Integer.toHexString(da[i])).append(' ');
        }
        return r + " sp=" + srcPos + " dp=" + dstPos + " out=" + sb.toString().trim();
    }

    public static void main(String[] args) {
        System.out.println(run(new int[] {0x41, 0xa4, 0xa1, 0x42}, 8));
        System.out.println(run(new int[] {0x8e, 0xa3, 0xb0, 0xc0, 0x43}, 8));
        System.out.println(run(new int[] {0x8e, 0xa2, 0xa1, 0xa1}, 8));
        System.out.println(run(new int[] {0x8e, 0xa2, 0xa1, 0xa1}, 1));
        System.out.println(run(new int[] {0x8e, 0x20, 0xa1, 0xa1}, 8));
        System.out.println(run(new int[] {0x8e, 0xa1, 0x30, 0xa1}, 8));
        System.out.println(run(new int[] {0x41, 0xa4}, 8));
        System.out.println(run(new int[] {0xa4, 0x30}, 8));
        System.out.println(run(new int[] {0x8e, 0xa1}, 8));
        System.out.println(run(new int[] {0x8e, 0xa3, 0xb0, 0xc0}, 1));
    }
}
