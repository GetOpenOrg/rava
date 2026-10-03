import java.io.File;
import java.nio.file.Files;
import java.nio.file.Path;

/**
 * File 规范路径族（方法级实测：getCanonicalPath 9 / getCanonicalFile 8 / lastModified 8 /
 * getAbsoluteFile 6 / mkdir 5，此前零覆盖）：.. 归一化等价、布尔断言为主
 * （macOS /tmp→/private/tmp 符号链接使 canonical 与 absolute 前缀不同，不打印路径本体）。
 */
public class TestFileCanonicalPaths {

    public static void main(String[] args) throws Exception {
        Path dir = Files.createTempDirectory("canon");
        File root = dir.toFile();
        File f = new File(root, "x.txt");
        f.createNewFile();

        // .. 与规范路径等价
        File detour = new File(root, "sub/../x.txt");
        System.out.println("dotdot-eq=" + f.getCanonicalPath().equals(detour.getCanonicalPath()));
        System.out.println("canon-file-name=" + detour.getCanonicalFile().getName());

        // absolute 与 canonical 同文件身份（不打印本体）
        System.out.println("abs-parent-match=" + f.getAbsoluteFile().getParentFile()
                .equals(root.getAbsoluteFile()));
        System.out.println("canon-endswith=" + f.getCanonicalPath().endsWith("x.txt"));

        // lastModified：单调性布尔（不打印时刻）
        long t1 = f.lastModified();
        File f2 = new File(root, "y.txt");
        f2.createNewFile();
        System.out.println("mtime-positive=" + (t1 > 0)
                + " fresh-positive=" + (f2.lastModified() >= t1));

        // mkdir：已存在目录返回 false；父链缺失 mkdir 失败
        System.out.println("mkdir-existing=" + root.mkdir());
        File orphan = new File(root, "no/such/leaf");
        System.out.println("mkdir-orphan=" + orphan.mkdir());

        // isFile/isAbsolute 族
        System.out.println("isfile=" + f.isFile() + " isabs=" + new File(f.getAbsolutePath()).isAbsolute());

        f.delete();
        f2.delete();
        root.delete();
        System.out.println("cleaned=" + !root.exists());
    }
}
