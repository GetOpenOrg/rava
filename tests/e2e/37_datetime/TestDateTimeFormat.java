import java.time.LocalDateTime;
import java.time.format.DateTimeFormatter;
import java.util.Locale;

public class TestDateTimeFormat {
    public static void main(String[] args) {
        DateTimeFormatter f = DateTimeFormatter.ofPattern("yyyy-MM-dd HH:mm:ss");
        LocalDateTime t = LocalDateTime.of(2024, 3, 15, 13, 45, 30);
        System.out.println("custom=" + t.format(f));
        System.out.println("parsed=" + LocalDateTime.parse("2024-03-15 13:45:30", f));
        System.out.println("iso=" + t.format(DateTimeFormatter.ISO_LOCAL_DATE));
        System.out.println("basic=" + t.format(DateTimeFormatter.BASIC_ISO_DATE));
        DateTimeFormatter f2 = DateTimeFormatter.ofPattern("EEEE, MMMM yyyy", Locale.ENGLISH);
        System.out.println("named=" + t.format(f2));
        System.out.println("ofPatternInt=" + t.format(DateTimeFormatter.ofPattern("yyyy/MM")));
    }
}
