import java.util.zip.Adler32;
import java.util.zip.CRC32;
import java.util.zip.CRC32C;

public class ChecksumTest {
    public static void main(String[] args) {
        // Test 1: Adler32 basic
        Adler32 adler = new Adler32();
        System.out.println("Adler32 init: " + adler.getValue());

        // Test 2: Adler32 update single byte
        adler.update(1);
        System.out.println("Adler32 after 1: " + adler.getValue());

        // Test 3: Adler32 update byte array
        adler.reset();
        byte[] data = "Hello".getBytes();
        adler.update(data, 0, data.length);
        System.out.println("Adler32 Hello: " + adler.getValue());

        // Test 4: Adler32 reset
        adler.reset();
        System.out.println("Adler32 reset: " + adler.getValue());

        // Test 5: CRC32 basic
        CRC32 crc = new CRC32();
        System.out.println("CRC32 init: " + crc.getValue());

        // Test 6: CRC32 update single byte
        crc.update(1);
        System.out.println("CRC32 after 1: " + crc.getValue());

        // Test 7: CRC32 update byte array
        crc.reset();
        crc.update(data, 0, data.length);
        System.out.println("CRC32 Hello: " + crc.getValue());

        // Test 8: CRC32 reset
        crc.reset();
        System.out.println("CRC32 reset: " + crc.getValue());

        // Test 9: Longer data
        Adler32 a2 = new Adler32();
        CRC32 c2 = new CRC32();
        byte[] longer = "The quick brown fox jumps over the lazy dog".getBytes();
        a2.update(longer, 0, longer.length);
        c2.update(longer, 0, longer.length);
        System.out.println("Adler32 fox: " + a2.getValue());
        System.out.println("CRC32 fox: " + c2.getValue());

        // Test 10: CRC32C basic
        CRC32C crc32c = new CRC32C();
        System.out.println("CRC32C init: " + crc32c.getValue());

        // Test 11: CRC32C update single byte
        crc32c.update(1);
        System.out.println("CRC32C after 1: " + crc32c.getValue());

        // Test 12: CRC32C update byte array
        crc32c.reset();
        byte[] data2 = "Hello".getBytes();
        crc32c.update(data2, 0, data2.length);
        System.out.println("CRC32C Hello: " + crc32c.getValue());

        // Test 13: CRC32C reset
        crc32c.reset();
        System.out.println("CRC32C reset: " + crc32c.getValue());
    }
}
