import java.io.ByteArrayOutputStream;
import java.io.InputStream;
import java.net.URI;
import java.net.URL;
import java.net.URLConnection;

/**
 * 资源 URL 的字符串往返：getResource 得到的 URL 经 toString 后按字符串重建
 * （new URL(String)、URI.create(String).toURL()），重建的 URL 照常 openStream /
 * openConnection().getInputStream() 读到与 getResourceAsStream 相同的内容
 * （日志框架记下配置文件 URL 再重开、URL 存成字符串后再读、ServiceLoader 风格的清单查找）。
 * 只打印内容判定，不打印 URL 本身（协议随运行环境而异）。
 */
public class TestEmbeddedUrlRebuild {

    static class Inner {
    }

    static byte[] readAll(InputStream in) throws Exception {
        try (InputStream s = in) {
            ByteArrayOutputStream out = new ByteArrayOutputStream();
            byte[] buf = new byte[512];
            int n;
            while ((n = s.read(buf)) > 0) {
                out.write(buf, 0, n);
            }
            return out.toByteArray();
        }
    }

    static String head(byte[] b) {
        return String.format("%02X%02X%02X%02X", b[0], b[1], b[2], b[3]);
    }

    static void check(String tag, Class<?> c, String name) throws Exception {
        byte[] expect = readAll(c.getResourceAsStream(name));
        URL url = c.getResource(name);
        System.out.println(tag + " found=" + (url != null) + " head=" + head(expect));

        // new URL(String)：按协议名查找处理器
        @SuppressWarnings("deprecation")
        URL rebuilt = new URL(url.toString());
        byte[] viaStream = readAll(rebuilt.openStream());
        System.out.println(tag + " rebuilt-same-string=" + url.toString().equals(rebuilt.toString())
                + " rebuilt-eq=" + java.util.Arrays.equals(expect, viaStream));

        // URI.create(String).toURL()：URL.of 同经协议名查找处理器
        URL viaUri = URI.create(url.toString()).toURL();
        URLConnection conn = viaUri.openConnection();
        byte[] viaConn = readAll(conn.getInputStream());
        System.out.println(tag + " uri-eq=" + java.util.Arrays.equals(expect, viaConn)
                + " length-eq=" + (conn.getContentLengthLong() == expect.length));
    }

    public static void main(String[] args) throws Exception {
        check("self", TestEmbeddedUrlRebuild.class, "TestEmbeddedUrlRebuild.class");
        check("inner", Inner.class, "TestEmbeddedUrlRebuild$Inner.class");

        // 系统资源：JDK 自身类文件
        URL jdk = ClassLoader.getSystemResource("java/lang/String.class");
        @SuppressWarnings("deprecation")
        URL jdkRebuilt = new URL(jdk.toString());
        byte[] jdkBytes = readAll(jdkRebuilt.openStream());
        System.out.println("jdk head=" + head(jdkBytes)
                + " eq=" + java.util.Arrays.equals(jdkBytes, readAll(ClassLoader.getSystemResourceAsStream("java/lang/String.class"))));
    }
}
