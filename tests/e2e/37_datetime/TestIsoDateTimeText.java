import java.time.LocalDate;
import java.time.LocalDateTime;
import java.time.LocalTime;

/**
 * LocalDate / LocalTime / LocalDateTime 的 ISO toString 文本全分支
 *（JDK 25 经 jdk.internal.util.DateTimeHelper.formatTo 输出）。
 */
public class TestIsoDateTimeText {
    public static void main(String[] args) {
        // 年份：1/2/3/4 位、负数（含 |y|<1000 与 ≥1000）、>9999 带 '+'、0 年
        int[] years = {5, 42, 999, 2024, 9999, 10000, 123456, 0, -1, -42, -999, -2024};
        for (int y : years) {
            System.out.println("date." + y + "=" + LocalDate.of(y, 3, 7));
        }
        // 月 / 日两位与一位
        System.out.println("date.md=" + LocalDate.of(2024, 12, 31) + "," + LocalDate.of(2024, 1, 1)
                + "," + LocalDate.of(2024, 10, 9) + "," + LocalDate.of(2024, 9, 10));
        // 时间：秒纳秒全零只出 HH:mm；秒非零；纳秒非零时秒也输出
        System.out.println("time.hm=" + LocalTime.of(0, 0) + "," + LocalTime.of(9, 5) + "," + LocalTime.of(23, 59));
        System.out.println("time.s=" + LocalTime.of(10, 20, 30) + "," + LocalTime.of(1, 2, 3));
        System.out.println("time.nanoOnly=" + LocalTime.of(10, 20, 0, 1));
        // 纳秒精度截断：毫秒 / 微秒 / 纳秒 三档，含前导零
        int[] nanos = {500_000_000, 5_000_000, 123_000_000, 1_000, 123_456_000, 1, 123_456_789, 999_999_999, 10_000_000};
        for (int n : nanos) {
            System.out.println("time.nano." + n + "=" + LocalTime.of(12, 34, 56, n));
        }
        // LocalDateTime 组合（'T' 分隔）
        System.out.println("dt=" + LocalDateTime.of(2024, 2, 29, 8, 0) + ","
                + LocalDateTime.of(-50, 11, 5, 23, 59, 59, 7_000_000) + ","
                + LocalDateTime.of(12345, 6, 7, 0, 0, 1));
        // StringBuilder 追加（toString 经 formatTo 写入 buffer）
        StringBuilder sb = new StringBuilder("at ");
        sb.append(LocalDateTime.of(1999, 12, 31, 23, 59, 59)).append(" / ").append(LocalTime.MIDNIGHT);
        System.out.println("sb=" + sb);
        System.out.println("minmax=" + LocalDate.MIN + "," + LocalDate.MAX + "," + LocalTime.MAX);
    }
}
