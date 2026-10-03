import java.time.Instant;
import java.time.ZoneId;
import java.time.ZonedDateTime;
import java.util.Calendar;
import java.util.GregorianCalendar;
import java.util.TimeZone;
import java.util.concurrent.TimeUnit;

/**
 * TimeUnit 换算与 Calendar 毫秒/时区通道（方法级实测：toNanos 11 jar /
 * ofNanos 6 / atZone 7 / ofInstant 5 / getTimeInMillis 10 / setTimeInMillis 7 /
 * TimeZone.getID 12，此前零覆盖）。
 */
public class TestTimeUnitCalendar {

    public static void main(String[] args) {
        // TimeUnit 换算族
        System.out.println("to-nanos=" + TimeUnit.MILLISECONDS.toNanos(3));
        System.out.println("to-millis=" + TimeUnit.SECONDS.toMillis(2));
        System.out.println("to-seconds=" + TimeUnit.MINUTES.toSeconds(5));
        System.out.println("to-hours=" + TimeUnit.DAYS.toHours(2));
        System.out.println("convert=" + TimeUnit.NANOSECONDS.convert(1500, TimeUnit.MICROSECONDS));

        // Duration/Instant 通道：ofNanos/ofMillis → atZone → ZonedDateTime
        java.time.Duration d = java.time.Duration.ofNanos(1_500_000_000L);
        System.out.println("dur-seconds=" + d.getSeconds() + " nanos=" + d.getNano());
        Instant inst = Instant.ofEpochSecond(1_700_000_000L);
        ZonedDateTime zdt = inst.atZone(ZoneId.of("Asia/Shanghai"));
        System.out.println("zdt-hour=" + zdt.getHour());
        ZonedDateTime utc = inst.atZone(ZoneId.of("UTC"));
        System.out.println("utc-hour=" + utc.getHour());

        // Instant.ofEpochMilli 与 ZonedDateTime.ofInstant 双向
        ZonedDateTime viaOf = ZonedDateTime.ofInstant(Instant.ofEpochMilli(1_700_000_000_000L),
                ZoneId.of("GMT"));
        System.out.println("of-instant=" + viaOf.getYear() + "-" + viaOf.getHour());

        // Calendar 毫秒通道（getTimeInMillis/setTimeInMillis）
        Calendar c = GregorianCalendar.from(zdt);
        System.out.println("millis-consistent=" + (c.getTimeInMillis() == zdt.toInstant().toEpochMilli()));
        Calendar c2 = new GregorianCalendar(TimeZone.getTimeZone("GMT"));
        c2.clear();
        c2.setTimeInMillis(1_700_000_000_000L);
        System.out.println("set-millis-md=" + (c2.get(Calendar.MONTH) + 1)
                + "-" + c2.get(Calendar.DAY_OF_MONTH) + " h=" + c2.get(Calendar.HOUR_OF_DAY));

        // TimeZone 通道：getID/getRawOffset/getDefault 不打印默认值（跨机）
        TimeZone sh = TimeZone.getTimeZone("Asia/Shanghai");
        System.out.println("tz-id=" + sh.getID()
                + " raw-offset-hours=" + (sh.getRawOffset() / 3_600_000));
        System.out.println("tz-no-dst=" + !sh.useDaylightTime());
        TimeZone ny = TimeZone.getTimeZone("America/New_York");
        System.out.println("ny-dst=" + ny.useDaylightTime()
                + " dst-save=" + ny.getDSTSavings() / 3_600_000);
        // DST 生效与否按固定时刻判定
        System.out.println("ny-jan-dst=" + ny.inDaylightTime(
                new java.util.Date(1_767_225_600_000L)));   // 2026-01-01
        System.out.println("ny-jul-dst=" + ny.inDaylightTime(
                new java.util.Date(1_783_003_200_000L)));   // 2026-07-03
    }
}
