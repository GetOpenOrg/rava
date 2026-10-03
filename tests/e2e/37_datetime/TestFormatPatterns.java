import java.text.DateFormat;
import java.text.DecimalFormat;
import java.text.NumberFormat;
import java.text.SimpleDateFormat;
import java.util.Date;
import java.util.Locale;
import java.util.TimeZone;

/**
 * 格式模式往返与工厂族（方法级实测：DecimalFormat.toPattern 12 jar /
 * setGroupingUsed 6 / DateFormat.getTimeInstance 6 / getDateInstance 6，
 * 此前零覆盖）：toPattern/applyPattern 往返、分组开关、locale 工厂三档。
 */
public class TestFormatPatterns {

    public static void main(String[] args) {
        DecimalFormat df = new DecimalFormat("#,##0.00");
        System.out.println("fmt=" + df.format(1234567.891));
        System.out.println("pattern=" + df.toPattern());

        // applyPattern → toPattern 往返
        df.applyPattern("0.###");
        System.out.println("reapplied=" + df.toPattern() + " out=" + df.format(1.23456));

        // 前后缀与百分号模式
        df.applyPattern("\"€\"#,##0.00\"\"");
        System.out.println("prefixed=" + df.format(9.5));
        df.applyPattern("0.##%");
        System.out.println("percent=" + df.format(0.4567));

        // setGroupingUsed
        NumberFormat nf = NumberFormat.getInstance(Locale.US);
        nf.setGroupingUsed(false);
        System.out.println("no-group=" + nf.format(9876543));
        nf.setGroupingUsed(true);
        System.out.println("group=" + nf.format(9876543));

        // 负号与多重模式
        df.applyPattern("#,##0.00;(#,##0.00)");
        System.out.println("neg=" + df.format(-1234.5));

        // DateFormat 工厂三档（固定时区/locale，只打形态）
        TimeZone gmt = TimeZone.getTimeZone("GMT");
        Date d = new Date(1_700_000_000_000L);
        SimpleDateFormat time = (SimpleDateFormat) DateFormat.getTimeInstance(DateFormat.SHORT, Locale.US);
        time.setTimeZone(gmt);
        System.out.println("time-short=" + time.format(d));
        SimpleDateFormat date = (SimpleDateFormat) DateFormat.getDateInstance(DateFormat.MEDIUM, Locale.US);
        date.setTimeZone(gmt);
        System.out.println("date-medium=" + date.format(d));
        SimpleDateFormat both = (SimpleDateFormat) DateFormat.getDateTimeInstance(
                DateFormat.MEDIUM, DateFormat.MEDIUM, Locale.US);
        both.setTimeZone(gmt);
        System.out.println("datetime=" + both.format(d));
    }
}
