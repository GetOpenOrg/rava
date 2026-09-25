public class FormatOutput {
    public static void main(String[] args) {
        // Basic integer formatting
        String s1 = String.format("count=%d", 42);
        System.out.println(s1);

        // String substitution
        String s2 = String.format("hello %s", "world");
        System.out.println(s2);

        // Multiple args
        String s3 = String.format("%s has %d items", "list", 5);
        System.out.println(s3);
    }
}
