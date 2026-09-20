public class TestDivMod {
    public static void main(String[] args) {
        System.out.println("idiv=" + (7 / 2) + "," + (-7 / 2) + "," + (7 / -2));
        System.out.println("imod=" + (7 % 3) + "," + (-7 % 3) + "," + (7 % -3));
        System.out.println("ddiv=" + (5.0 / 0.0));
        System.out.println("dmod=" + (5.0 % 0.0));
        try {
            int x = 5 / 0;
            System.out.println(x);
        } catch (ArithmeticException e) {
            System.out.println("intDivZero=" + e.getClass().getSimpleName());
        }
        try {
            int y = 5 % 0;
            System.out.println(y);
        } catch (ArithmeticException e) {
            System.out.println("intModZero=" + e.getClass().getSimpleName());
        }
        System.out.println("longMod=" + (Long.MIN_VALUE % -1));
        System.out.println("dblModSign=" + (5.0 % -3.0));
    }
}
