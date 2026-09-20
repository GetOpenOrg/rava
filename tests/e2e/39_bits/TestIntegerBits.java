public class TestIntegerBits {
    public static void main(String[] args) {
        int v = 0x12345678;
        System.out.println("rotL=" + Integer.rotateLeft(v, 8));
        System.out.println("rotR=" + Integer.rotateRight(v, 8));
        System.out.println("bitCount=" + Integer.bitCount(0xFF00FF));
        System.out.println("lead0=" + Integer.numberOfLeadingZeros(1));
        System.out.println("trail0=" + Integer.numberOfTrailingZeros(8));
        System.out.println("hob=" + Integer.highestOneBit(0x123));
        System.out.println("lob=" + Integer.lowestOneBit(0x120));
        System.out.println("rev=" + Integer.reverse(0x1));
        System.out.println("revBytes=" + Integer.reverseBytes(v));
        System.out.println("signum=" + Integer.signum(-5) + "," + Integer.signum(0) + "," + Integer.signum(5));
        System.out.println("bin=" + Integer.toBinaryString(5));
        System.out.println("hex=" + Integer.toHexString(255));
        System.out.println("hob0=" + Integer.highestOneBit(0));
    }
}
