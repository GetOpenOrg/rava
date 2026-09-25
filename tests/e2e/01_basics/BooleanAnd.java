public class BooleanAnd {
    // Tests && operator used as a boolean expression (not just a condition)
    static boolean both(boolean a, boolean b) {
        return a && b;
    }

    static boolean allThree(boolean a, boolean b, boolean c) {
        return a && b && c;
    }

    static String classify(int x) {
        if (x > 0 && x < 10) {
            return "single digit positive";
        } else if (x >= 10 && x <= 99) {
            return "double digit";
        } else {
            return "other";
        }
    }

    public static void main(String[] args) {
        System.out.println(both(true, true));
        System.out.println(both(true, false));
        System.out.println(both(false, true));
        System.out.println(both(false, false));
        System.out.println(allThree(true, true, true));
        System.out.println(allThree(true, false, true));
        System.out.println(classify(5));
        System.out.println(classify(42));
        System.out.println(classify(-1));
    }
}
