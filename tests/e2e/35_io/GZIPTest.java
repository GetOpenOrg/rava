import java.io.ByteArrayOutputStream;
import java.util.zip.GZIPOutputStream;

public class GZIPTest {
    public static void main(String[] args) throws Exception {
        // Test 1: 基本 GZIP 压缩流程
        ByteArrayOutputStream baos = new ByteArrayOutputStream();
        GZIPOutputStream gzip = new GZIPOutputStream(baos);
        byte[] data = "Hello, GZIP!".getBytes();
        gzip.write(data, 0, data.length);
        gzip.finish();
        gzip.close();
        System.out.println("GZIP compress OK");

        // Test 2: 较长数据的压缩
        ByteArrayOutputStream baos2 = new ByteArrayOutputStream();
        GZIPOutputStream gzip2 = new GZIPOutputStream(baos2);
        String longText = "The quick brown fox jumps over the lazy dog. ";
        for (int i = 0; i < 10; i++) {
            byte[] chunk = longText.getBytes();
            gzip2.write(chunk, 0, chunk.length);
        }
        gzip2.finish();
        gzip2.close();
        System.out.println("GZIP long compress OK");

        // Test 3: ByteArrayOutputStream 基本操作
        ByteArrayOutputStream baos3 = new ByteArrayOutputStream();
        baos3.write(65); // 'A'
        baos3.write(66); // 'B'
        baos3.write(67); // 'C'
        System.out.println("BAOS size: " + baos3.size());
        System.out.println("BAOS content: " + baos3.toString());

        // Test 4: ByteArrayOutputStream write(byte[], off, len)
        ByteArrayOutputStream baos4 = new ByteArrayOutputStream();
        byte[] hello = "Hello World".getBytes();
        baos4.write(hello, 0, hello.length);
        System.out.println("BAOS4 size: " + baos4.size());
        System.out.println("BAOS4 content: " + baos4.toString());
    }
}
