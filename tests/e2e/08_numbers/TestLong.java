public class TestLong {
    public static void main(String[] args) {
        long a = 1000000000L;
        long b = 2000000000L;
        long sum = a + b;
        System.out.println(sum);

        long max = Long.MAX_VALUE;
        System.out.println(max);

        long x = 100L;
        long y = 7L;
        System.out.println(x / y);
        System.out.println(x % y);

        long shifted = 1L << 40;
        System.out.println(shifted);
    }
}
