import java.nio.file.FileSystem;
import java.nio.file.FileSystems;
import java.nio.file.Files;
import java.nio.file.Path;
import java.nio.file.attribute.BasicFileAttributes;
import java.util.Map;

/**
 * jdk.zipfs：目录树 walk、条目属性、跨文件系统 Files 操作
 * （jmod 覆盖计划 A 档；walk 结果显式排序，输出与遍历顺序解耦）。
 */
public class TestZipFsWalk {

    public static void main(String[] args) throws Exception {
        Path dir = Files.createTempDirectory("zipfs-walk");
        Path zip = dir.resolve("w.zip");

        try (FileSystem fs = FileSystems.newFileSystem(zip, Map.of("create", "true"))) {
            Files.createDirectories(fs.getPath("/a"));
            Files.createDirectories(fs.getPath("/b/y"));
            Files.writeString(fs.getPath("/a/x.txt"), "xxx");
            Files.writeString(fs.getPath("/b/y/z.txt"), "zzz");
            Files.writeString(fs.getPath("/top.txt"), "ttt");
        }

        try (FileSystem fs = FileSystems.newFileSystem(zip, (ClassLoader) null)) {
            Files.walk(fs.getPath("/"))
                    .map(p -> p.toString())
                    .sorted()
                    .forEach(p -> System.out.println("entry=" + p));

            BasicFileAttributes attrs = Files.readAttributes(fs.getPath("/a/x.txt"),
                    BasicFileAttributes.class);
            System.out.println("attrs-size=" + attrs.size()
                    + " file=" + attrs.isRegularFile()
                    + " dir=" + attrs.isDirectory());

            BasicFileAttributes rootAttrs = Files.readAttributes(fs.getPath("/b/y"),
                    BasicFileAttributes.class);
            System.out.println("dir-attr=" + rootAttrs.isDirectory());

            // 目录条目列举（读目录流）
            try (var s = Files.list(fs.getPath("/b"))) {
                System.out.println("list-count=" + s.count());
            }
        }

        Files.deleteIfExists(zip);
        Files.deleteIfExists(dir);
        System.out.println("done=true");
    }
}
