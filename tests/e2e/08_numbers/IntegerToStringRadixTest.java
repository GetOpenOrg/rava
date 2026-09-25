public class IntegerToStringRadixTest {
    public static void main(String[] args) {
        // Binary
        System.out.println(Integer.toString(255, 2));
        // Octal
        System.out.println(Integer.toString(255, 8));
        // Decimal
        System.out.println(Integer.toString(255, 10));
        // Hex
        System.out.println(Integer.toString(255, 16));
        // Base 36
        System.out.println(Integer.toString(35, 36));
        // Negative
        System.out.println(Integer.toString(-42, 16));
        // Zero
        System.out.println(Integer.toString(0, 16));
    }
}
