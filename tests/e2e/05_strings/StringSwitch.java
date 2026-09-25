public class StringSwitch {
    static String describe(String day) {
        switch (day) {
            case "Monday":
            case "Tuesday":
            case "Wednesday":
            case "Thursday":
            case "Friday":
                return "Weekday";
            case "Saturday":
            case "Sunday":
                return "Weekend";
            default:
                return "Unknown";
        }
    }

    static int priority(String level) {
        switch (level) {
            case "HIGH": return 3;
            case "MEDIUM": return 2;
            case "LOW": return 1;
            default: return 0;
        }
    }

    public static void main(String[] args) {
        System.out.println(describe("Monday"));    // Weekday
        System.out.println(describe("Saturday"));  // Weekend
        System.out.println(describe("Holiday"));   // Unknown
        System.out.println(priority("HIGH"));      // 3
        System.out.println(priority("LOW"));       // 1
        System.out.println(priority("NONE"));      // 0
    }
}
