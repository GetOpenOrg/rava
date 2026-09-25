public class SwitchExpression {
    static int score(int grade) {
        return switch (grade) {
            case 1 -> 10;
            case 2 -> 20;
            default -> 0;
        };
    }

    static String size(int n) {
        return switch (n) {
            case 1 -> "small";
            case 2 -> "medium";
            default -> "large";
        };
    }

    static int doubled(int x) {
        return switch (x) {
            case 1 -> {
                int r = x * 2;
                yield r;
            }
            case 2 -> x * 3;
            default -> x;
        };
    }

    public static void main(String[] args) {
        System.out.println(score(1));
        System.out.println(score(2));
        System.out.println(score(3));
        System.out.println(size(1));
        System.out.println(size(2));
        System.out.println(size(3));
        System.out.println(doubled(1));
        System.out.println(doubled(2));
        System.out.println(doubled(5));
    }
}
