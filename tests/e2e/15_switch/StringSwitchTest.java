/**
 * String switch 测试（依赖 String.hashCode）
 */
public class StringSwitchTest {
    public static void main(String[] args) {
        String[] days = {"Monday", "Wednesday", "Friday", "Sunday", "Unknown"};
        for (String day : days) {
            String type = getType(day);
            System.out.println(day + " -> " + type);
        }
    }

    static String getType(String day) {
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
}
