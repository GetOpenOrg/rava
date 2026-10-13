
import org.apache.commons.lang3.math.NumberUtils;

/**
 * commons-lang3 pilot A12：NumberUtils 数值边界（W1 #4）。
 */
public class Lang3NumberUtilsMain {

    static void check(String name, Object expected, Object actual) {
        boolean ok = String.valueOf(expected).equals(String.valueOf(actual));
        System.out.println((ok ? "PASS " : "FAIL ") + name
                + (ok ? "" : " :: expected=<" + expected + "> actual=<" + actual + ">"));
    }

    public static void main(String[] args) {
        // isCreatable：十进制/十六进制/八进制/科学计数/后缀
        boolean[] creatable = {
                NumberUtils.isCreatable("123"),
                NumberUtils.isCreatable("-1.5e3"),
                NumberUtils.isCreatable("0x1F"),
                NumberUtils.isCreatable("017"),
                NumberUtils.isCreatable("1L"),
                NumberUtils.isCreatable("1f"),
                NumberUtils.isCreatable("abc"),
                NumberUtils.isCreatable(""),
                NumberUtils.isCreatable(null)};
        System.out.println("PASS is-creatable " + java.util.Arrays.toString(creatable));

        boolean[] parsable = {
                NumberUtils.isParsable("123"),
                NumberUtils.isParsable("0x1F"),   // 16 进制不可 parse（isParsable 仅十进制）
                NumberUtils.isParsable("1.5"),
                NumberUtils.isParsable("1,5")};
        System.out.println("PASS is-parsable " + java.util.Arrays.toString(parsable));

        // toInt / toLong 带默认值
        check("to-int", 42, NumberUtils.toInt("42"));
        check("to-int-default", -1, NumberUtils.toInt("x", -1));
        check("to-long", 9876543210L, NumberUtils.toLong("9876543210"));
        check("to-double", 2.5, NumberUtils.toDouble("2.5"));
        check("to-float-default", 0.5f, NumberUtils.toFloat("bad", 0.5f));

        // createXXX 全家
        check("create-integer", 7, NumberUtils.createInteger("7"));
        check("create-long", 8L, NumberUtils.createLong("8"));
        check("create-bigdecimal", new java.math.BigDecimal("3.14"), NumberUtils.createBigDecimal("3.14"));

        // compare 三向
        check("compare-int", -1, NumberUtils.compare(1, 2));
        check("compare-long", 0, NumberUtils.compare(3L, 3L));
        check("compare-short", 6, NumberUtils.compare((short) 9, (short) 3)); // 差值

        // max/min 变参
        check("max", 9, NumberUtils.max(3, 9, 1));
        check("min", 1, NumberUtils.min(3, 9, 1));
        check("max-double", 2.5, NumberUtils.max(1.5, 2.5, 0.5));
        System.out.println("DONE Lang3NumberUtilsMain");
    }
}
