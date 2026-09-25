/**
 * Java 14+ Switch 表达式测试（箭头语法 + yield）
 */
public class SwitchExprTest {
    public static void main(String[] args) {
        // Switch expression with arrow syntax
        for (int i = 1; i <= 4; i++) {
            String result = switch (i) {
                case 1 -> "one";
                case 2 -> "two";
                case 3 -> "three";
                default -> "other";
            };
            System.out.println(i + " -> " + result);
        }

        // Switch expression with yield
        for (int i = 0; i < 3; i++) {
            int val = switch (i) {
                case 0 -> 100;
                case 1 -> {
                    int tmp = 200 + 50;
                    yield tmp;
                }
                default -> -1;
            };
            System.out.println("val(" + i + ") = " + val);
        }
    }
}
