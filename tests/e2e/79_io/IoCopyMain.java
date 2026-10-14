
import java.io.ByteArrayInputStream;
import java.io.ByteArrayOutputStream;
import java.io.IOException;
import java.nio.charset.StandardCharsets;

import org.apache.commons.io.IOUtils;
import org.apache.commons.io.input.CharSequenceReader;

/**
 * commons-io pilot A16：IOUtils 内存流拷贝往返（W1 #5）。
 */
public class IoCopyMain {

    static void check(String name, Object expected, Object actual) {
        boolean ok = String.valueOf(expected).equals(String.valueOf(actual));
        System.out.println((ok ? "PASS " : "FAIL ") + name
                + (ok ? "" : " :: expected=<" + expected + "> actual=<" + actual + ">"));
    }

    public static void main(String[] args) throws IOException {
        byte[] data = "你好, commons-io".getBytes(StandardCharsets.UTF_8);

        // toByteArray：InputStream → byte[]
        byte[] copied = IOUtils.toByteArray(new ByteArrayInputStream(data));
        check("tobytearray-len", data.length, copied.length);
        check("tobytearray-eq", true, java.util.Arrays.equals(data, copied));

        // copy 大块：带缓冲拷贝字节数
        ByteArrayOutputStream out = new ByteArrayOutputStream();
        long n = IOUtils.copyLarge(new ByteArrayInputStream(data), out);
        check("copylarge-count", data.length, n);

        // toString：指定字符集
        String text = IOUtils.toString(new ByteArrayInputStream(data), StandardCharsets.UTF_8);
        check("tostring-utf8", "你好, commons-io", text);

        // 字符流拷贝（CharSequenceReader 免文件）
        StringBuilder received = new StringBuilder();
        long chars = IOUtils.copyLarge(new CharSequenceReader("stream-copy"), new java.io.Writer() {
            @Override
            public void write(char[] cbuf, int off, int len) {
                received.append(cbuf, off, len);
            }

            @Override
            public void flush() {
            }

            @Override
            public void close() {
            }
        });
        check("char-copy", "11:stream-copy", chars + ":" + received);

        // write：String → OutputStream
        ByteArrayOutputStream out2 = new ByteArrayOutputStream();
        IOUtils.write("written", out2, StandardCharsets.UTF_8);
        check("write-bytes", "written", out2.toString(StandardCharsets.UTF_8));

        // readFully：精确读 N 字节
        byte[] head = new byte[3];
        IOUtils.readFully(new ByteArrayInputStream(data), head);
        check("readfully", true, java.util.Arrays.equals(
                java.util.Arrays.copyOf(data, 3), head));
        System.out.println("DONE IoCopyMain");
    }
}
