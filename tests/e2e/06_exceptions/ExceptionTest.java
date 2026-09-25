public class ExceptionTest {
    public static int safeDivide(int a, int b) {
        try {
            if (b == 0) {
                throw new RuntimeException("division by zero");
            }
            return a / b;
        } catch (RuntimeException e) {
            System.out.println("Caught exception");
            return -1;
        }
    }

    public static void main(String[] args) {
        System.out.println(safeDivide(10, 2));
        System.out.println(safeDivide(10, 0));
        System.out.println(safeDivide(15, 3));
    }
}
