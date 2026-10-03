/**
 * Character 控制符/数字/码点构造族（方法级实测：isISOControl 12 jar / appendCodePoint 11 /
 * forDigit 7 / isSpaceChar 5 / toCodePoint 5，此前零覆盖）：C0/C1 控制区、
 * 代理对拼装、isSpaceChar 与 isWhitespace 的 NBSP 分野。
 */
public class TestCharControlDigits {

    public static void main(String[] args) {
        System.out.println("c0=" + Character.isISOControl('\u0000')
                + Character.isISOControl('\t')
                + Character.isISOControl('\u001F'));
        System.out.println("del=" + Character.isISOControl('\u007F'));
        System.out.println("c1=" + Character.isISOControl('\u0085'));
        System.out.println("not-ctl=" + Character.isISOControl(' ')
                + Character.isISOControl('A') + Character.isISOControl('\u00A0'));

        // appendCodePoint：BMP 一单元、增补两单元
        StringBuilder sb = new StringBuilder();
        sb.appendCodePoint('A').appendCodePoint(0x1D11E);
        System.out.println("len=" + sb.length() + " charAt1-hi="
                + Integer.toHexString(sb.charAt(1)));

        // forDigit：进制数字；越界 radix 返回 NUL
        System.out.println("digit-f=" + Character.forDigit(15, 16)
                + " digit-0=" + Character.forDigit(0, 10));
        System.out.println("digit-oob=" + (Character.forDigit(16, 16) == '\0')
                + " radix-oob=" + (Character.forDigit(1, 1) == '\0'));

        // toCodePoint：高低代理拼装回码点
        char hi = sb.charAt(1);
        char lo = sb.charAt(2);
        System.out.println("to-cp=" + Integer.toHexString(Character.toCodePoint(hi, lo)));

        // isSpaceChar 与 isWhitespace 的分野：NBSP 是 space 但非 whitespace
        System.out.println("nbsp-space=" + Character.isSpaceChar('\u00A0')
                + " nbsp-ws=" + Character.isWhitespace('\u00A0'));
        System.out.println("sp-both=" + Character.isSpaceChar(' ')
                + Character.isWhitespace(' '));
    }
}
