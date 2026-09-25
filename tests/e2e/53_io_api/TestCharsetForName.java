import java.io.ByteArrayOutputStream;
import java.io.PrintStream;
import java.nio.charset.Charset;
import java.nio.charset.IllegalCharsetNameException;
import java.nio.charset.StandardCharsets;
import java.nio.charset.UnsupportedCharsetException;

/**
 * Charset.forName → sun.nio.cs.StandardCharsets（标准 provider）的名字 / 别名查找：规范名、
 * 别名、大小写变体、九个标准 charset 全集、与 StandardCharsets 常量 equals、isSupported、
 * 未知名 → UnsupportedCharsetException、非法名 → IllegalCharsetNameException、
 * 按名构造 PrintStream（TestFileOutputStream 揭出：Charset.<clinit> 构造 provider）。
 */
public class TestCharsetForName {
    public static void main(String[] args) throws Exception {
        String[] names = {"UTF-8", "utf8", "Unicode-1-1-UTF-8", "ISO-8859-1", "latin1", "iso8859_1",
                "US-ASCII", "ascii", "UTF-16", "UTF-16BE", "utf-16le", "UTF-32", "UTF-32BE", "UTF-32LE"};
        for (String n : names) {
            Charset cs = Charset.forName(n);
            System.out.println(n + " -> " + cs.name());
        }
        System.out.println(Charset.forName("utf-8").equals(StandardCharsets.UTF_8));
        System.out.println(Charset.forName("ISO-8859-1").equals(StandardCharsets.ISO_8859_1));
        System.out.println(Charset.forName("UTF-8") == Charset.forName("UTF-8"));
        System.out.println(Charset.isSupported("UTF-16LE") + " " + Charset.isSupported("US-ASCII"));
        try {
            Charset.forName("x-no-such-charset");
        } catch (UnsupportedCharsetException e) {
            System.out.println("UCE: " + e.getCharsetName());
        }
        try {
            Charset.forName("bad name!");
        } catch (IllegalCharsetNameException e) {
            System.out.println("ICNE: " + e.getCharsetName());
        }
        ByteArrayOutputStream bo = new ByteArrayOutputStream();
        try (PrintStream ps = new PrintStream(bo, true, "UTF-16BE")) {
            ps.print("Hi");
        }
        byte[] b = bo.toByteArray();
        StringBuilder sb = new StringBuilder();
        for (byte x : b) sb.append(x).append(' ');
        System.out.println(sb.toString().trim());
    }
}
