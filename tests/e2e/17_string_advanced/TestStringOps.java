public class TestStringOps {
    public static void main(String[] args) {
        String s = "  Hello, World!  ";
        System.out.println(s.trim());
        System.out.println(s.strip());
        System.out.println(s.trim().toLowerCase());
        System.out.println(s.trim().toUpperCase());
        System.out.println(s.trim().replace("World", "Java"));
        System.out.println(s.trim().contains("World"));
        System.out.println(s.trim().startsWith("Hello"));
        System.out.println(s.trim().endsWith("!"));
        System.out.println(s.trim().indexOf("o"));
        System.out.println(s.trim().substring(7));

        String[] parts = "a,b,c,d".split(",");
        System.out.println(parts.length);
        System.out.println(String.join("-", parts));

        System.out.println("abc".repeat(3));
        System.out.println("  ".isBlank());
        System.out.println("hello".isBlank());
    }
}
