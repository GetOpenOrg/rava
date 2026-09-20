public class TestNaN {
    public static void main(String[] args) {
        double nan = 0.0 / 0.0;
        System.out.println("nanEq=" + (nan == nan));
        System.out.println("nanPlus=" + (nan + 1.0));
        System.out.println("inf=" + (5.0 / 0.0));
        System.out.println("infSub=" + (Double.POSITIVE_INFINITY - Double.POSITIVE_INFINITY));
        System.out.println("negZeroEq=" + (0.0 == -0.0));
        System.out.println("negZeroDiv=" + (1.0 / -0.0));
        System.out.println("str=" + nan);
        System.out.println("minNormal=" + Double.MIN_NORMAL);
        System.out.println("isNaN=" + Double.isNaN(nan));
        System.out.println("infAdd=" + (Double.POSITIVE_INFINITY + 1.0));
    }
}
