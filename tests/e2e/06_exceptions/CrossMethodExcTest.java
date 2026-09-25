public class CrossMethodExcTest {
    public static void main(String[] args) {
        for (int i = -1; i <= 1; i++) {
            try {
                String result = safeDivide(10, i);
                System.out.println("10/" + i + " = " + result);
            } catch (ArithmeticException e) {
                System.out.println("10/" + i + " error: " + e.getMessage());
            }
        }
        System.out.println("Done.");
    }

    static String safeDivide(int a, int b) {
        if (b == 0) {
            throw new ArithmeticException("/ by zero");
        }
        return String.valueOf(a / b);
    }
}
