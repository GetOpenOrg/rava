public class TestSwitchExpression {
    public static void main(String[] args) {
        // Switch expression with -> (Java 14+)
        for (int day = 1; day <= 7; day++) {
            String type = switch (day) {
                case 1, 7 -> "weekend";
                case 2, 3, 4, 5, 6 -> "weekday";
                default -> "unknown";
            };
            System.out.println(day + ": " + type);
        }

        // Switch expression with yield
        int x = 3;
        int result = switch (x) {
            case 1 -> 10;
            case 2 -> 20;
            default -> {
                int temp = x * x;
                yield temp + 1;
            }
        };
        System.out.println(result);
    }
}
