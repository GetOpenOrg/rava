
import java.util.Locale;
import java.util.TreeSet;

import org.joda.time.Chronology;
import org.joda.time.DateTime;
import org.joda.time.DateTimeZone;
import org.joda.time.DateTimeFieldType;
import org.joda.time.chrono.BuddhistChronology;
import org.joda.time.chrono.GJChronology;
import org.joda.time.chrono.ISOChronology;

/**
 * joda-time pilot A6：Chronology 继承树与字段遍历（W1 #2）。
 * ISO/GJ/Buddhist 三历法同刻的年字段差异 + 字段元数据枚举。
 */
public class JodaChronologyMain {

    static void check(String name, Object expected, Object actual) {
        boolean ok = String.valueOf(expected).equals(String.valueOf(actual));
        System.out.println((ok ? "PASS " : "FAIL ") + name
                + (ok ? "" : " :: expected=<" + expected + "> actual=<" + actual + ">"));
    }

    public static void main(String[] args) {
        Locale.setDefault(Locale.CHINA);
        DateTimeZone tz = DateTimeZone.forOffsetHours(8);

        Chronology iso = ISOChronology.getInstance(tz);
        Chronology gj = GJChronology.getInstance(tz);
        Chronology buddhist = BuddhistChronology.getInstance(tz);

        DateTime base = new DateTime(2026, 10, 5, 0, 0, 0, 0, iso);

        check("iso-year", 2026, base.getYear());
        check("gj-year", 2026, base.withChronology(gj).getYear());
        check("buddhist-year", 2569, base.withChronology(buddhist).getYear()); // 2026 + 543
        check("buddhist-chrono-class", BuddhistChronology.class.getName(),
                buddhist.getClass().getName());

        // 历法字段：同一时刻不同历法的 year 字段
        var isoYear = iso.year();
        var budYear = buddhist.year();
        long instant = base.getMillis();
        check("field-get-iso", 2026, isoYear.get(instant));
        check("field-get-buddhist", 2569, budYear.get(instant));
        check("field-leap-iso", false, isoYear.isLeap(instant));
        check("field-set-year", 2027, iso.year().get(iso.year().set(instant, 2027)));

        // 继承树类型面（Chronology 家族核心方法的确定输出）
        check("iso-tostring", "ISOChronology[+08:00]", iso.toString());
        check("zone-id", "+08:00", iso.getZone().getID());

        // 字段清单（TreeSet 排序保证顺序）
        TreeSet<String> fieldNames = new TreeSet<>();
        for (var t : new org.joda.time.DateTimeFieldType[]{
                DateTimeFieldType.year(), DateTimeFieldType.monthOfYear(),
                DateTimeFieldType.dayOfMonth(), DateTimeFieldType.hourOfDay(),
                DateTimeFieldType.minuteOfHour(), DateTimeFieldType.secondOfMinute()}) {
            fieldNames.add(iso.toString() + "#" + t.getName());
        }
        System.out.println("  iso field types: " + fieldNames);
        System.out.println("DONE JodaChronologyMain");
    }
}
