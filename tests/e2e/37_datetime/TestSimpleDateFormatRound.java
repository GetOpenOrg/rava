import java.text.ParseException;
import java.text.SimpleDateFormat;
import java.util.Date;
import java.util.Locale;
import java.util.TimeZone;

/**
 * SimpleDateFormat 遗留格式化通道（joda 互操作与 lang3 FastDateFormat 的语义镜像，
 * e2e 此前零覆盖）：固定时区/locale 的 format-parse 往返、lenient 边界、模式字母族。
 */
public class TestSimpleDateFormatRound {

    public static void main(String[] args) throws Exception {
        TimeZone tz = TimeZone.getTimeZone("Asia/Shanghai");
        SimpleDateFormat fmt = new SimpleDateFormat("yyyy-MM-dd HH:mm:ss", Locale.ROOT);
        fmt.setTimeZone(tz);

        Date d = fmt.parse("2026-10-03 12:34:56");
        System.out.println("epoch-millis-pos=" + (d.getTime() > 0));
        System.out.println("roundtrip=" + fmt.format(d));

        SimpleDateFormat day = new SimpleDateFormat("yyyy-MM-dd EEE", Locale.US);
        day.setTimeZone(tz);
        System.out.println("dow=" + day.format(d));

        SimpleDateFormat zoneFmt = new SimpleDateFormat("yyyy-MM-dd HH:mm zzz", Locale.ROOT);
        zoneFmt.setTimeZone(tz);
        System.out.println("zone=" + zoneFmt.format(d));

        // 两位年窗口（yy → 2026）
        SimpleDateFormat yy = new SimpleDateFormat("yyMMdd", Locale.ROOT);
        yy.setTimeZone(TimeZone.getTimeZone("GMT"));
        Date parsed = yy.parse("261003");
        System.out.println("yy-back=" + yy.format(parsed));

        // lenient=false：非法日 → ParseException；lenient=true：滚动到下月
        SimpleDateFormat strict = new SimpleDateFormat("yyyy-MM-dd", Locale.ROOT);
        strict.setTimeZone(TimeZone.getTimeZone("GMT"));
        strict.setLenient(false);
        try {
            strict.parse("2026-11-31");
            System.out.println("strict=ok");
        } catch (ParseException e) {
            System.out.println("strict-ex=" + e.getClass().getSimpleName()
                    + " at=" + e.getErrorOffset());
        }
        SimpleDateFormat loose = new SimpleDateFormat("yyyy-MM-dd", Locale.ROOT);
        loose.setTimeZone(TimeZone.getTimeZone("GMT"));
        System.out.println("loose=" + loose.format(loose.parse("2026-11-31")));

        // parse 不匹配前缀：ParsePosition(0) 处停
        try {
            fmt.parse("not-a-date");
        } catch (ParseException e) {
            System.out.println("bad-input-ex=" + e.getClass().getSimpleName());
        }
    }
}
