public class ExceptionHierarchy {
    // Test 1: catch superclass catches subclass with conditional throw (tests structurizer fix)
    static String catchByBase(boolean throwRuntime) {
        try {
            if (throwRuntime) throw new RuntimeException("rte");
            return "no-throw";
        } catch (Exception e) {
            return "caught-exception: " + e.getMessage();
        }
    }

    // Test 2: more specific catch before general catch
    static String specificFirst() {
        try {
            throw new IllegalArgumentException("bad-arg");
        } catch (IllegalArgumentException e) {
            return "illegal-arg: " + e.getMessage();
        } catch (RuntimeException e) {
            return "runtime: " + e.getMessage();
        }
    }

    // Test 3: general catch used when no specific match
    static String generalFallback() {
        try {
            throw new RuntimeException("general");
        } catch (IllegalArgumentException e) {
            return "should-not-match";
        } catch (RuntimeException e) {
            return "runtime-fallback: " + e.getMessage();
        }
    }

    // Test 4: user-defined exception extending RuntimeException
    static String userException() {
        try {
            throw new RuntimeException("user-exc");
        } catch (Exception e) {
            return "user-caught: " + e.getMessage();
        }
    }

    public static void main(String[] args) {
        System.out.println(catchByBase(true));
        System.out.println(catchByBase(false));
        System.out.println(specificFirst());
        System.out.println(generalFallback());
        System.out.println(userException());
    }
}
