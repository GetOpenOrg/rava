import java.math.BigInteger;
import java.math.BigDecimal;

public class BigNumberTest {
    public static void main(String[] args) {
        // BigInteger
        BigInteger a = new BigInteger("123456789012345678901234567890");
        BigInteger b = BigInteger.valueOf(42);
        System.out.println("a: " + a);
        System.out.println("b: " + b);
        System.out.println("a+b: " + a.add(b));
        System.out.println("a-b: " + a.subtract(b));
        System.out.println("a*b: " + a.multiply(b));
        System.out.println("a/b: " + a.divide(b));
        System.out.println("a%b: " + a.remainder(b));

        BigInteger c = BigInteger.valueOf(100);
        BigInteger d = BigInteger.valueOf(-50);
        System.out.println("negate: " + c.negate());
        System.out.println("abs: " + d.abs());
        System.out.println("signum(100): " + c.signum());
        System.out.println("signum(-50): " + d.signum());
        System.out.println("compareTo: " + c.compareTo(d));

        // Bit operations
        BigInteger x = BigInteger.valueOf(0xFF);
        BigInteger y = BigInteger.valueOf(0x0F);
        System.out.println("and: " + x.and(y));
        System.out.println("or: " + x.or(y));
        System.out.println("xor: " + x.xor(y));

        // pow / gcd
        BigInteger base = BigInteger.valueOf(2);
        System.out.println("2^10: " + base.pow(10));
        System.out.println("gcd(12,8): " + BigInteger.valueOf(12).gcd(BigInteger.valueOf(8)));

        // Conversion
        BigInteger big = BigInteger.valueOf(999);
        System.out.println("intValue: " + big.intValue());
        System.out.println("longValue: " + big.longValue());

        // BigDecimal
        BigDecimal bd1 = new BigDecimal("123.456");
        BigDecimal bd2 = new BigDecimal("0.544");
        System.out.println("bd1: " + bd1);
        System.out.println("bd2: " + bd2);
        System.out.println("bd1+bd2: " + bd1.add(bd2));
        System.out.println("bd1-bd2: " + bd1.subtract(bd2));

        BigDecimal bd3 = new BigDecimal("10");
        BigDecimal bd4 = new BigDecimal("3");
        System.out.println("10*3: " + bd3.multiply(bd4));
        System.out.println("compareTo: " + bd1.compareTo(bd2));

        BigDecimal bd5 = BigDecimal.valueOf(42);
        System.out.println("valueOf(42): " + bd5);
        System.out.println("intValue: " + bd5.intValue());
    }
}
