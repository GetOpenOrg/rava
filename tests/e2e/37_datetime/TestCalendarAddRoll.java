import java.util.Calendar;
import java.util.TimeZone;

/**
 * Calendar 的 add 与 roll 区别（lang3 DateUtils / joda 互操作的地基，此前零覆盖）：
 * add 跨字段进位、roll 不进位、同字段多次 roll 环绕、clear/set 基准。
 */
public class TestCalendarAddRoll {

    static String fmt(Calendar c) {
        return c.get(Calendar.YEAR) + "-"
                + String.format("%02d", c.get(Calendar.MONTH) + 1) + "-"
                + String.format("%02d", c.get(Calendar.DAY_OF_MONTH));
    }

    static Calendar base() {
        Calendar c = Calendar.getInstance(TimeZone.getTimeZone("GMT"));
        c.clear();
        c.set(2026, Calendar.OCTOBER, 31);
        return c;
    }

    public static void main(String[] args) {
        // add：跨字段进位（10-31 + 1 天 → 11-01）
        Calendar add = base();
        add.add(Calendar.DAY_OF_MONTH, 1);
        System.out.println("add-day=" + fmt(add));

        // add 负数跨年
        Calendar back = base();
        back.add(Calendar.MONTH, -10);
        System.out.println("add-month-neg=" + fmt(back));

        // roll：不进位（10-31 roll 1 天 → 10-01）
        Calendar rollDay = base();
        rollDay.roll(Calendar.DAY_OF_MONTH, 1);
        System.out.println("roll-day=" + fmt(rollDay));

        // roll 月：10 月 roll 3 → 1 月，年不动
        Calendar rollMonth = base();
        rollMonth.roll(Calendar.MONTH, 3);
        System.out.println("roll-month=" + fmt(rollMonth));

        // roll 负环绕：1 月 roll -1 → 12 月
        Calendar rollNeg = base();
        rollNeg.set(2026, Calendar.JANUARY, 15);
        rollNeg.roll(Calendar.MONTH, -1);
        System.out.println("roll-neg=" + fmt(rollNeg));

        // add 大跨度跨年
        Calendar year = base();
        year.add(Calendar.DAY_OF_MONTH, 400);
        System.out.println("add-400d=" + fmt(year));

        // getActualMaximum 与月末
        Calendar feb = base();
        feb.set(2024, Calendar.FEBRUARY, 1);   // 闰年
        System.out.println("feb-max=" + feb.getActualMaximum(Calendar.DAY_OF_MONTH));
        Calendar feb28 = base();
        feb28.set(2023, Calendar.FEBRUARY, 1);
        System.out.println("feb28-max=" + feb28.getActualMaximum(Calendar.DAY_OF_MONTH));

        // 字段族
        Calendar fields = base();
        System.out.println("dow=" + fields.get(Calendar.DAY_OF_WEEK)
                + " week-of-year=" + fields.get(Calendar.WEEK_OF_YEAR));
    }
}
