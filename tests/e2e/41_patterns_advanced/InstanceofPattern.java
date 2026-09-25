public class InstanceofPattern {
    static void describe(Object obj) {
        if (obj instanceof String s) {
            System.out.println("String length: " + s.length());
            System.out.println(s.toUpperCase());
        } else if (obj instanceof Integer i) {
            System.out.println("Integer: " + i);
        } else {
            System.out.println("Unknown type");
        }
    }

    public static void main(String[] args) {
        describe("hello");
        describe(42);
        describe(3.14);
        describe("World");
        describe(42.0 * 2);
    }
}
