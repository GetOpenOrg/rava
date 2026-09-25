/**
 * Month 3 测试：Switch 语句 + 基础异常
 */
public class SwitchTest {
    public static String dayName(int day) {
        switch (day) {
            case 1: return "Monday";
            case 2: return "Tuesday";
            case 3: return "Wednesday";
            case 4: return "Thursday";
            case 5: return "Friday";
            case 6: return "Saturday";
            case 7: return "Sunday";
            default: return "Unknown";
        }
    }

    public static int classify(int x) {
        if (x < 0) return -1;
        switch (x) {
            case 0: return 0;
            case 1:
            case 2:
            case 3:
                return 1;
            default:
                return 2;
        }
    }

    public static void main(String[] args) {
        System.out.println(dayName(1));
        System.out.println(dayName(5));
        System.out.println(dayName(7));
        System.out.println(dayName(9));

        System.out.println(classify(-5));
        System.out.println(classify(0));
        System.out.println(classify(2));
        System.out.println(classify(10));
    }
}
