public class TypeCastingTest {
    public static void main(String[] args) {
        // Test 1: Widening conversions
        byte b = 42;
        short s = b;
        int i = s;
        long l = i;
        float f = l;
        double d = f;
        System.out.println(d);  // 42.0

        // Test 2: Narrowing conversions
        double d2 = 3.99;
        int i2 = (int) d2;
        System.out.println(i2);  // 3

        long l2 = 1234567890123L;
        int i3 = (int) l2;
        System.out.println(i3);  // 1912276171

        int i4 = 300;
        byte b2 = (byte) i4;
        System.out.println(b2);  // 44

        // Test 3: char conversions
        char c = 'A';
        int ci = c;
        System.out.println(ci);  // 65
        char c2 = (char) 66;
        System.out.println(c2);  // B

        // Test 4: String to number parsing
        int parsed = Integer.parseInt("123");
        System.out.println(parsed);  // 123
        double parsedD = Double.parseDouble("3.14");
        System.out.println(parsedD);  // 3.14
        long parsedL = Long.parseLong("9876543210");
        System.out.println(parsedL);  // 9876543210

        // Test 5: Number/Boolean to String
        String si = Integer.toString(42);
        System.out.println(si);  // 42
        String sd = Double.toString(3.14);
        System.out.println(sd);  // 3.14
        String fromVal = String.valueOf(true);
        System.out.println(fromVal);  // true

        // Test 6: Integer overflow wrapping
        int maxInt = Integer.MAX_VALUE;
        int overflow = maxInt + 1;
        System.out.println(overflow);  // -2147483648
        System.out.println(overflow == Integer.MIN_VALUE);  // true

        // Test 7: Autoboxing / unboxing
        Integer boxed = 42;
        int unboxed = boxed;
        System.out.println(unboxed);  // 42

        // Test 8: Double to int precision
        System.out.println((int) 1.9);    // 1
        System.out.println((int) -1.9);   // -1
        System.out.println((int) 0.5);    // 0

        // Test 9: Mixed arithmetic type promotion
        int a = 5;
        double bd = 2.5;
        double result = a + bd;
        System.out.println(result);  // 7.5

        int x = 7;
        int y = 2;
        System.out.println(x / y);     // 3 (integer division)
        System.out.println((double)x / y);  // 3.5
    }
}
