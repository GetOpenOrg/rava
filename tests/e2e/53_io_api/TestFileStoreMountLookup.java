import java.io.IOException;
import java.nio.ByteBuffer;
import java.nio.MappedByteBuffer;
import java.nio.channels.FileChannel;
import java.nio.charset.StandardCharsets;
import java.nio.file.*;
import java.nio.file.attribute.*;
import java.util.ArrayList;
import java.util.List;

/**
 * 文件系统 native 的平台路径边界（输出与平台无关）：
 * - 挂载表：Files.getFileStore / FileSystems.getFileStores（Linux 读 /proc/mounts：setmntent →
 *   getlinelen 求最长行 → rewind → getmntent；macOS 走 getfsstat / fsstatEntry / endfsstat）；
 * - 用户 / 组按名与按 id 查询（getpwnam0 / getgrnam0 / getpwuid / getgrgid），查不到的名字；
 * - 硬链接与改名（link0 / rename0）；目录流（Linux 为 SecureDirectoryStream：openat0 / fstatat0 /
 *   unlinkat0 / renameat0）；
 * - 用户自定义扩展属性（fsetxattr0 / flistxattr / fgetxattr0 / fremovexattr0）；
 * - FileChannel 的 transferTo / transferFrom（Linux copy_file_range）与 map（map0）；
 * - Files.copy 的内核内拷贝（Linux / macOS 的 directCopy0），含已存在目标与 REPLACE_EXISTING。
 */
public class TestFileStoreMountLookup {
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
        Path base = Files.createTempDirectory("rava_fsml");
        try {
            run(base);
        } finally {
            try (var walk = Files.walk(base)) {
                for (Path p : walk.sorted((a, b) -> b.compareTo(a)).toList()) {
                    Files.deleteIfExists(p);
                }
            }
        }
    }

    static void run(Path base) throws IOException {
        Path f = Files.writeString(base.resolve("data.txt"), "hello mounts");

        FileStore store = Files.getFileStore(f);
        show("store name nonempty", () -> !store.name().isEmpty());
        show("store type nonempty", () -> !store.type().isEmpty());
        show("store same as dir", () -> store.equals(Files.getFileStore(base)));
        show("store posix view", () -> store.supportsFileAttributeView(PosixFileAttributeView.class));
        show("store total > 0", () -> store.getTotalSpace() > 0);
        show("store usable <= total", () -> store.getUsableSpace() <= store.getTotalSpace());
        show("stores listed", () -> {
            int n = 0;
            boolean found = false;
            for (FileStore s : FileSystems.getDefault().getFileStores()) {
                n++;
                found |= s.equals(store);
            }
            return (n > 0) + " / contains store " + found;
        });

        UserPrincipalLookupService lookup = FileSystems.getDefault().getUserPrincipalLookupService();
        PosixFileAttributes pa = Files.readAttributes(f, PosixFileAttributes.class);
        UserPrincipal owner = pa.owner();
        GroupPrincipal group = pa.group();
        show("owner by name", () -> lookup.lookupPrincipalByName(owner.getName()).equals(owner));
        show("group by name", () -> lookup.lookupPrincipalByGroupName(group.getName()).equals(group));
        show("missing user", () -> lookup.lookupPrincipalByName("rava_no_such_user_x9"));
        show("missing group", () -> lookup.lookupPrincipalByGroupName("rava_no_such_group_x9"));

        Path hard = base.resolve("hard.txt");
        show("hard link", () -> Files.createLink(hard, f).getFileName());
        show("hard same file", () -> Files.isSameFile(hard, f));
        show("hard same key", () -> Files.readAttributes(hard, BasicFileAttributes.class).fileKey()
                .equals(Files.readAttributes(f, BasicFileAttributes.class).fileKey()));
        Path moved = base.resolve("moved.txt");
        show("rename", () -> Files.move(hard, moved).getFileName() + " / old exists " + Files.exists(hard));
        // 源与目标为同一文件（硬链接）：JDK 视为无操作，不报 FileAlreadyExistsException
        show("rename onto same file", () -> Files.move(moved, f).getFileName() + " / src exists " + Files.exists(moved));
        show("rename onto other", () -> Files.move(moved, Files.writeString(base.resolve("other.txt"), "x")));

        Path dir = Files.createDirectory(base.resolve("dir"));
        Files.writeString(dir.resolve("a"), "A");
        Files.writeString(dir.resolve("b"), "BB");
        show("dir stream", () -> dirOps(dir));

        UserDefinedFileAttributeView xv = Files.getFileAttributeView(f, UserDefinedFileAttributeView.class);
        show("xattr", () -> {
            if (!store.supportsFileAttributeView(UserDefinedFileAttributeView.class)) {
                // 不支持 user xattr 的文件系统（如旧内核 tmpfs）：与支持时同输出，只确认查询不崩
                return "ok";
            }
            xv.write("rava.k", ByteBuffer.wrap("v1".getBytes(StandardCharsets.UTF_8)));
            boolean listed = xv.list().contains("rava.k");
            ByteBuffer buf = ByteBuffer.allocate(xv.size("rava.k"));
            xv.read("rava.k", buf);
            String v = new String(buf.array(), 0, buf.position(), StandardCharsets.UTF_8);
            xv.delete("rava.k");
            boolean gone = !xv.list().contains("rava.k");
            return listed && v.equals("v1") && gone ? "ok" : "listed=" + listed + " value=" + v + " gone=" + gone;
        });

        Path copy = base.resolve("copy.txt");
        show("transferTo", () -> {
            try (FileChannel in = FileChannel.open(f); FileChannel out = FileChannel.open(copy,
                    StandardOpenOption.CREATE, StandardOpenOption.WRITE)) {
                long n = in.transferTo(6, in.size() - 6, out);
                return n + " " + Files.readString(copy) + " / src pos " + in.position();
            }
        });
        show("transferFrom", () -> {
            try (FileChannel in = FileChannel.open(f); FileChannel out = FileChannel.open(copy,
                    StandardOpenOption.WRITE)) {
                long n = out.transferFrom(in, 6, 5);
                return n + " " + Files.readString(copy) + " / src pos " + in.position();
            }
        });
        show("transferTo append", () -> {
            try (FileChannel in = FileChannel.open(f); FileChannel out = FileChannel.open(copy,
                    StandardOpenOption.APPEND)) {
                long n = in.transferTo(0, 5, out);
                return n + " " + Files.readString(copy);
            }
        });
        show("map", () -> {
            try (FileChannel ch = FileChannel.open(f)) {
                MappedByteBuffer mb = ch.map(FileChannel.MapMode.READ_ONLY, 6, 6);
                byte[] b = new byte[mb.remaining()];
                mb.get(b);
                return new String(b, StandardCharsets.UTF_8) + " / remaining " + mb.remaining();
            }
        });
        Path dup = base.resolve("dup.txt");
        show("Files.copy", () -> Files.readString(Files.copy(f, dup)));
        show("Files.copy onto existing", () -> Files.copy(f, dup));
        show("Files.copy replace", () -> Files.readString(Files.copy(copy, dup, StandardCopyOption.REPLACE_EXISTING)));
    }

    /** 目录流内的读属性、删除、改名；Linux 为 SecureDirectoryStream（*at 系统调用），其余平台用 Files 等价操作 */
    static String dirOps(Path dir) throws IOException {
        List<String> names = new ArrayList<>();
        try (DirectoryStream<Path> ds = Files.newDirectoryStream(dir)) {
            for (Path p : ds) {
                names.add(p.getFileName().toString());
            }
            names.sort(null);
            Path a = Path.of("a"), b = Path.of("b"), c = Path.of("c");
            long size;
            if (ds instanceof SecureDirectoryStream<Path> sds) {
                size = sds.getFileAttributeView(b, BasicFileAttributeView.class).readAttributes().size();
                sds.deleteFile(a);
                sds.move(b, sds, c);
            } else {
                size = Files.size(dir.resolve(b));
                Files.delete(dir.resolve(a));
                Files.move(dir.resolve(b), dir.resolve(c));
            }
            List<String> after = new ArrayList<>();
            try (DirectoryStream<Path> again = Files.newDirectoryStream(dir)) {
                again.forEach(p -> after.add(p.getFileName().toString()));
            }
            return names + " size(b)=" + size + " after " + after;
        }
    }
}
