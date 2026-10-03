import java.io.IOException;
import java.nio.file.*;
import java.nio.file.attribute.*;

/**
 * 不跟随符号链接的属性读写边界：悬空链接与自指链接（跟随即 ELOOP）上按 NOFOLLOW_LINKS 读属性、
 * 设时间（Linux 经 lutimes0，macOS 经 setattrlist0），跟随时的 errno → 异常映射。
 */
public class TestSymlinkNoFollowAttrs {
    interface Io {
        Object run() throws IOException;
    }

    static void show(String label, Io r) {
        try {
            System.out.println(label + " = " + r.run());
        } catch (IOException e) {
            System.out.println(label + " -> " + e.getClass().getSimpleName());
        }
    }

    public static void main(String[] args) throws IOException {
        Path base = Files.createTempDirectory("rava_snf");
        Path dangling = base.resolve("dangling");
        Path loop = base.resolve("loop");
        try {
            Files.createSymbolicLink(dangling, Path.of("missing-target"));
            Files.createSymbolicLink(loop, Path.of("loop"));
            FileTime t = FileTime.fromMillis(946684800000L);
            for (Path p : new Path[] {dangling, loop}) {
                String n = p.getFileName().toString();
                show(n + " exists", () -> Files.exists(p) + " / nofollow " + Files.exists(p, LinkOption.NOFOLLOW_LINKS));
                show(n + " is link", () -> Files.readAttributes(p, BasicFileAttributes.class, LinkOption.NOFOLLOW_LINKS).isSymbolicLink());
                show(n + " set times nofollow", () -> {
                    Files.getFileAttributeView(p, BasicFileAttributeView.class, LinkOption.NOFOLLOW_LINKS).setTimes(t, t, null);
                    return Files.getLastModifiedTime(p, LinkOption.NOFOLLOW_LINKS).toMillis();
                });
                show(n + " set mtime follow", () -> Files.setLastModifiedTime(p, t));
                show(n + " read attrs follow", () -> Files.readAttributes(p, BasicFileAttributes.class).size());
            }
        } finally {
            Files.deleteIfExists(dangling);
            Files.deleteIfExists(loop);
            Files.deleteIfExists(base);
        }
    }
}
