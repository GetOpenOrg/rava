public class TestDouble {
    public static void main(String[] args) {
        double a = 3.14;
        double b = 2.0;
        System.out.println(a + b);

        double x = 10.0;
        double y = 3.0;
        System.out.println(x / y);

        double pi = Math.PI;
        System.out.println((int)(pi * 100) / 100.0);

        float f = 1.5f;
        double d = f;
        System.out.println(d);

        System.out.println(Double.MAX_VALUE > 1e300);
    }
}
