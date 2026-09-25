public class StringFormatTest {
    public static void main(String[] args) {
        // Basic conversions
        System.out.println(String.format("int: %d", 42));
        System.out.println(String.format("float: %.2f", 3.14159));
        System.out.println(String.format("string: %s", "hello"));
        System.out.println(String.format("char: %c", 65));
        System.out.println(String.format("bool: %b", true));
        System.out.println(String.format("null bool: %b", (Object) null));
        System.out.println(String.format("hex: %x", 255));
        System.out.println(String.format("HEX: %X", 255));
        System.out.println(String.format("oct: %o", 8));
        System.out.println(String.format("sci: %.2e", 12345.6789));
        System.out.println(String.format("SCI: %.2E", 12345.6789));
        System.out.println(String.format("pct: 100%%"));
        System.out.println(String.format("newline: a%nb"));

        // Width and alignment
        System.out.println(String.format("[%10d]", 42));
        System.out.println(String.format("[%-10d]", 42));
        System.out.println(String.format("[%010d]", 42));
        System.out.println(String.format("[%10s]", "hi"));
        System.out.println(String.format("[%-10s]", "hi"));

        // Flags
        System.out.println(String.format("[%+d]", 42));
        System.out.println(String.format("[%+d]", -42));
        System.out.println(String.format("[% d]", 42));
        System.out.println(String.format("[%#x]", 255));
        System.out.println(String.format("[%#o]", 8));

        // Position arguments
        System.out.println(String.format("%2$s %1$s", "world", "hello"));

        // Multiple args
        System.out.println(String.format("%s=%d (%.1f%%)", "score", 95, 95.5));

        System.out.println("Done.");
    }
}
