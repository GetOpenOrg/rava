
import java.time.ZoneOffset;
import java.util.Locale;

import org.joda.time.DateTime;
import org.joda.time.DateTimeZone;
import org.joda.time.format.DateTimeFormat;
import org.joda.time.format.ISODateTimeFormat;

/**
 * joda-time pilot A4：固定时区/固定时刻的解析、格式化与不可变操作链（W1 #2）。
 */
public class JodaBasicMain {

    static void check(String name, Object expected, Object actual) {
        boolean ok = String.valueOf(expected).equals(String.valueOf(actual));
        System.out.println((ok ? "PASS " : "FAIL ") + name
                + (ok ? "" : " :: expected=<" + expected + "> actual=<" + actual + ">"));
    }

    public static void main(String[] args) {
        Locale.setDefault(Locale.CHINA);
        DateTimeZone.setDefault(DateTimeZone.forOffsetHours(8));

        DateTime t = new DateTime(2026, 10, 5, 14, 30, 0, DateTimeZone.forOffsetHours(8));

        check("tostring", "2026-10-05T14:30:00.000+08:00", t.toString());
        check("field-get", 14, t.getHourOfDay());
        check("dayofweek", 1, t.getDayOfWeek()); // 2026-10-05 是周一（ISO 周 1=Monday）

        // 不可变链：plus/with 均返回新实例
        DateTime plus7 = t.plusDays(7);
        DateTime withMonth = t.withMonthOfYear(3);
        check("plus-days", "2026-10-12T14:30:00.000+08:00", plus7.toString());
        check("with-month", "2026-03-05T14:30:00.000+08:00", withMonth.toString());
        check("immutable-self", "2026-10-05T14:30:00.000+08:00", t.toString());

        // 解析与格式化
        DateTime parsed = DateTime.parse("2026-01-15T09:00:00.000+08:00");
        check("parse", "2026-01-15T09:00:00.000+08:00", parsed.toString());
        check("fmt-pattern", "2026/10/05 14:30",
                DateTimeFormat.forPattern("yyyy/MM/dd HH:mm").print(t));
        check("fmt-iso-date", "2026-10-05", ISODateTimeFormat.date().print(t));

        // 时区换算
        DateTime utc = t.withZone(DateTimeZone.UTC);
        check("zone-convert", "2026-10-05T06:30:00.000Z", utc.toString());
        check("millis-roundtrip", t.getMillis(), parsed.withMillis(t.getMillis()).getMillis());

        // 比较
        check("is-before", true, t.isBefore(plus7));
        check("is-after", false, t.isAfter(plus7));
        System.out.println("DONE JodaBasicMain");
    }
}
