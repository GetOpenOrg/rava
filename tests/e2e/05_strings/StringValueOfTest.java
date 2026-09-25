public class StringValueOfTest {
    public static void main(String[] args) {
        // Test 1: String.valueOf(boolean)
        System.out.println(String.valueOf(true));   // true
        System.out.println(String.valueOf(false));  // false

        // Test 2: String.valueOf(int)
        System.out.println(String.valueOf(42));     // 42
        System.out.println(String.valueOf(-1));     // -1

        // Test 3: String.valueOf(long)
        System.out.println(String.valueOf(123456789L));  // 123456789

        // Test 4: String.valueOf(double)
        System.out.println(String.valueOf(3.14));   // 3.14

        // Test 5: String.valueOf(char)
        System.out.println(String.valueOf('A'));    // A
        System.out.println(String.valueOf('Z'));    // Z

        // Test 6: Boolean in string concatenation
        boolean flag = true;
        System.out.println("flag=" + flag);    // flag=true

        // Test 7: String.valueOf with boolean variable
        boolean b = false;
        String s = String.valueOf(b);
        System.out.println(s);  // false

        // Test 8: Boolean.toString
        System.out.println(Boolean.toString(true));   // true
        System.out.println(Boolean.toString(false));  // false

        // Test 9: Mixed valueOf calls
        System.out.println(String.valueOf(0));       // 0
        System.out.println(String.valueOf(true));    // true
        System.out.println(String.valueOf(1.0));     // 1.0
    }
}
