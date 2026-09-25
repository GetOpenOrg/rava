/**
 * Month 5 test: finally blocks — catch-all handlers, re-throw, try/catch/finally.
 */
public class FinallyTest {
    public static void testTryFinally() {
        try {
            System.out.println("try block");
        } finally {
            System.out.println("finally block");
        }
    }

    public static void testTryCatchFinally() {
        try {
            throw new RuntimeException("error");
        } catch (RuntimeException e) {
            System.out.println("caught exception");
        } finally {
            System.out.println("finally after catch");
        }
    }

    public static void testFinallyWithException() {
        try {
            try {
                throw new RuntimeException("inner");
            } finally {
                System.out.println("inner finally");
            }
        } catch (RuntimeException e) {
            System.out.println("outer caught");
        }
    }

    public static void main(String[] args) {
        testTryFinally();
        testTryCatchFinally();
        testFinallyWithException();
    }
}
