public class StringMethodsTest {
    public static void main(String[] args) {
        // Test 1: Basic string operations
        String s = "Hello, World!";
        System.out.println(s.length());           // 13
        System.out.println(s.charAt(0));           // H
        System.out.println(s.indexOf("World"));    // 7
        System.out.println(s.substring(7));        // World!
        System.out.println(s.substring(0, 5));     // Hello

        // Test 2: Case conversion
        System.out.println("hello".toUpperCase());  // HELLO
        System.out.println("WORLD".toLowerCase());  // world

        // Test 3: Trim and strip
        System.out.println("  hello  ".trim());     // hello
        System.out.println("  hello  ".strip());    // hello

        // Test 4: Contains, startsWith, endsWith
        System.out.println(s.contains("World"));    // true
        System.out.println(s.contains("world"));    // false
        System.out.println(s.startsWith("Hello"));  // true
        System.out.println(s.endsWith("!"));        // true

        // Test 5: Replace
        System.out.println(s.replace("World", "Java"));  // Hello, Java!
        System.out.println("aabaa".replace('a', 'b'));    // bbbbb

        // Test 6: Split
        String csv = "one,two,three";
        String[] parts = csv.split(",");
        System.out.println(parts.length);  // 3
        System.out.println(parts[0]);      // one
        System.out.println(parts[2]);      // three

        // Test 7: Join
        System.out.println(String.join("-", "a", "b", "c"));  // a-b-c

        // Test 8: Comparisons
        System.out.println("abc".equals("abc"));    // true
        System.out.println("abc".equals("ABC"));    // false
        System.out.println("abc".equalsIgnoreCase("ABC"));  // true
        System.out.println("abc".compareTo("abd")); // -1

        // Test 9: isEmpty and isBlank
        System.out.println("".isEmpty());      // true
        System.out.println(" ".isEmpty());     // false
        System.out.println("  ".isBlank());    // true
        System.out.println("hi".isBlank());    // false

        // Test 10: Repeat
        System.out.println("ab".repeat(3));    // ababab

        // Test 11: StringBuilder reverse
        StringBuilder sb = new StringBuilder("Hello");
        sb.reverse();
        System.out.println(sb.toString());  // olleH

        // Test 12: Integer parsing
        int num = Integer.parseInt("42");
        System.out.println(num + 8);              // 50

        // Test 13: String.format
        System.out.println(String.format("%s is %d", "age", 25));  // age is 25
        System.out.println(String.format("%.2f", 3.14159));        // 3.14

        // Test 14: lastIndexOf
        System.out.println("abcabc".lastIndexOf("bc"));  // 4
        System.out.println("abcabc".indexOf("bc"));      // 1

        // Test 15: String concatenation with +
        String a = "Hello";
        String b = " World";
        String c = a + b;
        System.out.println(c);  // Hello World
    }
}
