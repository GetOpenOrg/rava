import java.io.BufferedReader;
import java.io.ByteArrayInputStream;
import java.io.ByteArrayOutputStream;
import java.io.InputStreamReader;
import java.io.OutputStreamWriter;
import java.io.PrintStream;
import java.io.UnsupportedEncodingException;
import java.io.Writer;
import java.util.Arrays;

// 按字符集名构造的 Reader / Writer（StreamDecoder / StreamEncoder 的 String 重载）
public class TestCharsetNamedStreams {
    static String hex(byte[] b) {
        StringBuilder sb = new StringBuilder();
        for (byte x : b) sb.append(String.format("%02x", x & 0xff));
        return sb.toString();
    }

    public static void main(String[] args) throws Exception {
        String text = "héllo 世界 😀";
        for (String name : new String[]{"UTF-8", "utf8", "ISO-8859-1", "latin1", "US-ASCII", "UTF-16", "UTF-16LE"}) {
            ByteArrayOutputStream bos = new ByteArrayOutputStream();
            try (Writer w = new OutputStreamWriter(bos, name)) {
                w.write(text);
            }
            byte[] bytes = bos.toByteArray();
            System.out.println(name + " bytes=" + hex(bytes));
            try (BufferedReader r = new BufferedReader(new InputStreamReader(new ByteArrayInputStream(bytes), name))) {
                System.out.println(name + " read=" + r.readLine());
            }
        }
        ByteArrayOutputStream bos = new ByteArrayOutputStream();
        PrintStream ps = new PrintStream(bos, true, "UTF-8");
        ps.print("ps:é");
        ps.flush();
        System.out.println("printstream=" + hex(bos.toByteArray()));
        for (String bad : new String[]{"NO-SUCH-CHARSET", "bad name!"}) {
            try {
                new InputStreamReader(new ByteArrayInputStream(new byte[0]), bad);
                System.out.println("unexpected");
            } catch (UnsupportedEncodingException e) {
                System.out.println("reader UEE: " + e.getMessage());
            }
            try {
                new OutputStreamWriter(new ByteArrayOutputStream(), bad);
                System.out.println("unexpected");
            } catch (UnsupportedEncodingException e) {
                System.out.println("writer UEE: " + e.getMessage());
            }
        }
        InputStreamReader isr = new InputStreamReader(new ByteArrayInputStream("abc".getBytes("UTF-8")), "UTF-8");
        System.out.println("encoding=" + isr.getEncoding());
        char[] buf = new char[8];
        int n = isr.read(buf);
        System.out.println("n=" + n + " " + new String(buf, 0, n) + " " + Arrays.toString("é".getBytes("ISO-8859-1")));
        isr.close();
        System.out.println("closed encoding=" + isr.getEncoding());
        for (String name : new String[]{"ISO-8859-1", "US-ASCII", "UTF-16BE", "UTF-16LE", "UTF-32"}) {
            OutputStreamWriter osw = new OutputStreamWriter(new ByteArrayOutputStream(), name);
            System.out.println(name + " writer encoding=" + osw.getEncoding());
        }
    }
}
