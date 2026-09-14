public class TestEnumBasic {
    enum Day {
        MONDAY, TUESDAY, WEDNESDAY, THURSDAY, FRIDAY, SATURDAY, SUNDAY
    }

    public static void main(String[] args) {
        Day d = Day.WEDNESDAY;
        System.out.println(d);
        System.out.println(d.name());
        System.out.println(d.ordinal());

        Day[] days = Day.values();
        System.out.println(days.length);

        Day fromName = Day.valueOf("FRIDAY");
        System.out.println(fromName);
    }
}
