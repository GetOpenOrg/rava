import java.math.BigDecimal;
import java.math.RoundingMode;

public class TestBigDecimal {
    public static void main(String[] args) {
        BigDecimal a = new BigDecimal("1.1");
        BigDecimal b = new BigDecimal("2.2");
        System.out.println("add=" + a.add(b));
        System.out.println("sub=" + b.subtract(a));
        System.out.println("mul=" + a.multiply(b));
        System.out.println("div=" + a.divide(new BigDecimal("3"), 10, RoundingMode.HALF_UP));
        System.out.println("scaleSet=" + new BigDecimal("3.14159").setScale(2, RoundingMode.HALF_UP));
        System.out.println("cmp=" + a.compareTo(b));
        System.out.println("abs=" + new BigDecimal("-5.5").abs());
        System.out.println("neg=" + a.negate());
        System.out.println("max=" + a.max(b));
        System.out.println("strip=" + new BigDecimal("10.0").stripTrailingZeros().toPlainString());
        System.out.println("plain=" + new BigDecimal("1E+2").toPlainString());
        System.out.println("precision=" + a.precision() + ",scale=" + a.scale());
        System.out.println("zero=" + BigDecimal.ZERO);
        System.out.println("one=" + BigDecimal.ONE);
        System.out.println("ten=" + BigDecimal.TEN);
        System.out.println("pow=" + new BigDecimal("2").pow(10));
    }
}
