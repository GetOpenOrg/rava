import java.time.Duration;
import java.time.Period;
import java.time.LocalDate;

public class TestDurationPeriod {
    public static void main(String[] args) {
        Duration d1 = Duration.ofHours(25);
        Duration d2 = Duration.ofMinutes(90);
        System.out.println("hours=" + d1.toHours());
        System.out.println("minutes=" + d1.toMinutes());
        System.out.println("plus=" + d1.plus(d2).toMinutes());
        System.out.println("minus=" + d1.minus(Duration.ofMinutes(30)).toMinutes());
        System.out.println("seconds=" + d2.getSeconds());
        System.out.println("between=" + Duration.between(LocalDate.of(2024, 1, 1).atStartOfDay(),
                LocalDate.of(2024, 1, 3).atStartOfDay()).toDays());
        Period p = Period.of(2, 3, 10);
        System.out.println("py=" + p.getYears());
        System.out.println("pm=" + p.getMonths());
        System.out.println("pd=" + p.getDays());
        System.out.println("pwith=" + p.withYears(5));
        Period pb = Period.between(LocalDate.of(2020, 1, 1), LocalDate.of(2023, 4, 8));
        System.out.println("pbetween=" + pb);
        System.out.println("ptotMonths=" + pb.toTotalMonths());
    }
}
