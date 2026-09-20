public class TestNumericCast {

    public static void main(String[] args) {
        int i = 200;

        // 窄化：高位截断
        byte b = (byte) i;
        short s = (short) 40000;
        System.out.println("int 200 -> byte=" + b);
        System.out.println("int 40000 -> short=" + s);
        System.out.println("int 70000 -> char=" + (int) (char) 70000);

        // char <-> int
        char c = 'A';
        System.out.println("char A -> int=" + (int) c);
        System.out.println("int 66 -> char=" + (char) 66);
        System.out.println("65 -> char=" + (char) 65);

        // char 算术提升
        char c2 = (char) (c + 1);
        System.out.println("A+1=" + c2);
        System.out.println("char compare=" + (c < 'B'));

        // byte / short 算术提升到 int
        byte b1 = 10, b2 = 20;
        byte b3 = (byte) (b1 + b2);
        System.out.println("byte add=" + b3);
        short sh1 = 1000, sh2 = 2000;
        short sh3 = (short) (sh1 + sh2);
        System.out.println("short add=" + sh3);

        // int -> long
        long l = i;
        System.out.println("int->long=" + l);
        System.out.println("long 5000000000 -> int=" + (int) 5000000000L);

        // 浮点 <-> 整型
        System.out.println("int->double=" + (double) 7);
        System.out.println("int->float=" + (float) 7);
        System.out.println("double 3.9 -> int=" + (int) 3.9);
        System.out.println("double -3.9 -> int=" + (int) (-3.9));
        System.out.println("float 2.5f -> int=" + (int) 2.5f);
        System.out.println("double 1e20 -> long=" + (long) 1e20);
        System.out.println("int->float precision=" + (float) 16777217);

        // 特殊浮点转换
        double nan = Double.NaN;
        double posInf = Double.POSITIVE_INFINITY;
        double negInf = Double.NEGATIVE_INFINITY;
        System.out.println("NaN->int=" + (int) nan);
        System.out.println("+Inf->int=" + (int) posInf);
        System.out.println("-Inf->int=" + (int) negInf);
        System.out.println("NaN->long=" + (long) nan);
        System.out.println("Inf->long=" + (long) posInf);

        // 混合运算中的隐式提升
        byte bb = 5;
        short ss = 6;
        char cc = 7;
        int ii = 8;
        long ll = 9L;
        float ff = 1.5f;
        double dd = 2.5;
        System.out.println("byte+short=" + (bb + ss));
        System.out.println("char+int=" + (cc + ii));
        System.out.println("int+long=" + (ii + ll));
        System.out.println("long+float=" + (ll + ff));
        System.out.println("float+double=" + (ff + dd));
        System.out.println("byte+char+short+int+long+float+double=" + (bb + cc + ss + ii + ll + ff + dd));

        // 整数除法 vs 浮点除法
        System.out.println("7/2=" + (7 / 2));
        System.out.println("7/2.0=" + (7 / 2.0));
        System.out.println("7/2f=" + (7 / 2f));

        // 强制转换后再赋值的复合赋值
        int roundTrip = (int) (byte) (short) (long) 123L;
        System.out.println("roundTrip=" + roundTrip);
        double d2 = (int) 5.99;
        System.out.println("double from casted int=" + d2);

        // 取反与转换
        System.out.println("-(byte)5=" + (-(byte) 5));
        System.out.println("(int)(char)(byte)-1=" + (int) (char) (byte) -1);

        System.out.println("done");
    }
}
