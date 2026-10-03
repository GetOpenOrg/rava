import java.io.BufferedReader;
import java.nio.channels.FileChannel;
import java.nio.charset.StandardCharsets;
import java.nio.file.DirectoryStream;
import java.nio.file.Files;
import java.nio.file.Path;
import java.nio.file.StandardOpenOption;

/**
 * Files 流与通道族（方法级实测：newInputStream 17 jar / newOutputStream 9 /
 * newBufferedReader 5 / newByteChannel 5 / newDirectoryStream 5 / toAbsolutePath 9，
 * 此前零覆盖）：全部走临时目录，路径不打印（只打印名字与内容）。
 */
public class TestFilesStreamChannels {

    public static void main(String[] args) throws Exception {
        Path dir = Files.createTempDirectory("files-e2e");
        Path f1 = dir.resolve("a.txt");
        Path f2 = dir.resolve("b.txt");
        Path sub = dir.resolve("sub");
        Files.createDirectories(sub);
        Files.write(f2, "skip-me".getBytes(StandardCharsets.UTF_8));

        // newOutputStream 写 → newInputStream 读
        try (var out = Files.newOutputStream(f1, StandardOpenOption.CREATE,
                StandardOpenOption.WRITE, StandardOpenOption.TRUNCATE_EXISTING)) {
            out.write("stream-io".getBytes(StandardCharsets.UTF_8));
        }
        try (var in = Files.newInputStream(f1)) {
            System.out.println("stream-read=" + new String(in.readAllBytes(), StandardCharsets.UTF_8));
        }

        // newBufferedReader / newBufferedWriter
        try (BufferedReader br = Files.newBufferedReader(f1, StandardCharsets.UTF_8)) {
            System.out.println("reader-line=" + br.readLine());
        }
        try (var w = Files.newBufferedWriter(dir.resolve("c.txt"), StandardCharsets.UTF_8)) {
            w.write("buffered");
        }
        try (BufferedReader br = Files.newBufferedReader(dir.resolve("c.txt"), StandardCharsets.UTF_8)) {
            System.out.println("writer-roundtrip=" + br.readLine());
        }

        // newByteChannel：可读可写通道
        try (FileChannel ch = (FileChannel) Files.newByteChannel(f1,
                StandardOpenOption.READ, StandardOpenOption.WRITE)) {
            System.out.println("channel-size=" + ch.size());
            ch.position(0);
            java.nio.ByteBuffer buf = java.nio.ByteBuffer.allocate(6);
            ch.read(buf);
            System.out.println("channel-head=" + new String(buf.array(), 0, 6, StandardCharsets.UTF_8));
        }

        // newDirectoryStream：glob 过滤
        try (DirectoryStream<Path> ds = Files.newDirectoryStream(dir, "*.txt")) {
            int n = 0;
            for (Path p : ds) {
                n++;
            }
            System.out.println("glob-count=" + n);
        }
        // 过滤器形态
        try (DirectoryStream<Path> ds = Files.newDirectoryStream(dir,
                entry -> Files.isDirectory(entry))) {
            for (Path p : ds) {
                System.out.println("dir-entry=" + p.getFileName());
            }
        }

        // toAbsolutePath：同根校验（不打印路径本体）
        System.out.println("abs-same-root=" + f1.toAbsolutePath().startsWith(dir.toAbsolutePath()));
        System.out.println("fs-readonly=" + !Files.getFileSystem(dir).isReadOnly());

        Files.walk(dir).sorted(java.util.Comparator.reverseOrder()).forEach(p -> {
            try {
                Files.deleteIfExists(p);
            } catch (Exception e) {
                // 清理
            }
        });
        System.out.println("done=" + !Files.exists(dir));
    }
}
