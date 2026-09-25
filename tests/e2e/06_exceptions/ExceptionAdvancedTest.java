import java.util.ArrayList;
import java.util.List;

public class ExceptionAdvancedTest {
    public static void main(String[] args) {
        // 1. Multi-catch
        for (int i = 0; i < 3; i++) {
            try {
                if (i == 0) throw new IllegalArgumentException("bad arg");
                if (i == 1) throw new UnsupportedOperationException("not supported");
                if (i == 2) throw new ArithmeticException("math error");
            } catch (IllegalArgumentException | UnsupportedOperationException e) {
                System.out.println("caught multi: " + e.getMessage());
            } catch (ArithmeticException e) {
                System.out.println("caught arith: " + e.getMessage());
            }
        }

        // 2. Exception chaining
        try {
            try {
                throw new RuntimeException("root cause");
            } catch (RuntimeException e) {
                throw new IllegalStateException("wrapped: " + e.getMessage());
            }
        } catch (IllegalStateException e) {
            System.out.println("chained: " + e.getMessage());
        }

        // 3. Finally always runs
        StringBuilder sb = new StringBuilder();
        try {
            sb.append("try ");
            throw new RuntimeException("test");
        } catch (RuntimeException e) {
            sb.append("catch ");
        } finally {
            sb.append("finally");
        }
        System.out.println("flow: " + sb.toString());

        // 4. Exception in loop with accumulator
        List<Integer> valid = new ArrayList<>();
        List<Integer> inputs = new ArrayList<>();
        inputs.add(1); inputs.add(0); inputs.add(2); inputs.add(0); inputs.add(3);
        for (int x : inputs) {
            try {
                if (x == 0) throw new ArithmeticException("/ by zero");
                valid.add(100 / x);
            } catch (ArithmeticException e) {
                // skip zeros
            }
        }
        System.out.println("valid: " + valid);

        // 5. Nested try-catch
        try {
            try {
                throw new IllegalArgumentException("inner");
            } finally {
                System.out.println("inner finally");
            }
        } catch (IllegalArgumentException e) {
            System.out.println("outer catch: " + e.getMessage());
        }

        // 6. Exception message
        try {
            throw new RuntimeException("test message 123");
        } catch (RuntimeException e) {
            System.out.println("message: " + e.getMessage());
        }

        System.out.println("Done.");
    }
}
