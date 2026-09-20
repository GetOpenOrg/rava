import java.util.*;

public class TestEnumSetMap {
    enum Day { MON, TUE, WED, THU, FRI, SAT, SUN }

    public static void main(String[] args) {
        EnumSet<Day> all = EnumSet.allOf(Day.class);
        System.out.println("all=" + all);
        EnumSet<Day> few = EnumSet.of(Day.MON, Day.WED);
        System.out.println("few=" + few);
        EnumSet<Day> range = EnumSet.range(Day.TUE, Day.THU);
        System.out.println("range=" + range);
        EnumSet<Day> none = EnumSet.noneOf(Day.class);
        System.out.println("none=" + none);
        EnumMap<Day, String> em = new EnumMap<>(Day.class);
        em.put(Day.MON, "work");
        em.put(Day.SAT, "rest");
        System.out.println("em=" + em);
        System.out.println("emGet=" + em.get(Day.MON));
        try {
            Day.valueOf("NOPE");
        } catch (IllegalArgumentException ex) {
            System.out.println("valueOfBad=" + ex.getClass().getSimpleName());
        }
        System.out.println("vals=" + Arrays.toString(Day.values()));
    }
}
