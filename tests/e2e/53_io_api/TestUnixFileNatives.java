import java.io.IOException;
import java.nio.file.*;
import java.nio.file.attribute.*;
import java.util.Comparator;
import java.util.Set;
import java.util.stream.Stream;

/**
 * POSIX 文件 native 族：mkdir0 / symlink0 / readlink0 / realpath0 / chmod0 / fchmod0 /
 * chown0 / lchown0 / utimes0 / futimens0 / lutimes0 / fstat0 / statvfs0 / bufferedCopy0
 * （macOS 另经 setattrlist0 / fsetattrlist0）。覆盖成功路径与 errno → 异常映射的边界。
 */
public class TestUnixFileNatives {
    static void show(String label, ThrowingRunnable r) {
        try {
            r.run();
        } catch (IOException e) {
            System.out.println(label + " -> " + e.getClass().getSimpleName());
        }
    }

    interface ThrowingRunnable {
        void run() throws IOException;
    }

    public static void main(String[] args) throws IOException {
        Path base = Files.createTempDirectory("rava_ufn");
        try {
            run(base);
        } finally {
            try (Stream<Path> s = Files.walk(base)) {
                for (Path p : s.sorted(Comparator.reverseOrder()).toList()) {
                    Files.deleteIfExists(p);
                }
            }
            System.out.println("cleaned = " + !Files.exists(base));
        }
    }

    static void run(Path base) throws IOException {
        // mkdir0：新建、已存在、父目录缺失
        Path dir = base.resolve("d");
        Files.createDirectory(dir);
        System.out.println("mkdir = " + Files.isDirectory(dir));
        show("mkdir again", () -> Files.createDirectory(dir));
        show("mkdir no parent", () -> Files.createDirectory(base.resolve("x/y")));
        Path deep = Files.createDirectories(base.resolve("p/q/r"));
        System.out.println("mkdirs = " + Files.isDirectory(deep));

        // bufferedCopy0 / fstat0：文件复制、覆盖、目标已存在
        Path src = dir.resolve("a.txt");
        Files.writeString(src, "x".repeat(20000) + "end");
        Path dst = base.resolve("b.txt");
        Files.copy(src, dst);
        System.out.println("copy = " + Files.size(dst) + " " + Files.readString(dst).endsWith("end"));
        show("copy exists", () -> Files.copy(src, dst));
        Files.writeString(src, "short");
        Files.copy(src, dst, StandardCopyOption.REPLACE_EXISTING);
        System.out.println("copy replace = " + Files.readString(dst));
        Path emptySrc = dir.resolve("empty");
        Files.createFile(emptySrc);
        Files.copy(emptySrc, base.resolve("empty2"));
        System.out.println("copy empty = " + Files.size(base.resolve("empty2")));
        Path dirCopy = base.resolve("dcopy");
        Files.copy(dir, dirCopy);
        System.out.println("copy dir = " + Files.isDirectory(dirCopy));

        // symlink0 / readlink0 / realpath0
        Path link = base.resolve("lnk");
        Files.createSymbolicLink(link, Path.of("d/a.txt"));
        System.out.println("readlink = " + Files.readSymbolicLink(link));
        System.out.println("is link = " + Files.isSymbolicLink(link));
        show("symlink exists", () -> Files.createSymbolicLink(link, Path.of("d")));
        show("readlink plain", () -> Files.readSymbolicLink(src));
        Path real = link.toRealPath();
        System.out.println("realpath = " + real.endsWith(Path.of("d", "a.txt")) + " " + real.isAbsolute());
        System.out.println("realpath dots = " + base.resolve("d/../p/./q").toRealPath().endsWith(Path.of("p", "q")));
        show("realpath missing", () -> base.resolve("nope").toRealPath());
        Path dangling = base.resolve("dangling");
        Files.createSymbolicLink(dangling, Path.of("missing-target"));
        System.out.println("dangling exists = " + Files.exists(dangling) + ", nofollow = "
                + Files.exists(dangling, LinkOption.NOFOLLOW_LINKS));

        // chmod0 / fchmod0（经属性视图）
        Files.setPosixFilePermissions(src, PosixFilePermissions.fromString("rw-r-----"));
        System.out.println("chmod = " + PosixFilePermissions.toString(Files.getPosixFilePermissions(src)));
        Files.getFileAttributeView(src, PosixFileAttributeView.class)
                .setPermissions(Set.of(PosixFilePermission.OWNER_READ, PosixFilePermission.OWNER_WRITE));
        System.out.println("fchmod = " + PosixFilePermissions.toString(Files.getPosixFilePermissions(src)));
        show("chmod missing", () -> Files.setPosixFilePermissions(base.resolve("nope"),
                PosixFilePermissions.fromString("rw-------")));

        // chown0 / lchown0：改为当前属主（无权限要求）
        UserPrincipal owner = Files.getOwner(src);
        Files.setOwner(src, owner);
        System.out.println("chown = " + Files.getOwner(src).equals(owner));
        // getpwuid：新建文件属主即当前用户，名字经 uid 反查
        System.out.println("owner name = " + owner.getName().equals(System.getProperty("user.name")));
        Files.getFileAttributeView(link, PosixFileAttributeView.class, LinkOption.NOFOLLOW_LINKS).setOwner(owner);
        System.out.println("lchown = " + Files.getOwner(link, LinkOption.NOFOLLOW_LINKS).equals(owner));

        // futimens0 / utimes0 / lutimes0（时间只取毫秒，规避文件系统精度差异）
        FileTime t1 = FileTime.fromMillis(1577934245123L);
        Files.setLastModifiedTime(src, t1);
        System.out.println("mtime = " + Files.getLastModifiedTime(src).toMillis());
        BasicFileAttributeView view = Files.getFileAttributeView(src, BasicFileAttributeView.class);
        FileTime t2 = FileTime.fromMillis(946684800000L);
        view.setTimes(null, t2, null);
        BasicFileAttributes attrs = view.readAttributes();
        System.out.println("atime = " + attrs.lastAccessTime().toMillis() + ", mtime kept = "
                + attrs.lastModifiedTime().toMillis());
        Files.getFileAttributeView(link, BasicFileAttributeView.class, LinkOption.NOFOLLOW_LINKS)
                .setTimes(t2, t2, null);
        System.out.println("link mtime = " + Files.getLastModifiedTime(link, LinkOption.NOFOLLOW_LINKS).toMillis()
                + ", target mtime = " + Files.getLastModifiedTime(src).toMillis());
        Files.setLastModifiedTime(src, FileTime.fromMillis(0));
        System.out.println("epoch mtime = " + Files.getLastModifiedTime(src).toMillis());

        // statvfs0；getmntonname0（macOS 挂载点）/ 挂载表（Linux）：toString 为「挂载点 (设备名)」
        FileStore store = Files.getFileStore(base);
        long total = store.getTotalSpace();
        System.out.println("store = " + (total > 0) + " " + (store.getUsableSpace() <= total) + " "
                + (store.getUnallocatedSpace() <= total) + " " + (store.getBlockSize() > 0));
        System.out.println("mount = " + store.toString().startsWith("/") + " "
                + store.equals(Files.getFileStore(base.resolve("d"))));
    }
}
