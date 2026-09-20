public class TestFloatBits {
    public static void main(String[] args) {
        double d = Double.longBitsToDouble(0x3FF0000000000000L);
        System.out.println("doubleFromBits=" + d);
        System.out.println("bits=" + Double.doubleToLongBits(1.0));
        System.out.println("floatFromBits=" + Float.intBitsToFloat(0x3F800000));
        System.out.println("floatBits=" + Float.floatToIntBits(1.0f));
        System.out.println("isNaN=" + Double.isNaN(Double.NaN));
        System.out.println("isInf=" + Double.isInfinite(Double.POSITIVE_INFINITY));
        System.out.println("cmp=" + Double.compare(Double.NaN, 1.0));
        System.out.println("minNormal=" + Double.MIN_NORMAL);
        System.out.println("maxValue=" + Float.MAX_VALUE);
        System.out.println("compare=" + Double.compare(1.0, 2.0));
    }
}
