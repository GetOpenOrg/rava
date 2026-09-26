// FS-N6：Double.toString（静态 / 实例 / 拼接 / StringBuilder / println）走 DoubleToDecimal 字节码链。
public class TestDoubleToStringSpec {
    public static void main(String[] args) {
        double[] vs = {0.0, -0.0, 1.0, 0.1, 0.2 + 0.1, 1e7, 9999999.0, 1e-3, 1e-4, 123456789.123, 1.0 / 3,
            2.0 / 3, Math.PI, Math.E, 9007199254740993.0, 1e21, 1e22, 1e23, 5e-324, Double.MIN_NORMAL,
            Double.MAX_VALUE, Double.NaN, Double.POSITIVE_INFINITY, Double.NEGATIVE_INFINITY, 100.0, 1234.5e-10, -7.25};
        for (double v : vs) {
            Double boxed = v;
            StringBuilder sb = new StringBuilder().append(v);
            System.out.println(Double.toString(v) + " | " + boxed + " | " + sb + " | " + String.valueOf(v) + " | " + ("" + v));
        }
        System.out.println(1.5);
        Object o = 2.25;
        System.out.println(o);
        System.out.println(Float.toString(0.1f) + " " + (0.1f + 0.2f) + " " + Float.MIN_VALUE);
    }
}
