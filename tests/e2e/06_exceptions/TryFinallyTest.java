public class TryFinallyTest {
    static class AppException extends Exception {
        public AppException(String msg) {
            super(msg);
        }
    }

    public static void main(String[] args) {
        // Test 1: Basic try-catch
        try {
            int x = 10 / 0;
            System.out.println("unreachable");
        } catch (ArithmeticException e) {
            System.out.println("Caught: " + e.getMessage());
        }

        // Test 2: try-finally (no exception)
        try {
            System.out.println("try block");
        } finally {
            System.out.println("finally block");
        }

        // Test 3: try-catch-finally (with exception)
        try {
            throw new RuntimeException("test error");
        } catch (RuntimeException e) {
            System.out.println("Caught: " + e.getMessage());
        } finally {
            System.out.println("finally after catch");
        }

        // Test 4: Nested try-catch
        try {
            try {
                throw new IllegalArgumentException("inner");
            } catch (IllegalArgumentException e) {
                System.out.println("Inner caught: " + e.getMessage());
                throw new RuntimeException("rethrown");
            }
        } catch (RuntimeException e) {
            System.out.println("Outer caught: " + e.getMessage());
        }

        // Test 5: Catch specific exception
        try {
            throw new IllegalStateException("state error");
        } catch (IllegalStateException e) {
            System.out.println("Caught state: " + e.getMessage());
        }

        // Test 6: Custom exception
        try {
            throwCustom();
        } catch (AppException e) {
            System.out.println("Custom: " + e.getMessage());
        }

        // Test 7: Finally always runs (normal path)
        System.out.println("Before: " + safeMethod());

        // Test 8: Exception in method call
        try {
            riskyMethod(-1);
        } catch (IllegalArgumentException e) {
            System.out.println("Method threw: " + e.getMessage());
        }

        // Test 9: Catch and continue in loop
        int sum = 0;
        for (int i = 0; i < 5; i++) {
            try {
                sum += 10 / (i - 2);
            } catch (ArithmeticException e) {
                sum += 0;
            }
        }
        System.out.println("Sum: " + sum);

        // Test 10: Exception message construction
        try {
            throw new RuntimeException("error " + 42);
        } catch (RuntimeException e) {
            System.out.println(e.getMessage());
        }
    }

    static void throwCustom() throws AppException {
        throw new AppException("custom error");
    }

    static String safeMethod() {
        try {
            return "ok";
        } finally {
            // finally runs even with return
        }
    }

    static int riskyMethod(int x) {
        if (x < 0) {
            throw new IllegalArgumentException("negative: " + x);
        }
        return x * 2;
    }
}
