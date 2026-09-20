import java.math.BigInteger;

public class TestBigInteger {
    public static void main(String[] args) {
        BigInteger a = new BigInteger("123456789");
        BigInteger b = new BigInteger("987654321");
        System.out.println("add=" + a.add(b));
        System.out.println("sub=" + b.subtract(a));
        System.out.println("mul=" + a.multiply(b));
        System.out.println("mod=" + b.mod(a));
        System.out.println("pow=" + a.pow(3));
        System.out.println("gcd=" + a.gcd(b));
        System.out.println("isPrime97=" + BigInteger.valueOf(97).isProbablePrime(10));
        System.out.println("shiftL=" + a.shiftLeft(4));
        System.out.println("shiftR=" + a.shiftRight(4));
        System.out.println("and=" + a.and(b));
        System.out.println("or=" + a.or(b));
        System.out.println("xor=" + a.xor(b));
        System.out.println("not=" + a.not());
        System.out.println("abs=" + a.negate().abs());
        System.out.println("sign=" + a.signum());
        System.out.println("cmp=" + a.compareTo(b));
        System.out.println("valueOf=" + BigInteger.valueOf(1000000));
    }
}
