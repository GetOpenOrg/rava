public class TestUnsignedInt {
    public static void main(String[] args) {
        int neg = -1;
        System.out.println("toULong=" + Integer.toUnsignedLong(neg));
        System.out.println("divU=" + Integer.divideUnsigned(neg, 2));
        System.out.println("remU=" + Integer.remainderUnsigned(neg, 10));
        System.out.println("cmpU=" + Integer.compareUnsigned(neg, 1));
        System.out.println("parseU=" + Integer.parseUnsignedInt("4294967295"));
        System.out.println("formatU=" + Integer.toUnsignedString(neg));
        System.out.println("shiftRU=" + (neg >>> 16));
        System.out.println("parseU2=" + Integer.parseUnsignedInt("128"));
        System.out.println("divU2=" + Integer.divideUnsigned(20, 3));
        System.out.println("cmpU2=" + Integer.compareUnsigned(200, 100));
    }
}
