import java.math.BigDecimal;
import java.math.RoundingMode;

/**
 * BigDecimal 精确转换与舍入边界（JSON/DB 数值通道的地基，intValueExact 此前零覆盖）：
 * intValueExact/longValueExact/toBigIntegerExact 命中与截断异常、setScale 舍入模式族
 * 含 UNNECESSARY 异常。
 */
public class TestBigDecimalExactConversions {

    public static void main(String[] args) {
        System.out.println("int-exact=" + new BigDecimal("42").intValueExact());
        System.out.println("long-exact=" + new BigDecimal("9223372036854775807").longValueExact());
        System.out.println("bigint-exact=" + new BigDecimal("12345678901234567890")
                .toBigIntegerExact());

        // 截断（小数部分）→ ArithmeticException
        try {
            new BigDecimal("1.5").intValueExact();
        } catch (ArithmeticException e) {
            System.out.println("frac-ex=" + e.getClass().getSimpleName());
        }
        // 溢出（超 int 范围）→ ArithmeticException
        try {
            new BigDecimal("3000000000").intValueExact();
        } catch (ArithmeticException e) {
            System.out.println("overflow-ex=" + e.getClass().getSimpleName());
        }
        // 非精确转换（不抛，截断取整）
        System.out.println("intvalue-trunc=" + new BigDecimal("2.9").intValue());
        System.out.println("intvalue-neg=" + new BigDecimal("-2.9").intValue());

        // setScale 舍入模式族
        BigDecimal v = new BigDecimal("2.55");
        System.out.println("half-up=" + v.setScale(1, RoundingMode.HALF_UP));
        System.out.println("half-even=" + v.setScale(1, RoundingMode.HALF_EVEN));
        System.out.println("down=" + v.setScale(1, RoundingMode.DOWN));
        System.out.println("up=" + v.setScale(1, RoundingMode.UP));
        System.out.println("floor=" + new BigDecimal("-2.55").setScale(1, RoundingMode.FLOOR));
        System.out.println("ceiling=" + new BigDecimal("-2.51").setScale(1, RoundingMode.CEILING));
        try {
            new BigDecimal("2.55").setScale(1, RoundingMode.UNNECESSARY);
        } catch (ArithmeticException e) {
            System.out.println("unnecessary-ex=" + e.getClass().getSimpleName());
        }

        // scale/unscaledValue/stripTrailingZeros 形态
        System.out.println("scale=" + new BigDecimal("1.230").scale()
                + " stripped=" + new BigDecimal("1.230").stripTrailingZeros()
                + " plain=" + new BigDecimal("1E+3").toPlainString());
    }
}
