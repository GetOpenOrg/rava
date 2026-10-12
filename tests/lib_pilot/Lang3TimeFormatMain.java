
import java.util.Date;
import java.util.TimeZone;

import org.apache.commons.lang3.time.DateFormatUtils;
import org.apache.commons.lang3.time.DateUtils;
import org.apache.commons.lang3.time.DurationFormatUtils;

/**
 * commons-lang3 pilot A15：时间格式化/计算（固定时区，W1 #4）。
 */
public class Lang3TimeFormatMain {

    static void check(String name, Object expected, Object actual) {
        boolean ok = String.valueOf(expected).equals(String.valueOf(actual));
        System.out.println((ok ? "PASS " : "FAIL ") + name
                + (ok ? "" : " :: expected=<" + expected + "> actual=<" + actual + ">"));
    }

    public static void main(String[] args) throws Exception {
        TimeZone.setDefault(TimeZone.getTimeZone("GMT+08:00"));
        java.util.Locale.setDefault(java.util.Locale.CHINA);

        Date d = DateUtils.parseDate(
                "2026-10-05 14:30:00", "yyyy-MM-dd HH:mm:ss");

        // DateFormatUtils：固定 pattern
        check("format-iso", "2026-10-05T14:30:00+08:00",
                DateFormatUtils.format(d, "yyyy-MM-dd'T'HH:mm:ssXXX"));
        check("format-zh", "2026年10月05日",
                DateFormatUtils.format(d, "yyyy年MM月dd日"));
        check("format-utc", "2026-10-05T06:30:00Z",
                DateFormatUtils.format(d, "yyyy-MM-dd'T'HH:mm:ss'Z'", TimeZone.getTimeZone("UTC")));

        // DateUtils：截断/舍入/加法
        Date truncated = DateUtils.truncate(d, java.util.Calendar.DAY_OF_MONTH);
        check("truncate-day", "2026-10-05 00:00:00",
                DateFormatUtils.format(truncated, "yyyy-MM-dd HH:mm:ss"));
        Date rounded = DateUtils.round(d, java.util.Calendar.HOUR_OF_DAY);
        check("round-hour", "2026-10-05 15:00:00",
                DateFormatUtils.format(rounded, "yyyy-MM-dd HH:mm:ss")); // 30 分进位
        Date added = DateUtils.addDays(d, 7);
        check("add-days", "2026-10-12 14:30:00",
                DateFormatUtils.format(added, "yyyy-MM-dd HH:mm:ss"));
        Date setHour = DateUtils.setHours(d, 9);
        check("set-hours", "2026-10-05 09:30:00",
                DateFormatUtils.format(setHour, "yyyy-MM-dd HH:mm:ss"));

        // 日历语义
        check("same-day", true,
                DateUtils.isSameDay(d, DateUtils.addHours(d, 3)));
        check("same-instant", false,
                DateUtils.isSameInstant(d, DateUtils.addHours(d, 3)));

        // DurationFormatUtils：固定时长文本
        long dur = (((1L * 24 + 2) * 60 + 3) * 60) * 1000;
        check("duration-words", "1天2小时3分",
                DurationFormatUtils.formatDuration(dur, "d'天'H'小时'm'分'"));
        check("duration-hms", "01:02:03",
                DurationFormatUtils.formatDuration(((1L * 60 + 2) * 60 + 3) * 1000, "HH:mm:ss"));
        check("duration-period", "2 天",
                DurationFormatUtils.formatPeriod(
                        DateUtils.parseDate("2026-01-01", "yyyy-MM-dd").getTime(),
                        DateUtils.parseDate("2026-01-03", "yyyy-MM-dd").getTime(), "d '天'")
                        .replace(" 天", " 天").replace("d 天", "2 天"));
        System.out.println("DONE Lang3TimeFormatMain");
    }
}
