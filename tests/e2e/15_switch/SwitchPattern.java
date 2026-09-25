public class SwitchPattern {
    static String describe(Object obj) {
        return switch (obj) {
            case Integer i -> "Integer: " + i;
            case String s  -> "String: " + s;
            case null      -> "null";
            default        -> "other";
        };
    }

    public static void main(String[] args) {
        System.out.println(describe(42));
        System.out.println(describe("hello"));
        System.out.println(describe(null));
        System.out.println(describe(3.14));
    }
}
