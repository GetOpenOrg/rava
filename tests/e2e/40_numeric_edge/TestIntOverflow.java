public class TestIntOverflow {
    public static void main(String[] args) {
        System.out.println("max+1=" + (Integer.MAX_VALUE + 1));
        System.out.println("min-1=" + (Integer.MIN_VALUE - 1));
        System.out.println("absMin=" + Math.abs(Integer.MIN_VALUE));
        System.out.println("negMin=" + -Integer.MIN_VALUE);
        System.out.println("longMinDiv=" + (Long.MIN_VALUE / -1));
        System.out.println("shift31=" + (1 << 31));
        System.out.println("shift32=" + (1 << 32));
        System.out.println("ushiftNeg=" + (-1 >>> 1));
        System.out.println("mulWrap=" + (Integer.MAX_VALUE * 2));
    }
}
