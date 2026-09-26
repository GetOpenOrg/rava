import java.io.ByteArrayOutputStream;
import java.io.OutputStreamWriter;
import java.io.PrintStream;
import java.io.Writer;
import java.nio.charset.Charset;

/**
 * 字符流按 charset 编码（sun.nio.cs.StreamEncoder 手写边界，TestCharsetForName 揭出：
 * 恒按 UTF-8 编码）。九个标准 charset × 含 BMP 非 ASCII（é、€）与增补字符（U+1F600 代理对）
 * 的文本：8 位编码不可映射 → '?'（代理对计一个字符）、UTF-16 首次写出 BOM、UTF-32 无 BOM
 * 大端、分两次写入时代理对跨写入边界、PrintStream 与 OutputStreamWriter 两个入口。
 */
public class TestStreamEncoderCharsets {
    static String hex(byte[] b) {
        StringBuilder sb = new StringBuilder();
        for (byte x : b) sb.append(String.format("%02x", x & 0xff));
        return sb.toString();
    }

    public static void main(String[] args) throws Exception {
        String text = "Hé€😀!";
        String[] names = {"UTF-8", "ISO-8859-1", "US-ASCII", "UTF-16", "UTF-16BE", "UTF-16LE",
                "UTF-32", "UTF-32BE", "UTF-32LE"};
        for (String n : names) {
            ByteArrayOutputStream bo = new ByteArrayOutputStream();
            try (PrintStream ps = new PrintStream(bo, true, n)) {
                ps.print(text);
            }
            System.out.println(n + " " + hex(bo.toByteArray()));
        }
        for (String n : new String[] {"UTF-16", "UTF-16LE", "UTF-8"}) {
            ByteArrayOutputStream bo = new ByteArrayOutputStream();
            try (Writer w = new OutputStreamWriter(bo, Charset.forName(n))) {
                w.write("a\uD83D");
                w.write("\uDE00b");
                w.write("c");
            }
            System.out.println("split " + n + " " + hex(bo.toByteArray()));
        }
    }
}
