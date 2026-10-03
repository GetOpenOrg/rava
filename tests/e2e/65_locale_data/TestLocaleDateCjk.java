import java.time.DayOfWeek;
import java.time.LocalDate;
import java.time.Month;
import java.time.format.DateTimeFormatter;
import java.time.format.FormatStyle;
import java.time.format.TextStyle;
import java.util.Locale;

/**
 * jdk.localedata 非英语 CLDR 数据：本地化日期与日历字段显示名
 * （jmod 覆盖计划 A 档；固定日期，无时间成分，输出确定）。
 */
public class TestLocaleDateCjk {

    public static void main(String[] args) {
        LocalDate d = LocalDate.of(2026, 10, 3);
        Locale[] ls = { Locale.CHINA, Locale.TAIWAN, Locale.JAPAN, Locale.KOREA, Locale.GERMANY };
        for (Locale l : ls) {
            System.out.println(l.toLanguageTag()
                    + " full=" + d.format(DateTimeFormatter.ofLocalizedDate(FormatStyle.FULL).withLocale(l)));
            System.out.println(l.toLanguageTag()
                    + " long=" + d.format(DateTimeFormatter.ofLocalizedDate(FormatStyle.LONG).withLocale(l)));
        }
        System.out.println("dow-cn=" + DayOfWeek.MONDAY.getDisplayName(TextStyle.FULL, Locale.CHINA));
        System.out.println("dow-kr=" + DayOfWeek.FRIDAY.getDisplayName(TextStyle.SHORT, Locale.KOREA));
        System.out.println("month-jp=" + Month.OCTOBER.getDisplayName(TextStyle.FULL, Locale.JAPAN));
        System.out.println("month-de=" + Month.MARCH.getDisplayName(TextStyle.SHORT, Locale.GERMANY));
    }
}
