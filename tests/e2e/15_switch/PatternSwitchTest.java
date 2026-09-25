public class PatternSwitchTest {
    static String describe(Object obj) {
        return switch (obj) {
            case Integer i -> "int: " + i;
            case String s  -> "str: " + s;
            case Double d  -> "double: " + d;
            default        -> "other: " + obj;
        };
    }

    public static void main(String[] args) {
        System.out.println(describe(42));
        System.out.println(describe("hello"));
        System.out.println(describe(3.14));
        System.out.println(describe(true));
    }
}
