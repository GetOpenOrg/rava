// FS-IO1 / IO2：File.canRead / canWrite / canExecute、Files.isReadable / isExecutable 按 access(2)
// 精确判定；getTotalSpace / getFreeSpace / getUsableSpace 按 statvfs。只断言与运行用户无关的事实。
import java.io.File;
import java.nio.file.*;

public class TestFileAccessSpace {
    public static void main(String[] args) throws Exception {
        File dir = Files.createTempDirectory("fas").toFile();
        File f = new File(dir, "data.txt");
        Files.writeString(f.toPath(), "hello");
        System.out.println("file r=" + f.canRead() + " w=" + f.canWrite() + " x=" + f.canExecute());
        System.out.println("nio r=" + Files.isReadable(f.toPath()) + " x=" + Files.isExecutable(f.toPath()));
        System.out.println("setExecutable " + f.setExecutable(true) + " x=" + f.canExecute() + " nio x=" + Files.isExecutable(f.toPath()));
        System.out.println("dir x=" + dir.canExecute() + " r=" + dir.canRead());
        File missing = new File(dir, "missing");
        System.out.println("missing r=" + missing.canRead() + " w=" + missing.canWrite() + " x=" + missing.canExecute()
            + " nio=" + Files.isReadable(missing.toPath()));
        long total = dir.getTotalSpace(), free = dir.getFreeSpace(), usable = dir.getUsableSpace();
        System.out.println("space total>0 " + (total > 0) + " free<=total " + (free <= total) + " usable<=free " + (usable <= free)
            + " missing total " + missing.getTotalSpace());
        f.delete();
        dir.delete();
        System.out.println("cleanup " + !dir.exists());
    }
}
