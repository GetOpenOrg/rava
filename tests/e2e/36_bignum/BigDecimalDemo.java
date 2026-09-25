import java.math.BigDecimal;
import java.math.RoundingMode;

public class BigDecimalDemo {
    public static void main(String[] args) {
        BigDecimal a = new BigDecimal("123456789.987654321");
        BigDecimal b = new BigDecimal("987654321.123456789");

        System.out.println("a = " + a);
        System.out.println("b = " + b);
        System.out.println("a + b = " + a.add(b));
        System.out.println("b - a = " + b.subtract(a));
        System.out.println("a * b = " + a.multiply(b));
        System.out.println("b / a (scale=6) = " + b.divide(a, 6, RoundingMode.HALF_UP));

        BigDecimal c = new BigDecimal("100");
        BigDecimal d = new BigDecimal("3");
        System.out.println("100 / 3 (scale=4) = " + c.divide(d, 4, RoundingMode.HALF_UP));

        System.out.println("a.negate() = " + a.negate());
        System.out.println("a.abs() = " + a.negate().abs());

        System.out.println("compareTo(a,b) = " + a.compareTo(b));
        System.out.println("min(a,b) = " + a.min(b));
        System.out.println("max(a,b) = " + a.max(b));

        BigDecimal pi = new BigDecimal("3.14159265358979");
        System.out.println("pi.scale() = " + pi.scale());
        System.out.println("pi.precision() = " + pi.precision());
        System.out.println("pi.intValue() = " + pi.intValue());

        BigDecimal val = BigDecimal.valueOf(42L);
        System.out.println("valueOf(42) = " + val);
    }
}
