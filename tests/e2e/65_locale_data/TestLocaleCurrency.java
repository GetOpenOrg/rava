import java.text.NumberFormat;
import java.util.Currency;
import java.util.Locale;

/**
 * jdk.localedata：货币符号与 locale 感知的 String.format
 * （jmod 覆盖计划 A 档；Currency 资源数据 + 分组/小数随 locale 变化）。
 */
public class TestLocaleCurrency {

    public static void main(String[] args) {
        Currency cny = Currency.getInstance(Locale.CHINA);
        System.out.println("cny=" + cny.getCurrencyCode() + " sym=[" + cny.getSymbol(Locale.CHINA) + "]");
        Currency jpy = Currency.getInstance(Locale.JAPAN);
        System.out.println("jpy=" + jpy.getCurrencyCode() + " sym=[" + jpy.getSymbol(Locale.JAPAN) + "]"
                + " digits=" + jpy.getDefaultFractionDigits());
        Currency eur = Currency.getInstance(Locale.GERMANY);
        System.out.println("eur=" + eur.getCurrencyCode() + " sym=[" + eur.getSymbol(Locale.GERMANY) + "]");

        System.out.println("fmt-cn=" + String.format(Locale.CHINA, "%,.2f", 1234567.891));
        System.out.println("fmt-de=" + String.format(Locale.GERMANY, "%,.2f", 1234567.891));
        System.out.println("fmt-jp=" + String.format(Locale.JAPAN, "%,.2f", 1234567.891));

        System.out.println("cur-cn=" + NumberFormat.getCurrencyInstance(Locale.CHINA).format(9999.5));
        System.out.println("cur-kr=" + NumberFormat.getCurrencyInstance(Locale.KOREA).format(1234));
    }
}
