public class TestSwitchNull {
    static String f(String s) {
        return switch (s) {
            case null -> "NULL";
            case "a" -> "A";
            case String t -> "len" + t.length();
        };
    }

    static String g(String s) {
        try {
            return switch (s) {
                case "a" -> "A";
                default -> "other";
            };
        } catch (NullPointerException e) {
            return "NPE:" + e.getClass().getSimpleName();
        }
    }

    public static void main(String[] args) {
        System.out.println(f(null));
        System.out.println(f("a"));
        System.out.println(f("hello"));
        System.out.println(g(null));
        System.out.println(g("a"));
        System.out.println(g("b"));
    }
}
