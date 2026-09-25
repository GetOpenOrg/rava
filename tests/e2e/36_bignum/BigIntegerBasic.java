import java.math.BigInteger;

public class BigIntegerBasic {
    public static void main(String[] args) {
        BigInteger a = BigInteger.valueOf(1000000000L);
        BigInteger b = BigInteger.valueOf(999999999L);

        System.out.println(a.add(b));
        System.out.println(a.subtract(b));
        System.out.println(a.multiply(BigInteger.valueOf(2)));
        System.out.println(a.divide(BigInteger.valueOf(3)));
        System.out.println(a.mod(BigInteger.valueOf(7)));

        // pow / gcd
        System.out.println(BigInteger.valueOf(2).pow(10));
        System.out.println(BigInteger.valueOf(12).gcd(BigInteger.valueOf(8)));

        // abs / negate
        System.out.println(BigInteger.valueOf(-42).abs());
        System.out.println(BigInteger.valueOf(5).negate());

        // compareTo
        System.out.println(a.compareTo(b) > 0);
        System.out.println(a.compareTo(a) == 0);

        // bitwise
        BigInteger x = BigInteger.valueOf(0b1010);
        BigInteger y = BigInteger.valueOf(0b1100);
        System.out.println(x.and(y));
        System.out.println(x.or(y));
        System.out.println(x.xor(y));

        // shift
        System.out.println(BigInteger.ONE.shiftLeft(8));
        System.out.println(BigInteger.valueOf(256).shiftRight(3));

        // bitLength / isProbablePrime
        System.out.println(BigInteger.valueOf(127).bitLength());
        System.out.println(BigInteger.valueOf(7).isProbablePrime(10));
        System.out.println(BigInteger.valueOf(4).isProbablePrime(10));

        // intValue / longValue
        System.out.println(BigInteger.valueOf(42).intValue());
        System.out.println(BigInteger.valueOf(999).longValue());

        System.out.println("done");
    }
}
