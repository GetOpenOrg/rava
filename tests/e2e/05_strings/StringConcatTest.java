public class StringConcatTest {
    public static void main(String[] args) {
        // Basic string literal concatenation
        String hello = "Hello" + " " + "World";
        System.out.println(hello);

        // String + int
        int x = 42;
        String msg = "Value: " + x;
        System.out.println(msg);

        // String + String variable
        String a = "foo";
        String b = "bar";
        String c = a + b;
        System.out.println(c);

        // Multiple types
        double d = 3.14;
        boolean flag = true;
        String mixed = "pi=" + d + " flag=" + flag;
        System.out.println(mixed);

        // Concatenation in expression
        System.out.println("Result: " + (10 + 20));
    }
}
