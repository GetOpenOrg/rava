import java.time.*;

public class JavaDateDemo {
    public static void main(String[] args) {
        // LocalDate
        LocalDate date = LocalDate.of(2026, 9, 5);
        System.out.println(date.getYear());
        System.out.println(date.getMonthValue());
        System.out.println(date.getDayOfMonth());
        System.out.println(date.plusDays(3));
        System.out.println(date.minusDays(1));
        System.out.println(date.plusMonths(1));

        // LocalTime
        LocalTime time = LocalTime.of(14, 30, 15);
        System.out.println(time.getHour());
        System.out.println(time.getMinute());
        System.out.println(time.getSecond());
        System.out.println(time.plusHours(2));

        // LocalDateTime
        LocalDateTime dt = LocalDateTime.of(date, time);
        System.out.println(dt.getYear());
        System.out.println(dt.getHour());
        System.out.println(dt.toLocalDate().equals(date));
        System.out.println(dt.toLocalTime().equals(time));

        // Duration
        Duration d = Duration.ofSeconds(3661);
        System.out.println(d.getSeconds());
        System.out.println(d.toMinutes());
        Duration d2 = Duration.ofMillis(5000);
        System.out.println(d2.toMillis());

        // Period
        Period p = Period.of(1, 2, 3);
        System.out.println(p.getYears());
        System.out.println(p.getMonths());
        System.out.println(p.getDays());

        // Instant
        Instant epoch = Instant.ofEpochMilli(0);
        System.out.println(epoch.toEpochMilli());

        System.out.println("done");
    }
}
