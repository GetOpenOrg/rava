import java.text.DecimalFormat;
import java.text.NumberFormat;
import java.util.Locale;

public class TestFormatLocale {
    public static void main(String[] args) {
        System.out.println("us=" + String.format(Locale.US, "%,.2f", 1234.5));
        System.out.println("de=" + String.format(Locale.GERMANY, "%,.2f", 1234.5));
        NumberFormat nf = NumberFormat.getCurrencyInstance(Locale.US);
        System.out.println("cur=" + nf.format(99.99));
        DecimalFormat df = new DecimalFormat("#,###.00");
        System.out.println("dec=" + df.format(1234567.891));
        System.out.println("pct=" + NumberFormat.getPercentInstance(Locale.US).format(0.25));
    }
}
