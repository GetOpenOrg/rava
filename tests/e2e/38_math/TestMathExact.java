public class TestMathExact {
    public static void main(String[] args) {
        System.out.println("addExact=" + Math.addExact(1000000000, 1000000000));
        System.out.println("mulExact=" + Math.multiplyExact(1000, 1000));
        System.out.println("subExact=" + Math.subtractExact(10, 3));
        System.out.println("negateExact=" + Math.negateExact(-5));
        System.out.println("toIntExact=" + Math.toIntExact(2147483647L));
        System.out.println("floorDiv=" + Math.floorDiv(-7, 3) + "," + Math.floorDiv(7, 3));
        System.out.println("floorMod=" + Math.floorMod(-7, 3) + "," + Math.floorMod(7, 3));
        System.out.println("copySign=" + Math.copySign(1.0, -2.0));
        System.out.println("nextAfter=" + Math.nextAfter(1.0, 2.0));
        System.out.println("nextUp=" + Math.nextUp(1.0));
        System.out.println("nextDown=" + Math.nextDown(1.0));
        System.out.println("hypot=" + Math.hypot(3, 4));
        System.out.println("atan2=" + Math.atan2(1, 1));
        System.out.println("expm1=" + Math.expm1(1.0));
        System.out.println("log1p=" + Math.log1p(1.0));
        System.out.println("ulp=" + Math.ulp(1.0));
        try {
            Math.addExact(Integer.MAX_VALUE, 1);
            System.out.println("overflow not thrown");
        } catch (ArithmeticException e) {
            System.out.println("overflow=" + e.getClass().getSimpleName());
        }
    }
}
