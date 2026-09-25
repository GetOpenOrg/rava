/**
 * Month 5 test: Multi-catch — different exception types route to different catch blocks,
 * exception hierarchy matching.
 */
public class MultiCatchTest {
    public static void testMultiCatch(int x) {
        try {
            if (x == 1) {
                throw new IllegalArgumentException("bad argument");
            } else if (x == 2) {
                throw new RuntimeException("runtime error");
            } else if (x == 3) {
                throw new ArithmeticException("math error");
            }
            System.out.println("no exception");
        } catch (IllegalArgumentException e) {
            System.out.println("caught IllegalArgumentException");
        } catch (ArithmeticException e) {
            System.out.println("caught ArithmeticException");
        } catch (RuntimeException e) {
            System.out.println("caught RuntimeException");
        }
    }

    public static void testHierarchy() {
        // IllegalArgumentException is a subclass of RuntimeException
        try {
            throw new IllegalArgumentException("hierarchy test");
        } catch (RuntimeException e) {
            System.out.println("caught as RuntimeException");
        }
    }

    public static void testNested() {
        try {
            try {
                throw new ArithmeticException("inner");
            } catch (IllegalArgumentException e) {
                System.out.println("inner: IllegalArgument");
            }
        } catch (ArithmeticException e) {
            System.out.println("outer: ArithmeticException");
        }
    }

    public static void main(String[] args) {
        testMultiCatch(0);
        testMultiCatch(1);
        testMultiCatch(2);
        testMultiCatch(3);
        testHierarchy();
        testNested();
    }
}
