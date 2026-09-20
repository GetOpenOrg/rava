import java.time.ZonedDateTime;
import java.time.ZoneId;
import java.time.LocalDateTime;

public class TestZonedDateTime {
    public static void main(String[] args) {
        ZonedDateTime z1 = ZonedDateTime.of(LocalDateTime.of(2024, 6, 1, 12, 0), ZoneId.of("America/New_York"));
        ZonedDateTime z2 = z1.withZoneSameInstant(ZoneId.of("Asia/Tokyo"));
        System.out.println("ny=" + z1);
        System.out.println("tokyo=" + z2);
        System.out.println("offsetHours=" + (z2.getOffset().getTotalSeconds() - z1.getOffset().getTotalSeconds()) / 3600);
        System.out.println("sameInstant=" + z1.toInstant().equals(z2.toInstant()));
        ZonedDateTime z3 = ZonedDateTime.of(LocalDateTime.of(2024, 1, 1, 0, 0), ZoneId.of("UTC"));
        System.out.println("utc=" + z3);
        System.out.println("zoneId=" + z2.getZone());
    }
}
