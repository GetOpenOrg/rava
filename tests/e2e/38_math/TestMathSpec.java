/** Math 规格：random 分布性质、IEEEremainder（rint 语义）、pow 的 NaN/无穷规则、FdLibm 逐位结果、
 *  round / nextUp / nextDown / nextAfter / copySign / signum / scalb / toRadians 边界。 */
public class TestMathSpec {
    static String b(double d) { return Double.toString(d) + "/" + Long.toHexString(Double.doubleToRawLongBits(d)); }

    public static void main(String[] args) {
        // random：[0,1)、非恒定
        double r0 = Math.random();
        boolean inRange = true, varied = false;
        for (int i = 0; i < 200; i++) {
            double r = Math.random();
            if (!(r >= 0.0 && r < 1.0)) inRange = false;
            if (r != r0) varied = true;
        }
        System.out.println("random inRange=" + inRange + " varied=" + varied);

        // IEEEremainder：商取最接近整数（偶数优先）
        System.out.println("rem " + Math.IEEEremainder(5, 2) + " " + Math.IEEEremainder(7, 2) + " " + Math.IEEEremainder(-5, 2)
                + " " + Math.IEEEremainder(5.5, 2) + " " + Math.IEEEremainder(1, 0) + " " + Math.IEEEremainder(3, Double.POSITIVE_INFINITY));

        // pow 特殊规则
        System.out.println("pow " + Math.pow(1, Double.NaN) + " " + Math.pow(-1, Double.POSITIVE_INFINITY) + " " + Math.pow(Double.NaN, 0)
                + " " + Math.pow(-8, 1.0 / 3) + " " + Math.pow(-2, 3) + " " + Math.pow(0.0, -1) + " " + Math.pow(-0.0, -1) + " " + Math.pow(2, 0.5));

        // FdLibm 逐位（Java 规格：StrictMath 结果 = fdlibm）
        double[] xs = {0.5, 1.0, 2.0, 10.0, 1e-5, 123.456, -0.75};
        for (double x : xs) {
            System.out.println("x=" + x + " sin=" + b(StrictMath.sin(x)) + " cos=" + b(StrictMath.cos(x)) + " tan=" + b(StrictMath.tan(x))
                    + " exp=" + b(StrictMath.exp(x)) + " cbrt=" + b(StrictMath.cbrt(x)) + " sinh=" + b(StrictMath.sinh(x))
                    + " atan=" + b(StrictMath.atan(x)) + " log1p=" + b(StrictMath.log1p(Math.abs(x))));
        }
        System.out.println("log " + b(StrictMath.log(10)) + " log10 " + b(StrictMath.log10(2)) + " hypot " + b(StrictMath.hypot(3e200, 4e200))
                + " atan2 " + b(StrictMath.atan2(1, -1)) + " asin " + b(StrictMath.asin(0.3)) + " acos " + b(StrictMath.acos(-0.3))
                + " tanh " + b(StrictMath.tanh(0.7)) + " cosh " + b(StrictMath.cosh(2.5)) + " sqrt " + b(Math.sqrt(2)));

        // round（半值向正无穷）、ceil/floor 的 -0.0
        System.out.println("round " + Math.round(-2.5) + " " + Math.round(2.5) + " " + Math.round(-0.5f) + " " + Math.round(0.49999999999999994)
                + " " + Math.round(Double.NaN) + " " + Math.round(1e20) + " ceil " + Math.ceil(-0.5) + " floor " + Math.floor(-0.0));

        // nextUp / nextDown / nextAfter / copySign / signum / scalb / ulp
        System.out.println("next " + Math.nextUp(-1.0) + " " + Math.nextDown(1.0) + " " + Math.nextUp(-0.0) + " " + Math.nextAfter(0.0, -1)
                + " " + Math.nextUp(Double.MAX_VALUE) + " " + Math.nextUp(1.0f) + " " + Math.nextDown(-Float.MIN_VALUE));
        System.out.println("misc " + Math.copySign(3.0, -0.0) + " " + Math.signum(-0.0) + " " + Math.signum(-7.5f) + " " + Math.scalb(1.5, 1030)
                + " " + Math.scalb(3.0, -1080) + " " + Math.ulp(1.0) + " " + Math.toRadians(180) + " " + Math.toDegrees(Math.PI / 3));
        System.out.println("fma " + Math.fma(0.1, 10, -1) + " abs " + Math.abs(-0.0) + " " + Math.abs(Integer.MIN_VALUE)
                + " getExponent " + Math.getExponent(1024.0) + " " + Math.getExponent(Double.MIN_VALUE));
    }
}
