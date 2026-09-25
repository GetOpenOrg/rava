public class EnumSwitch {
    enum Day { MON, TUE, WED, THU, FRI }

    static String describe(Day d) {
        switch (d) {
            case MON: return "Monday";
            case TUE: return "Tuesday";
            case WED: return "Wednesday";
            case THU: return "Thursday";
            case FRI: return "Friday";
            default:  return "Weekend";
        }
    }

    public static void main(String[] args) {
        System.out.println(describe(Day.MON));
        System.out.println(describe(Day.TUE));
        System.out.println(describe(Day.WED));
        System.out.println(describe(Day.THU));
        System.out.println(describe(Day.FRI));
    }
}
