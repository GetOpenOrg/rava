
import java.util.Locale;

import org.joda.time.DateTime;
import org.joda.time.DateTimeZone;
import org.joda.time.Days;
import org.joda.time.Duration;
import org.joda.time.Interval;
import org.joda.time.Months;
import org.joda.time.Period;
import org.joda.time.format.PeriodFormat;

/**
 * joda-time pilot A5：Period/Duration/Interval 的区间计算（W1 #2）。
 */
public class JodaPeriodMain {

    static void check(String name, Object expected, Object actual) {
        boolean ok = String.valueOf(expected).equals(String.valueOf(actual));
        System.out.println((ok ? "PASS " : "FAIL ") + name
                + (ok ? "" : " :: expected=<" + expected + "> actual=<" + actual + ">"));
    }

    public static void main(String[] args) {
        Locale.setDefault(Locale.CHINA);
        DateTimeZone.setDefault(DateTimeZone.forOffsetHours(8));

        DateTime start = new DateTime(2026, 1, 1, 0, 0, 0, 0, DateTimeZone.forOffsetHours(8));
        DateTime end = new DateTime(2026, 10, 5, 14, 30, 0, 0, DateTimeZone.forOffsetHours(8));

        // Duration：绝对毫秒
        Duration duration = new Duration(start, end);
        check("duration-millis", 23985000000L, duration.getMillis());
        check("duration-standard-days", 277, duration.getStandardDays());

        // Period：日历字段视角
        Period period = new Period(start, end);
        check("period-months", 9, period.getMonths());
        check("period-days", 4, period.getDays());
        check("period-hours", 14, period.getHours());
        check("period-minutes", 30, period.getMinutes());

        // 工厂比较器
        check("days-between", 277, Days.daysBetween(start, end).getDays());
        check("months-between", 9, Months.monthsBetween(start, end).getMonths());

        // Interval：包含与重叠
        Interval whole = new Interval(start, end);
        DateTime mid = new DateTime(2026, 5, 1, 0, 0, 0, 0, DateTimeZone.forOffsetHours(8));
        check("interval-contains", true, whole.contains(mid));
        Interval overlap = new Interval(mid, end.plusDays(1));
        check("interval-overlap", true, whole.overlaps(overlap));
        check("interval-abuts", true, whole.abuts(new Interval(end, end.plusDays(1))));

        // Period 文本化（中文 locale 下 PeriodFormat 的输出）
        String text = PeriodFormat.getDefault().print(new Period(0, 9, 0, 4, 14, 30, 0, 0));
        System.out.println("  period-text: " + text);

        // Period 与 Duration 互换的语义差异（月≠固定天数）
        check("plus-period", "2026-10-01T00:00:00.000+08:00",
                start.plus(Months.NINE).toString());
        check("plus-duration", "2026-01-01T01:00:00.000+08:00",
                start.plus(Duration.standardHours(1)).toString());
        System.out.println("DONE JodaPeriodMain");
    }
}
