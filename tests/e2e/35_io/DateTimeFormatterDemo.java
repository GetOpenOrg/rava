import java.time.*;
import java.time.format.*;

public class DateTimeFormatterDemo {
    public static void main(String[] args) {
        // Standard formatters
        LocalDate date = LocalDate.of(2026, 9, 5);
        System.out.println(date.format(DateTimeFormatter.ISO_LOCAL_DATE));

        LocalTime time = LocalTime.of(14, 30, 0);
        System.out.println(time.format(DateTimeFormatter.ISO_LOCAL_TIME));

        LocalDateTime dt = LocalDateTime.of(date, time);
        System.out.println(dt.format(DateTimeFormatter.ISO_LOCAL_DATE_TIME));

        // Custom patterns
        DateTimeFormatter fmt1 = DateTimeFormatter.ofPattern("yyyy/MM/dd");
        System.out.println(date.format(fmt1));

        DateTimeFormatter fmt2 = DateTimeFormatter.ofPattern("dd-MMM-yyyy", java.util.Locale.ENGLISH);
        System.out.println(date.format(fmt2));

        DateTimeFormatter fmt3 = DateTimeFormatter.ofPattern("HH:mm:ss");
        System.out.println(time.format(fmt3));

        // parse
        LocalDate parsed = LocalDate.parse("2026-09-05", DateTimeFormatter.ISO_LOCAL_DATE);
        System.out.println(parsed.equals(date));

        LocalDate parsed2 = LocalDate.parse("2026/09/05", fmt1);
        System.out.println(parsed2.equals(date));

        // ZonedDateTime with UTC
        ZonedDateTime zdt = ZonedDateTime.of(dt, ZoneId.of("UTC"));
        System.out.println(zdt.getZone());
        System.out.println(zdt.getYear());

        System.out.println("done");
    }
}
