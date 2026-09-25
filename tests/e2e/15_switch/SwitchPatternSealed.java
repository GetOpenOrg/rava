
// Java 21: switch(obj) { case Type t when guard -> ... ; case null -> ... }
public class SwitchPatternSealed {
    public static void main(String[] args) {
        System.out.println(stringify(42));
        System.out.println(stringify(-42));
        System.out.println(stringify("Some text"));
        System.out.println(stringify(""));
        System.out.println(stringify(null));
    }

    static String stringify(Object value) {
        return switch (value) {
            case Integer i when i == 42 -> "42 is the answer";
            case Integer i when i > 0   -> "positive number";
            case Integer i when i < 0   -> "negative number";
            case Integer i              -> "zero";
            case String s when s.isEmpty() -> "empty string";
            case String s               -> "non-empty string";
            case null                   -> "null value";
            default                     -> "unhandled type";
        };
    }
}
