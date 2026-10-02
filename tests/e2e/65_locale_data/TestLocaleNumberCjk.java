import java.text.DecimalFormatSymbols;
import java.text.NumberFormat;
import java.util.Locale;

/**
 * jdk.localedata 非英语 CLDR 数据：CJK 与德语 Locale 的数字格式化
 * （jmod 覆盖计划 A 档；LocaleProviderAdapter / 资源包类装载，A 档最大闭包之一）。
 */
public class TestLocaleNumberCjk {

    public static void main(String[] args) {
        Locale[] ls = { Locale.CHINA, Locale.TAIWAN, Locale.JAPAN, Locale.KOREA, Locale.GERMANY };
        long v = 1234567;
        double d = 1234.5;
        for (Locale l : ls) {
            NumberFormat nf = NumberFormat.getNumberInstance(l);
            System.out.println(l.toLanguageTag() + " int=" + nf.format(v)
                    + " dec=" + nf.format(d));
            DecimalFormatSymbols s = new DecimalFormatSymbols(l);
            System.out.println("  group=U+" + String.format("%04X", (int) s.getGroupingSeparator())
                    + " decimal=U+" + String.format("%04X", (int) s.getDecimalSeparator())
                    + " minus=[" + s.getMinusSign() + "]");
        }
    }
}
