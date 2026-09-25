/**
 * StringBuilder / StringBuffer 追加浮点数（JDK 25：ASB.append(double|float) 经
 * DoubleToDecimal / FloatToDecimal 的 LATIN1 / UTF16 单例 putDecimal 直写缓冲）。
 * 分支：Latin1 与 UTF16 两种 builder 编码 × double / float × 常规值、整数值、
 * 大/小指数（科学计数）、负数、特殊值（NaN / ±Infinity / ±0.0）、极值；
 * StringBuffer、insert、String.valueOf、连续追加与扩容。
 */
public class TestAppendDecimal {
    static final double[] DS = {0.1, 1.0, -2.5, 3.141592653589793, 100.0, 1e7, 1.0E-3,
            1.0E-5, 1.2345678E20, -9.87e-12, Double.MAX_VALUE, Double.MIN_VALUE,
            Double.MIN_NORMAL, Double.NaN, Double.POSITIVE_INFINITY,
            Double.NEGATIVE_INFINITY, 0.0, -0.0};
    static final float[] FS = {0.1f, 1.0f, -2.5f, 3.1415927f, 1e7f, 1.0E-3f, 1.0E-5f,
            3.4e38f, Float.MIN_VALUE, Float.NaN, Float.POSITIVE_INFINITY,
            Float.NEGATIVE_INFINITY, 0.0f, -0.0f};

    public static void main(String[] args) {
        // ── Latin1 builder ─────────────────────────────────────────────
        StringBuilder sb = new StringBuilder();
        for (double d : DS) sb.append(d).append('|');
        System.out.println("latin1.double=" + sb);
        sb.setLength(0);
        for (float f : FS) sb.append(f).append('|');
        System.out.println("latin1.float=" + sb);

        // ── UTF16 builder（先放入非 Latin1 字符，coder 切到 UTF16）────────
        StringBuilder u = new StringBuilder("值:");
        for (double d : DS) u.append(d).append('|');
        System.out.println("utf16.double=" + u);
        System.out.println("utf16.len=" + u.length() + ",charAt3=" + u.charAt(3));
        StringBuilder uf = new StringBuilder("浮");
        for (float f : FS) uf.append(f).append(',');
        System.out.println("utf16.float=" + uf);
        // UTF16 下追加后继续追加中文，字符序列不错位
        u.setLength(0);
        u.append("前").append(2.5).append("后").append(-0.0f).append("尾");
        System.out.println("utf16.mixed=" + u + "," + u.indexOf("后"));

        // ── 扩容：小初始容量下连续追加 ─────────────────────────────────
        StringBuilder small = new StringBuilder(1);
        for (int i = 0; i < 20; i++) small.append(i * 0.5);
        System.out.println("grow=" + small.length() + "," + small);

        // ── StringBuffer / insert / valueOf ────────────────────────────
        StringBuffer buf = new StringBuffer("sb:");
        buf.append(6.02214076e23).append(' ').append(1.5f);
        System.out.println("buffer=" + buf);
        StringBuilder ins = new StringBuilder("[]");
        ins.insert(1, 42.0).insert(1, 0.25f);
        System.out.println("insert=" + ins);
        System.out.println("valueOf=" + String.valueOf(1e-7) + "," + String.valueOf(123456.789f));
        System.out.println("concat=" + ("x" + 0.3 + "y" + 2.0f));
    }
}
