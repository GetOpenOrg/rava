public class MathEnhancedTest {
    public static void main(String[] args) {
        // 三角反函数 (use values with exact results)
        System.out.println("asin(0.0): " + Math.asin(0.0));
        System.out.println("acos(1.0): " + Math.acos(1.0));
        System.out.println("atan(0.0): " + Math.atan(0.0));
        System.out.println("atan2(0.0, 1.0): " + Math.atan2(0.0, 1.0));

        // 指数/对数
        System.out.println("log10(100.0): " + Math.log10(100.0));
        System.out.println("cbrt(27.0): " + Math.cbrt(27.0));
        System.out.println("cbrt(8.0): " + Math.cbrt(8.0));

        // 其他
        System.out.println("hypot(3.0, 4.0): " + Math.hypot(3.0, 4.0));

        // 符号
        System.out.println("signum(-5.0): " + Math.signum(-5.0));
        System.out.println("signum(0.0): " + Math.signum(0.0));
        System.out.println("signum(5.0): " + Math.signum(5.0));

        // 精确运算
        System.out.println("addExact(5, 3): " + Math.addExact(5, 3));
        System.out.println("subtractExact(10, 3): " + Math.subtractExact(10, 3));
        System.out.println("multiplyExact(4, 5): " + Math.multiplyExact(4, 5));

        // 整除
        System.out.println("floorDiv(7, 2): " + Math.floorDiv(7, 2));
        System.out.println("floorDiv(-7, 2): " + Math.floorDiv(-7, 2));
        System.out.println("floorMod(7, 2): " + Math.floorMod(7, 2));
        System.out.println("floorMod(-7, 2): " + Math.floorMod(-7, 2));

        // toRadians / toDegrees with integer-friendly values
        System.out.println("toDegrees(0.0): " + Math.toDegrees(0.0));
        System.out.println("toRadians(0.0): " + Math.toRadians(0.0));
    }
}
