/**
 * Double.parseDouble / Float.parseFloat / valueOf 的语法与舍入边界
 * （Rosetta 语料 IntegerMethodsDemo 揭出：FloatingDecimal.parseDouble 为 panic 存根）。
 * 覆盖：首尾空白（trim 语义）、符号、NaN / Infinity、指数、前后缀小数点、f/d 后缀、
 * 上溢 / 下溢 / 次正规边界、十六进制浮点（含舍入与次正规）、float 直接舍入
 * （非经 double 二次舍入）、NumberFormatException 消息、null → NPE。
 */
public class TestParseDoubleEdge {
    static void d(String s) {
        try {
            double v = Double.parseDouble(s);
            System.out.println("d[" + s + "]=" + v + " bits=" + Long.toHexString(Double.doubleToRawLongBits(v)));
        } catch (NumberFormatException e) {
            System.out.println("d[" + s + "] NFE " + e.getMessage());
        }
    }

    static void f(String s) {
        try {
            float v = Float.parseFloat(s);
            System.out.println("f[" + s + "]=" + v + " bits=" + Integer.toHexString(Float.floatToRawIntBits(v)));
        } catch (NumberFormatException e) {
            System.out.println("f[" + s + "] NFE " + e.getMessage());
        }
    }

    public static void main(String[] args) {
        String[] cases = {
            "2.5", "  2.5  ", "\t-3.25\n", "-0", "+0.0", "1e10", "1E-5", ".5", "5.", "1.5f", "1.5D",
            "NaN", "-NaN", "+Infinity", "-Infinity", "infinity",
            "1e400", "-1e400", "1e-400", "4.9e-324", "2.4703282292062327e-324", "2.4703282292062328e-324",
            "1.7976931348623157e308", "1.7976931348623159e308", "0.1", "123456789012345678901234567890",
            "3.4028235e38", "3.4028236e38", "1.4e-45", "7.0e-46", "1.00000017881393432617187499",
            "0x1p0", "0x1.8p1", "-0x1.fffffffffffffp1023", "0x1.fffffffffffff8p1023", "0x1p-1074",
            "0x1p-1075", "0x1.8p-1075", "0x1.000001p0", "0x1.0000011p0", "0xAbC.dEfp-4f", "0x.8p1",
            "0xp1", "0x1", "1e", "1e+", "abc", ".", "+", "", "   ", "1.2.3", "1_000", "1.5x",
            "..", "1..", ".1.", "1e5.3", "-1.2.3", "0x1.2.3p1",
        };
        for (String s : cases) {
            d(s);
            f(s);
        }
        System.out.println("valueOf=" + Double.valueOf("  6.02e23 ") + " " + Float.valueOf("-1.25f"));
        try {
            Double.parseDouble(null);
        } catch (NullPointerException e) {
            System.out.println("null -> NPE");
        }
        double sum = 0;
        for (String s : new String[]{"0.1", "0.2", "0.3"}) sum += Double.parseDouble(s);
        System.out.println("sum=" + sum);
    }
}
