public class StringBuilderAdvancedTest {
    public static void main(String[] args) {
        // Test 1: Basic StringBuilder operations
        StringBuilder sb = new StringBuilder();
        sb.append("Hello");
        sb.append(" ");
        sb.append("World");
        System.out.println(sb.toString());  // Hello World

        // Test 2: StringBuilder with different types
        StringBuilder sb2 = new StringBuilder();
        sb2.append("int=");
        sb2.append(42);
        sb2.append(" double=");
        sb2.append(3.14);
        sb2.append(" str=");
        sb2.append("abc");
        System.out.println(sb2.toString());  // int=42 double=3.14 str=abc

        // Test 3: StringBuilder chaining
        String result = new StringBuilder()
            .append("a")
            .append("b")
            .append("c")
            .toString();
        System.out.println(result);  // abc

        // Test 4: StringBuilder length
        StringBuilder sb3 = new StringBuilder("test");
        System.out.println(sb3.length());  // 4

        // Test 5: StringBuilder delete and append
        StringBuilder sb4 = new StringBuilder("Hello World");
        sb4.delete(5, 11);
        sb4.append(" Java");
        System.out.println(sb4.toString());  // Hello Java

        // Test 6: StringBuilder reverse
        StringBuilder sb5 = new StringBuilder("abcde");
        sb5.reverse();
        System.out.println(sb5.toString());  // edcba

        // Test 7: StringBuilder replace
        StringBuilder sb6 = new StringBuilder("Hello World");
        sb6.replace(6, 11, "Java");
        System.out.println(sb6.toString());  // Hello Java

        // Test 8: StringBuilder charAt and indexOf
        StringBuilder sb7 = new StringBuilder("Hello");
        System.out.println(sb7.charAt(1));   // e
        System.out.println(sb7.indexOf("ll")); // 2

        // Test 9: Build string in loop
        StringBuilder sb8 = new StringBuilder();
        for (int i = 0; i < 5; i++) {
            if (i > 0) sb8.append(",");
            sb8.append(i);
        }
        System.out.println(sb8.toString());  // 0,1,2,3,4

        // Test 10: StringBuilder substring
        StringBuilder sb9 = new StringBuilder("Hello World");
        System.out.println(sb9.substring(6));     // World
        System.out.println(sb9.substring(0, 5));  // Hello
    }
}
