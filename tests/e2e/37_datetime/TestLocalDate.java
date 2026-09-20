import java.time.LocalDate;
import java.time.DayOfWeek;

public class TestLocalDate {
    public static void main(String[] args) {
        LocalDate d = LocalDate.of(2024, 2, 29);
        System.out.println("date=" + d);
        System.out.println("leap=" + d.isLeapYear());
        System.out.println("dow=" + d.getDayOfWeek());
        System.out.println("len=" + d.lengthOfMonth());
        System.out.println("plus=" + d.plusDays(1));
        System.out.println("minus=" + d.minusMonths(1));
        System.out.println("with=" + d.with(DayOfWeek.MONDAY));
        System.out.println("cmp=" + d.compareTo(LocalDate.of(2024, 3, 1)));
        System.out.println("epoch=" + LocalDate.of(1970, 1, 1).toEpochDay());
        LocalDate d2 = LocalDate.of(2023, 2, 28);
        System.out.println("notleap=" + d2.isLeapYear());
        System.out.println("dayOfYear=" + d.getDayOfYear());
    }
}
