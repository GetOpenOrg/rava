import java.time.LocalDate;
import java.time.LocalDateTime;
import java.time.Instant;

public class LocalDateTimeTest {
    public static void main(String[] args) {
        // LocalDate
        LocalDate date = LocalDate.of(2024, 6, 15);
        System.out.println("date: " + date);
        System.out.println("year: " + date.getYear());
        System.out.println("month: " + date.getMonthValue());
        System.out.println("day: " + date.getDayOfMonth());
        System.out.println("dayOfYear: " + date.getDayOfYear());

        LocalDate plusDays = date.plusDays(10);
        System.out.println("plus10days: " + plusDays);
        LocalDate minusMonths = date.minusMonths(2);
        System.out.println("minus2months: " + minusMonths);

        LocalDate date2 = LocalDate.of(2024, 12, 25);
        System.out.println("isBefore: " + date.isBefore(date2));
        System.out.println("isAfter: " + date.isAfter(date2));
        System.out.println("isEqual: " + date.isEqual(date));

        System.out.println("leapYear2024: " + date.isLeapYear());
        LocalDate date3 = LocalDate.of(2023, 1, 1);
        System.out.println("leapYear2023: " + date3.isLeapYear());

        // Parse
        LocalDate parsed = LocalDate.parse("2025-03-15");
        System.out.println("parsed: " + parsed);

        // Plus years
        LocalDate plusYears = date.plusYears(2);
        System.out.println("plus2years: " + plusYears);

        // LocalDateTime
        LocalDateTime dt = LocalDateTime.of(2024, 6, 15, 10, 30, 45);
        System.out.println("datetime: " + dt);
        System.out.println("hour: " + dt.getHour());
        System.out.println("minute: " + dt.getMinute());
        System.out.println("second: " + dt.getSecond());

        LocalDateTime plusHours = dt.plusHours(5);
        System.out.println("plus5hours: " + plusHours);
        LocalDateTime minusDays2 = dt.minusDays(3);
        System.out.println("minus3days: " + minusDays2);

        LocalDateTime dt2 = LocalDateTime.of(2024, 12, 25, 0, 0, 0);
        System.out.println("dtBefore: " + dt.isBefore(dt2));

        // toLocalDate
        LocalDate fromDt = dt.toLocalDate();
        System.out.println("toLocalDate: " + fromDt);

        // Instant
        Instant epoch = Instant.ofEpochSecond(0);
        System.out.println("epoch: " + epoch);

        Instant milli = Instant.ofEpochMilli(1000);
        System.out.println("milli: " + milli);
        System.out.println("epochSec: " + milli.getEpochSecond());
        System.out.println("toEpochMilli: " + milli.toEpochMilli());

        Instant later = epoch.plusSeconds(3600);
        System.out.println("plus1hour: " + later);
        System.out.println("isBefore: " + epoch.isBefore(later));
        System.out.println("isAfter: " + epoch.isAfter(later));
    }
}
