import java.text.DecimalFormatSymbols;
import java.text.NumberFormat;
import java.util.Currency;
import java.util.Locale;

/**
 * L-2 货币数据层：Currency 按国家取货币（currency.data）、本地符号（CLDR CurrencyNames）、
 * 小数位 / 数字代码，DecimalFormatSymbols 的货币字段与货币格式。
 */
public class TestCurrencyApi {
    public static void main(String[] args) {
        Currency usd = Currency.getInstance(Locale.US);
        System.out.println(usd.getCurrencyCode() + " " + usd.getDefaultFractionDigits() + " " + usd.getSymbol(Locale.US));
        Currency jpy = Currency.getInstance(Locale.JAPAN);
        System.out.println(jpy.getCurrencyCode() + " " + jpy.getDefaultFractionDigits() + " "
                + jpy.getSymbol(Locale.JAPAN) + " " + jpy.getSymbol(Locale.US));
        Currency eur = Currency.getInstance("EUR");
        System.out.println(eur.getSymbol(Locale.GERMANY) + " " + eur.getSymbol(Locale.US) + " " + eur.getNumericCode());
        Currency gbp = Currency.getInstance(Locale.UK);
        System.out.println(gbp + " " + gbp.getSymbol(Locale.UK) + " " + (gbp == Currency.getInstance("GBP")));
        DecimalFormatSymbols s = DecimalFormatSymbols.getInstance(Locale.US);
        System.out.println(s.getCurrency() + " " + s.getCurrencySymbol() + " " + s.getInternationalCurrencySymbol());
        System.out.println(NumberFormat.getCurrencyInstance(Locale.US).format(1234.5));
        System.out.println(NumberFormat.getCurrencyInstance(Locale.JAPAN).format(1234.5));
        try {
            Currency.getInstance("XYZ");
        } catch (IllegalArgumentException e) {
            System.out.println("IAE");
        }
    }
}
