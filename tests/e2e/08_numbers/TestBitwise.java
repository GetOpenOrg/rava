public class TestBitwise {
    public static void main(String[] args) {
        int a = 0b1010;
        int b = 0b1100;
        System.out.println(a & b);
        System.out.println(a | b);
        System.out.println(a ^ b);
        System.out.println(~a & 0xFF);

        int x = 8;
        System.out.println(x >> 1);
        System.out.println(x << 2);

        int neg = -8;
        System.out.println(neg >> 1);
        System.out.println(neg >>> 1);
    }
}
