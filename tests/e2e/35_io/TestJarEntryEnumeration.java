import java.io.ByteArrayOutputStream;
import java.util.Enumeration;
import java.util.jar.Attributes;
import java.util.jar.JarEntry;
import java.util.jar.JarFile;
import java.util.jar.Manifest;

/**
 * JarFile.entries 枚举与条目级 Manifest 属性（方法级实测：entries 10 jar，
 * 此前只测 stream() 形态）：Manifest 排首的枚举序、条目属性读写。
 */
public class TestJarEntryEnumeration {

    public static void main(String[] args) throws Exception {
        var files = java.nio.file.Files.createTempDirectory("jar-enum");
        var jarPath = files.resolve("e.jar");

        Manifest mf = new Manifest();
        mf.getMainAttributes().put(Attributes.Name.MANIFEST_VERSION, "1.0");
        Attributes entryAttrs = new Attributes();
        entryAttrs.putValue("Content-Type", "text/plain");
        mf.getEntries().put("res/a.txt", entryAttrs);

        ByteArrayOutputStream bos = new ByteArrayOutputStream();
        try (var jos = new java.util.jar.JarOutputStream(bos, mf)) {
            jos.putNextEntry(new JarEntry("res/a.txt"));
            jos.write('a');
            jos.closeEntry();
            jos.putNextEntry(new JarEntry("res/b.txt"));
            jos.write('b');
            jos.closeEntry();
        }
        java.nio.file.Files.write(jarPath, bos.toByteArray());

        try (JarFile jf = new JarFile(jarPath.toFile())) {
            // entries() 枚举：META-INF/MANIFEST.MF 最先，其余按写入序
            StringBuilder order = new StringBuilder();
            int count = 0;
            for (Enumeration<JarEntry> e = jf.entries(); e.hasMoreElements();) {
                if (count > 0) {
                    order.append(",");
                }
                order.append(e.nextElement().getName());
                count++;
            }
            System.out.println("count=" + count);
            System.out.println("order=" + order);

            // 条目级属性
            Attributes got = jf.getManifest().getAttributes("res/a.txt");
            System.out.println("entry-attr=" + got.getValue("Content-Type"));
            System.out.println("b-no-attrs=" + (jf.getManifest().getAttributes("res/b.txt") == null));
            System.out.println("main-ver=" + jf.getManifest().getMainAttributes()
                    .getValue(Attributes.Name.MANIFEST_VERSION));

            // getManifest 的 entries 键集
            System.out.println("manifest-entry-keys=" + jf.getManifest().getEntries().size());
        }

        java.nio.file.Files.deleteIfExists(jarPath);
        java.nio.file.Files.deleteIfExists(files);
        System.out.println("done=true");
    }
}
