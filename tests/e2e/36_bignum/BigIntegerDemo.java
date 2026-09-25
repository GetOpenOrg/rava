import java.math.BigInteger;

public class BigIntegerDemo {
    public static void main(String[] args) {
        BigInteger a = new BigInteger("12345678901234567890");
        BigInteger b = new BigInteger("98765432109876543210");

        System.out.println("a = " + a);
        System.out.println("b = " + b);
        System.out.println("a + b = " + a.add(b));
        System.out.println("b - a = " + b.subtract(a));
        System.out.println("a * 2 = " + a.multiply(BigInteger.TWO));
        System.out.println("b / a = " + b.divide(a));
        System.out.println("b mod a = " + b.mod(a));
        System.out.println("gcd(a,b) = " + a.gcd(b));
        System.out.println("a.pow(2) = " + a.pow(2));
        System.out.println("a.negate() = " + a.negate());
        System.out.println("a.abs() = " + a.negate().abs());

        BigInteger p = new BigInteger("104723");
        System.out.println("104723 isProbablePrime(10) = " + p.isProbablePrime(10));

        BigInteger x = BigInteger.valueOf(255);
        System.out.println("255 in binary: " + x.toString(2));
        System.out.println("255 in hex: " + x.toString(16));
        System.out.println("bitLength: " + x.bitLength());
        System.out.println("bitCount: " + x.bitCount());
    }
}
