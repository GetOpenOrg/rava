public class TestOverflow {

    public static void main(String[] args) {
        // int 环绕
        int maxInt = Integer.MAX_VALUE;
        System.out.println("maxInt=" + maxInt);
        System.out.println("maxInt+1=" + (maxInt + 1));
        System.out.println("minInt=" + Integer.MIN_VALUE);
        System.out.println("-(minInt)=" + (-Integer.MIN_VALUE));

        // long 环绕
        long maxLong = Long.MAX_VALUE;
        System.out.println("maxLong=" + maxLong);
        System.out.println("maxLong+1=" + (maxLong + 1));
        System.out.println("minLong=" + Long.MIN_VALUE);
        System.out.println("-(minLong)=" + (-Long.MIN_VALUE));

        // 乘法溢出
        System.out.println("65536*65536=" + (65536 * 65536));
        System.out.println("big*3L=" + (maxLong / 2 * 3L));

        // 整数除法：向零取整 + 负数取模符号
        System.out.println("-7/2=" + (-7 / 2));
        System.out.println("7/-2=" + (7 / -2));
        System.out.println("-7%2=" + (-7 % 2));
        System.out.println("7%-2=" + (7 % -2));

        // 除零：整数抛 ArithmeticException，浮点不抛
        try {
            int z = 10 / 0;
            System.out.println("no throw " + z);
        } catch (ArithmeticException e) {
            System.out.println("int div0: " + e.getClass().getSimpleName());
        }
        try {
            int z = 10 % 0;
            System.out.println("no throw " + z);
        } catch (ArithmeticException e) {
            System.out.println("int mod0: " + e.getClass().getSimpleName());
        }
        System.out.println("double div0=" + (10.0 / 0.0));
        System.out.println("double negdiv0=" + (-10.0 / 0.0));
        System.out.println("double 0/0=" + (0.0 / 0.0));

        // 短类型累加溢出
        byte b = 100;
        b = (byte) (b + 100);
        System.out.println("byte=" + b);
        short s = 30000;
        s = (short) (s + 30000);
        System.out.println("short=" + s);

        System.out.println("done");
    }
}
