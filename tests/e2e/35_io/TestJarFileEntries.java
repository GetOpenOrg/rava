import java.io.ByteArrayOutputStream;
import java.nio.charset.StandardCharsets;
import java.nio.file.Files;
import java.nio.file.Path;
import java.util.jar.Attributes;
import java.util.jar.JarEntry;
import java.util.jar.JarFile;
import java.util.jar.JarOutputStream;
import java.util.jar.Manifest;

/**
 * java.util.jar 通道（mybatis 的 jar 资源面，此前零覆盖）：JarOutputStream 写
 * （含 Manifest 属性）、JarFile 条目枚举/读取、Manifest 解析、jar 输入流读取。
 */
public class TestJarFileEntries {

    public static void main(String[] args) throws Exception {
        Path dir = Files.createTempDirectory("jar-e2e");
        Path jarPath = dir.resolve("t.jar");

        Manifest mf = new Manifest();
        mf.getMainAttributes().put(Attributes.Name.MANIFEST_VERSION, "1.0");
        mf.getMainAttributes().putValue("Implementation-Title", "rava-e2e");

        ByteArrayOutputStream bos = new ByteArrayOutputStream();
        try (JarOutputStream jos = new JarOutputStream(bos, mf)) {
            jos.putNextEntry(new JarEntry("res/app.txt"));
            jos.write("jar-content".getBytes(StandardCharsets.UTF_8));
            jos.closeEntry();
            jos.putNextEntry(new JarEntry("dir/nested.dat"));
            jos.write(new byte[] { 1, 2, 3 });
            jos.closeEntry();
        }
        Files.write(jarPath, bos.toByteArray());

        try (JarFile jf = new JarFile(jarPath.toFile())) {
            System.out.println("manifest-version="
                    + jf.getManifest().getMainAttributes().getValue("Implementation-Title"));

            JarEntry e = jf.getJarEntry("res/app.txt");
            System.out.println("entry=" + e.getName() + " size=" + e.getSize()
                    + " dir=" + e.isDirectory());

            byte[] buf = new byte[64];
            int n = jf.getInputStream(e).read(buf);
            System.out.println("content=" + new String(buf, 0, n, StandardCharsets.UTF_8));

            // 条目枚举（稳定排序后打印——Manifest 排最前）
            jf.stream().map(JarEntry::getName).sorted().forEach(nm -> System.out.println("item=" + nm));

            // 嵌套条目 + 缺失边界
            System.out.println("nested=" + jf.getJarEntry("dir/nested.dat").getSize());
            System.out.println("missing=" + (jf.getJarEntry("nope") == null));
        }

        Files.deleteIfExists(jarPath);
        Files.deleteIfExists(dir);
        System.out.println("done=true");
    }
}
