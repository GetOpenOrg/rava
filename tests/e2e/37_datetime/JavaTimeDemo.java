import java.time.LocalDate;
import java.time.LocalTime;
import java.time.LocalDateTime;
import java.time.Duration;
import java.time.Period;

public class JavaTimeDemo {
    public static void main(String[] args) {
        // LocalDate — fixed date, no now()
        LocalDate d1 = LocalDate.parse("2024-03-15");
        LocalDate d2 = LocalDate.parse("2025-07-04");

        System.out.println("d1 = " + d1);
        System.out.println("d2 = " + d2);
        System.out.println("d1.getYear() = " + d1.getYear());
        System.out.println("d1.getMonthValue() = " + d1.getMonthValue());
        System.out.println("d1.getDayOfMonth() = " + d1.getDayOfMonth());
        System.out.println("d1.isBefore(d2) = " + d1.isBefore(d2));
        System.out.println("d1.isAfter(d2) = " + d1.isAfter(d2));
        System.out.println("d1.plusDays(10) = " + d1.plusDays(10));
        System.out.println("d1.plusMonths(2) = " + d1.plusMonths(2));
        System.out.println("d1.plusYears(1) = " + d1.plusYears(1));
        System.out.println("d1.minusDays(5) = " + d1.minusDays(5));

        // Period
        Period period = Period.between(d1, d2);
        System.out.println("period years = " + period.getYears());
        System.out.println("period months = " + period.getMonths());
        System.out.println("period days = " + period.getDays());

        // LocalTime
        LocalTime t = LocalTime.parse("14:30:00");
        System.out.println("time = " + t);
        System.out.println("hour = " + t.getHour());
        System.out.println("minute = " + t.getMinute());
        System.out.println("second = " + t.getSecond());
        System.out.println("plusHours(3) = " + t.plusHours(3));

        // Duration
        Duration dur = Duration.ofHours(2).plusMinutes(30);
        System.out.println("duration = " + dur);
        System.out.println("duration toMinutes = " + dur.toMinutes());
    }
}
