import java.io.InputStream;
import java.net.URI;
import java.net.URL;
import java.nio.charset.StandardCharsets;
import java.nio.file.Files;
import java.nio.file.Path;

/**
 * URL 解析族（方法级实测：openStream 被 36 jar 引用、getProtocol/toExternalForm/
 * getFile 各 14-16 jar，此前零覆盖）：解析五件套、URI.toURL、file: 形态的
 * openStream（本地可测通道）。openConnection 网络面归 K9。
 */
public class TestUrlParsingFaces {

    public static void main(String[] args) throws Exception {
        URL u = new URL("https://example.com:8443/a/b?q=1#frag");
        System.out.println("protocol=" + u.getProtocol());
        System.out.println("host=" + u.getHost());
        System.out.println("port=" + u.getPort() + " default-port=" + u.getDefaultPort());
        System.out.println("path=" + u.getPath());
        System.out.println("query=" + u.getQuery());
        System.out.println("ref=" + u.getRef());
        System.out.println("external=" + u.toExternalForm());
        System.out.println("file=" + u.getFile());

        // 默认端口 -1（未写明时）；写明时取写明值
        URL noPort = new URL("http://example.com/x");
        System.out.println("no-port=" + noPort.getPort() + " backfill=" + noPort.getDefaultPort());

        // URI.toURL
        URL fromUri = URI.create("https://example.com/z").toURL();
        System.out.println("uri-to-url=" + fromUri.getPath());

        // file: URL 的 openStream：本地读（36 jar 用法的可测形态）
        Path tmp = Files.createTempFile("url-e2e", ".txt");
        Files.write(tmp, "via-file-url".getBytes(StandardCharsets.UTF_8));
        URL fileUrl = tmp.toUri().toURL();
        System.out.println("file-scheme=" + fileUrl.getProtocol());
        try (InputStream in = fileUrl.openStream()) {
            System.out.println("openstream=" + new String(in.readAllBytes(), StandardCharsets.UTF_8));
        }
        Files.deleteIfExists(tmp);

        // URL 的 equals/hashCode 规范形态（与路径无关部分）
        URL same = new URL("https://example.com:8443/a/b?q=1#frag");
        System.out.println("eq=" + u.equals(same) + " hash-eq=" + (u.hashCode() == same.hashCode()));
        // fragment 不参与 equals
        URL noFrag = new URL("https://example.com:8443/a/b?q=1");
        System.out.println("frag-independent=" + u.equals(noFrag));
    }
}
