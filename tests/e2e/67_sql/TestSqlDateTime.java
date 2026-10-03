import java.sql.Time;
import java.sql.Timestamp;

/**
 * java.sql 日期时间族：valueOf/toString 往返、纳秒、toInstant/Local 转换、compareTo
 * （jmod 覆盖计划 A 档；平台类加载器载入 java.sql 模块）。
 */
public class TestSqlDateTime {

    public static void main(String[] args) {
        java.sql.Date d = java.sql.Date.valueOf("2026-10-03");
        System.out.println("date=" + d + " local=" + d.toLocalDate());

        Time t = Time.valueOf("12:34:56");
        System.out.println("time=" + t + " local=" + t.toLocalTime());

        Timestamp ts = Timestamp.valueOf("2026-10-03 12:34:56.123456789");
        System.out.println("ts=" + ts);
        System.out.println("nanos=" + ts.getNanos());
        System.out.println("instant=" + ts.toInstant());
        System.out.println("local-dt=" + ts.toLocalDateTime());

        Timestamp a = Timestamp.valueOf("2026-01-01 00:00:00");
        Timestamp b = Timestamp.valueOf("2026-01-02 00:00:00");
        System.out.println("cmp=" + a.compareTo(b) + " rev=" + b.compareTo(a) + " same=" + a.compareTo(a));
        System.out.println("after=" + a.after(b) + " before=" + a.before(b));

        // toString 截断：毫秒 3 位形态
        Timestamp ms = Timestamp.valueOf("2026-10-03 12:34:56.500");
        System.out.println("ms-ts=" + ms);

        // 非法格式 → IllegalArgumentException（只打印类型，消息文本不比对）
        try {
            java.sql.Date.valueOf("2026/10/03");
        } catch (IllegalArgumentException e) {
            System.out.println("date-iae=" + e.getClass().getSimpleName());
        }
        try {
            Time.valueOf("99:99:99");
        } catch (IllegalArgumentException e) {
            System.out.println("time-iae=" + e.getClass().getSimpleName());
        }
    }
}
