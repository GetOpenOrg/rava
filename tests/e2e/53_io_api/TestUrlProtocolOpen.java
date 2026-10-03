import java.io.ByteArrayOutputStream;
import java.io.File;
import java.io.FileOutputStream;
import java.io.IOException;
import java.io.InputStream;
import java.io.OutputStream;
import java.net.MalformedURLException;
import java.net.URL;
import java.nio.charset.StandardCharsets;
import java.nio.file.Files;
import java.util.jar.JarEntry;
import java.util.jar.JarOutputStream;
import java.util.jar.Manifest;

// 协议处理器按 URL 串协议名查找的边界：jar: / file: / 大写与 url: 前缀 / 相对上下文 URL 读取真实文件，
// 未知协议与不带协议的串抛 MalformedURLException（处理器按协议键放行、URL.handler 按对象区分时不得丢失分支）
public class TestUrlProtocolOpen {
    static String read(URL u) throws IOException {
        try (InputStream in = u.openStream()) {
            ByteArrayOutputStream out = new ByteArrayOutputStream();
            in.transferTo(out);
            return out.toString(StandardCharsets.UTF_8);
        }
    }

    static void open(String label, URL u) {
        try {
            System.out.println(label + ": " + u.getProtocol() + " [" + read(u) + "]");
        } catch (IOException e) {
            System.out.println(label + ": " + e.getClass().getSimpleName());
        }
    }

    static void parse(String spec) {
        try {
            URL u = new URL(spec);
            System.out.println("parsed " + u.getProtocol());
        } catch (MalformedURLException e) {
            String m = e.getMessage();
            System.out.println("MalformedURLException: " + m.substring(0, m.indexOf(':')));
        }
    }

    public static void main(String[] args) throws Exception {
        File dir = Files.createTempDirectory("urlproto").toFile();
        File jar = new File(dir, "data.jar");
        Manifest mf = new Manifest();
        mf.getMainAttributes().putValue("Manifest-Version", "1.0");
        try (JarOutputStream jos = new JarOutputStream(new FileOutputStream(jar), mf)) {
            jos.putNextEntry(new JarEntry("dir/x.txt"));
            jos.write("hello from jar".getBytes(StandardCharsets.UTF_8));
            jos.closeEntry();
            jos.putNextEntry(new JarEntry("dir/y.txt"));
            jos.write("sibling entry".getBytes(StandardCharsets.UTF_8));
            jos.closeEntry();
        }
        File txt = new File(dir, "plain.txt");
        try (OutputStream os = new FileOutputStream(txt)) {
            os.write("plain file".getBytes(StandardCharsets.UTF_8));
        }
        String jarPath = jar.toURI().getPath();
        String txtPath = txt.toURI().getPath();

        URL inJar = new URL("jar:file:" + jarPath + "!/dir/x.txt");
        open("jar", inJar);
        open("jar-relative", new URL(inJar, "y.txt"));
        open("jar-missing", new URL("jar:file:" + jarPath + "!/dir/none.txt"));
        open("file", new URL("file:" + txtPath));
        open("file-upper", new URL("  URL:FILE:" + txtPath));
        open("file-parts", new URL("file", "", txtPath));
        parse("nosuchproto://host/x");
        parse("no-scheme/path");
        parse("#frag:x");

        jar.delete();
        txt.delete();
        dir.delete();
    }
}
