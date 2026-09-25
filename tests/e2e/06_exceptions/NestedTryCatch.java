public class NestedTryCatch {
    public static void main(String[] args) {
        // 1. Inner catch handles exception; outer try continues normally.
        try {
            System.out.println("outer try");
            try {
                System.out.println("inner try");
                throw new RuntimeException("inner error");
            } catch (RuntimeException e) {
                System.out.println("inner catch: " + e.getMessage());
            }
            System.out.println("after inner try");
        } catch (RuntimeException e) {
            System.out.println("outer catch (should not reach)");
        }

        // 2. Inner catch type doesn't match; exception bubbles to outer catch.
        try {
            try {
                throw new RuntimeException("bubbled");
            } catch (IllegalArgumentException e) {
                System.out.println("wrong catch (should not reach)");
            }
            System.out.println("after inner (should not reach)");
        } catch (RuntimeException e) {
            System.out.println("outer caught: " + e.getMessage());
        }

        // 3. No exception; both try bodies complete normally.
        try {
            int x = 1;
            try {
                x = x + 1;
            } catch (RuntimeException e) {
                System.out.println("inner catch (should not reach)");
            }
            System.out.println("x = " + x);
        } catch (RuntimeException e) {
            System.out.println("outer catch (should not reach)");
        }

        System.out.println("done");
    }
}
