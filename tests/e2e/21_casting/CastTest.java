public class CastTest {
    public static void main(String[] args) {
        // i2b: int → byte (truncate to 8-bit signed)
        System.out.println((byte) 300);    // 44
        System.out.println((byte) -1);     // -1
        System.out.println((byte) 128);    // -128

        // i2s: int → short (truncate to 16-bit signed)
        System.out.println((short) 70000);  // 4464
        System.out.println((short) -1);     // -1

        // i2c: int → char (truncate to 16-bit unsigned)
        System.out.println((int)(char) -1);       // 65535
        System.out.println((char) 65601);          // A (65601 & 0xFFFF = 65, 'A')

        // l2i: long → int (truncate to 32-bit signed)
        System.out.println((int) 4294967338L);  // 42
        System.out.println((int) -1L);          // -1

        // f2i: float → int (truncate toward zero)
        System.out.println((int) 3.99f);   // 3

        // d2i: double → int (truncate toward zero)
        System.out.println((int) -2.7);    // -2

        // Widening: i2f, i2d (should preserve value)
        System.out.println((float) 42);    // 42.0
        System.out.println((double) 42);   // 42.0

        // Chain: long → int → byte  (1000L → -24)
        System.out.println((byte)(int) 1000L);  // -24
    }
}
