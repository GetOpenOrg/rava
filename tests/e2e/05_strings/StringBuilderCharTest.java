public class StringBuilderCharTest {
    public static void main(String[] args) {
        // Test 1: append char from charAt
        String word = "Hello";
        StringBuilder sb = new StringBuilder();
        for (int i = 0; i < word.length(); i++) {
            sb.append(word.charAt(i));
        }
        System.out.println(sb.toString());  // Hello

        // Test 2: append char literal
        StringBuilder sb2 = new StringBuilder();
        sb2.append('A');
        sb2.append('B');
        sb2.append('C');
        System.out.println(sb2.toString());  // ABC

        // Test 3: Mixed append (char + string + int)
        StringBuilder sb3 = new StringBuilder();
        sb3.append("val=");
        sb3.append(42);
        sb3.append(',');
        sb3.append("ok");
        System.out.println(sb3.toString());  // val=42,ok

        // Test 4: Reverse string using charAt + append
        String original = "World";
        StringBuilder sb4 = new StringBuilder();
        for (int i = original.length() - 1; i >= 0; i--) {
            sb4.append(original.charAt(i));
        }
        System.out.println(sb4.toString());  // dlroW

        // Test 5: Append boolean and double
        StringBuilder sb6 = new StringBuilder();
        sb6.append(true);
        sb6.append(" ");
        sb6.append(3.14);
        System.out.println(sb6.toString());  // true 3.14
    }
}
