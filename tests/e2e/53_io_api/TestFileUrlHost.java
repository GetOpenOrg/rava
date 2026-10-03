import java.io.ByteArrayOutputStream;
import java.io.File;
import java.io.FileOutputStream;
import java.io.IOException;
import java.io.InputStream;
import java.io.OutputStream;
import java.net.URL;
import java.net.URLConnection;
import java.nio.charset.StandardCharsets;
import java.nio.file.Files;

// file: URL 的 host 决定 file 处理器走本地文件还是改走 ftp 的边界：
// host 为 "" / "localhost"（大小写不敏感）/ "~" / null 时读本地文件；其余 host（含相对解析 "//主机/路径" 得到的）
// 由 file 处理器改建 ftp URL 再打开连接（只建连接对象、不联网）。闭包分析器对 URL.host 的逐对象精度不得丢失后一分支
public class TestFileUrlHost {
    static String read(URLConnection c) throws IOException {
        try (InputStream in = c.getInputStream()) {
            ByteArrayOutputStream out = new ByteArrayOutputStream();
            in.transferTo(out);
            return out.toString(StandardCharsets.UTF_8);
        }
    }

    static void open(String label, URL u) {
        try {
            URLConnection c = u.openConnection();
            String proto = c.getURL().getProtocol();
            if (proto.equals("file")) {
                System.out.println(label + ": host=[" + u.getHost() + "] local [" + read(c) + "]");
            } else {
                System.out.println(label + ": host=[" + u.getHost() + "] -> " + proto + " host=" + c.getURL().getHost()
                        + " ref=" + c.getURL().getRef() + " conn=" + c.getClass().getSimpleName());
            }
        } catch (IOException e) {
            System.out.println(label + ": " + e.getClass().getSimpleName());
        }
    }

    public static void main(String[] args) throws Exception {
        File dir = Files.createTempDirectory("urlhost").toFile();
        File txt = new File(dir, "plain.txt");
        try (OutputStream os = new FileOutputStream(txt)) {
            os.write("plain file".getBytes(StandardCharsets.UTF_8));
        }
        String path = txt.getAbsolutePath().replace(File.separatorChar, '/');
        URL base = dir.toURI().toURL();
        try {
            open("ctor-empty", new URL("file", "", path));
            open("ctor-localhost", new URL("file", "localhost", path));
            open("ctor-LOCALHOST", new URL("file", "LocalHost", path));
            open("ctor-tilde", new URL("file", "~", path));
            open("spec-plain", new URL("file:" + path));
            open("spec-authority-localhost", new URL("file://localhost" + path));
            open("to-url", txt.toURI().toURL());
            open("relative-plain", new URL(base, "plain.txt"));
            open("ctor-remote", new URL("file", "127.0.0.1", path));
            open("spec-remote", new URL("file://127.0.0.1" + path + "#frag"));
            String host = args.length > 0 ? args[0] : "example.invalid";
            open("relative-remote", new URL(base, "//" + host + path));
            open("ctor-remote-dynamic", new URL("file", host.toUpperCase(java.util.Locale.ROOT), path));
            // 本地连接对象：host 恒为常量时不联网也能读取
            URL again = new URL("file", "", path);
            System.out.println("same-file: " + again.sameFile(new URL("file:" + path)));
        } finally {
            txt.delete();
            dir.delete();
        }
    }
}
