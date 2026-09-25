public class StringMethods {
    public static void main(String[] args) {
        String s = "  Hello World  ";
        System.out.println(s.trim());              // Hello World
        System.out.println(s.strip());             // Hello World
        String stripped = s.stripLeading();
        System.out.println(stripped.length());     // 13 (Hello World  )
        System.out.println(s.isBlank());           // false
        String blank = "   ";
        System.out.println(blank.isBlank());       // true
        String ha = "ha";
        System.out.println(ha.repeat(3));          // hahaha
        String hello = "hello";
        System.out.println(hello.contains("ell")); // true
        System.out.println(hello.startsWith("hel")); // true
        System.out.println(hello.endsWith("lo"));    // true
        String world = "world";
        System.out.println(hello.startsWith(world)); // false
    }
}
