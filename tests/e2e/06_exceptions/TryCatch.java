public class TryCatch {
    // Throws when fail=true; otherwise does nothing.
    static void mayThrow(boolean fail) {
        if (fail) {
            throw new RuntimeException("test error");
        }
    }

    public static void main(String[] args) {
        // 1. No exception path: catch block should NOT execute.
        try {
            mayThrow(false);
            System.out.println("no exception");
        } catch (RuntimeException e) {
            System.out.println("unexpected catch");
        }

        // 2. Exception path: catch block MUST execute.
        try {
            mayThrow(true);
            System.out.println("should not reach");
        } catch (RuntimeException e) {
            System.out.println("caught exception");
        }

        System.out.println("done");
    }
}
