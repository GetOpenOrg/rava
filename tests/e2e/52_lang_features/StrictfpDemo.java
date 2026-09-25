public class StrictfpDemo {
    // strictfp method (Java 17+ equivalent to normal method semantics)
    static strictfp double compute(double a, double b) {
        return a * b + Math.PI * a - b / 3.0;
    }

    static double chainOps(double x) {
        // multi-step floating-point chain
        x = x * 1.5 + 2.7;
        x = Math.sqrt(x);
        x = x * x - 1.0;
        return x;
    }

    public static void main(String[] args) {
        // basic strictfp computation
        double r1 = compute(3.0, 4.0);
        System.out.println(String.format("%.6f", r1));

        // chained float ops
        double r2 = chainOps(2.0);
        System.out.println(String.format("%.6f", r2));

        // special values
        System.out.println(Double.isNaN(0.0 / 0.0));
        System.out.println(Double.isInfinite(1.0 / 0.0));
        System.out.println(Double.isInfinite(-1.0 / 0.0));

        // boundary constants
        System.out.println(Double.MAX_VALUE > 1e300);
        System.out.println(Double.MIN_VALUE > 0.0);

        // float precision
        float f1 = 1.0f / 3.0f;
        float f2 = f1 * 3.0f;
        System.out.println(String.format("%.1f", (double) f2));
    }
}
