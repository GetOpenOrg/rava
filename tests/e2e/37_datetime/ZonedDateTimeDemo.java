import java.time.Instant;
import java.time.LocalDate;
import java.time.ZoneOffset;
import java.time.Period;
import java.time.format.DateTimeFormatter;

public class ZonedDateTimeDemo {
    public static void main(String[] args) {
        // Instant
        Instant epoch = Instant.ofEpochSecond(0);
        System.out.println(epoch.getEpochSecond());  // 0
        System.out.println(epoch.toEpochMilli());    // 0

        Instant later = epoch.plusSeconds(3600);
        System.out.println(later.getEpochSecond());  // 3600
        System.out.println(later.isAfter(epoch));    // true

        // ZoneOffset
        ZoneOffset utc = ZoneOffset.UTC;
        System.out.println(utc.getTotalSeconds());   // 0
        ZoneOffset plus8 = ZoneOffset.ofHours(8);
        System.out.println(plus8.getTotalSeconds()); // 28800

        // Period
        Period p = Period.of(1, 2, 3);
        System.out.println(p.getYears());    // 1
        System.out.println(p.getMonths());   // 2
        System.out.println(p.getDays());     // 3

        Period days = Period.ofDays(10);
        System.out.println(days.isNegative());  // false
        System.out.println(days.isZero());      // false

        // DateTimeFormatter
        DateTimeFormatter fmt = DateTimeFormatter.ofPattern("yyyy-MM-dd");
        LocalDate date = LocalDate.of(2026, 9, 11);
        System.out.println(date.format(fmt));  // 2026-09-11
    }
}
