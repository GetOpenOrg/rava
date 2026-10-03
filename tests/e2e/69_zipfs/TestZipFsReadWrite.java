import java.nio.file.ClosedFileSystemException;
import java.nio.file.FileSystem;
import java.nio.file.FileSystems;
import java.nio.file.Files;
import java.nio.file.Path;
import java.util.Map;

/**
 * jdk.zipfs：zip 文件系统的创建/写入/重读、URI 形态、已关闭系统的异常
 * （jmod 覆盖计划 A 档；临时目录下运行期自建、结束清理、不打印绝对路径）。
 */
public class TestZipFsReadWrite {

    public static void main(String[] args) throws Exception {
        Path dir = Files.createTempDirectory("zipfs-e2e");
        Path zip = dir.resolve("t.zip");

        try (FileSystem fs = FileSystems.newFileSystem(zip, Map.of("create", "true"))) {
            Path inside = fs.getPath("/a/b.txt");
            Files.createDirectories(inside.getParent());
            Files.writeString(inside, "hello-zipfs");
            Files.writeString(fs.getPath("/c.txt"), "second");
        }

        try (FileSystem fs = FileSystems.newFileSystem(zip, (ClassLoader) null)) {
            System.out.println("read=" + Files.readString(fs.getPath("/a/b.txt")));
            System.out.println("size=" + Files.size(fs.getPath("/c.txt")));
            System.out.println("isDir=" + Files.isDirectory(fs.getPath("/a")));
        }

        try (FileSystem fs = FileSystems.newFileSystem(
                java.net.URI.create("jar:" + zip.toUri()), Map.of())) {
            System.out.println("uri-read=" + Files.readString(fs.getPath("/a/b.txt")));
        }

        FileSystem fs2 = FileSystems.newFileSystem(zip, (ClassLoader) null);
        Path p2 = fs2.getPath("/c.txt");
        fs2.close();
        try {
            Files.readString(p2);
        } catch (ClosedFileSystemException e) {
            System.out.println("closed-ex=" + e.getClass().getSimpleName());
        }

        Files.deleteIfExists(zip);
        Files.deleteIfExists(dir);
        System.out.println("done=true");
    }
}
