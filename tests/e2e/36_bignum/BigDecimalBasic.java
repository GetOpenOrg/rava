import java.math.BigDecimal;
import java.math.RoundingMode;

public class BigDecimalBasic {
    public static void main(String[] args) {
        BigDecimal a = new BigDecimal("10.50");
        BigDecimal b = new BigDecimal("3.20");

        System.out.println(a.add(b));
        System.out.println(a.subtract(b));
        System.out.println(a.multiply(b));

        // divide with explicit scale to avoid ArithmeticException
        System.out.println(a.divide(b, 2, RoundingMode.HALF_UP));

        // setScale
        System.out.println(new BigDecimal("3.14159").setScale(2, RoundingMode.HALF_UP));
        System.out.println(new BigDecimal("2.5").setScale(0, RoundingMode.HALF_UP));

        // compareTo
        System.out.println(a.compareTo(b) > 0);
        System.out.println(b.compareTo(a) < 0);
        System.out.println(a.compareTo(new BigDecimal("10.50")) == 0);

        // intValue / doubleValue
        System.out.println(new BigDecimal("42.9").intValue());
        System.out.println(new BigDecimal("3.14").doubleValue());

        // toPlainString
        System.out.println(new BigDecimal("1E+5").toPlainString());

        // valueOf
        System.out.println(BigDecimal.valueOf(7).add(BigDecimal.valueOf(3)));

        // scale / precision
        BigDecimal c = new BigDecimal("123.456");
        System.out.println(c.scale());
        System.out.println(c.precision());

        System.out.println("done");
    }
}
