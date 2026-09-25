public class TryFinally {
    static void doWork(boolean shouldThrow) {
        try {
            System.out.println("try");
            if (shouldThrow) {
                throw new RuntimeException("test error");
            }
            System.out.println("after work");
        } finally {
            System.out.println("finally");
        }
    }

    public static void main(String[] args) {
        // Test 1: normal path — finally runs after try body
        doWork(false);

        // Test 2: exception path — finally runs before propagation
        try {
            doWork(true);
        } catch (RuntimeException e) {
            System.out.println("caught: " + e.getMessage());
        }

        System.out.println("done");
    }
}
