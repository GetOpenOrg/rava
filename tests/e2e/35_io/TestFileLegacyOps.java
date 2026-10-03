import java.io.File;
import java.io.FileFilter;
import java.io.FilenameFilter;
import java.nio.file.Files;
import java.nio.file.Path;

/**
 * java.io.File 遗留族（方法级实测：getAbsolutePath 20 jar / toURI 16 /
 * listFiles(FileFilter) 14，此前零覆盖）：临时目录自建自清，不打印绝对路径
 * （用文件名与形态断言），listFiles 双过滤形态、mkdirs、separatorChar。
 */
public class TestFileLegacyOps {

    public static void main(String[] args) throws Exception {
        Path dir = Files.createTempDirectory("file-e2e");
        File root = dir.toFile();

        new File(root, "alpha.txt").createNewFile();
        new File(root, "beta.log").createNewFile();
        File sub = new File(root, "sub");
        System.out.println("mkdirs=" + sub.mkdirs());
        new File(sub, "gamma.txt").createNewFile();

        System.out.println("isDir=" + root.isDirectory() + " exists=" + root.exists());
        System.out.println("abs-absolute=" + new File(root, "alpha.txt").getAbsolutePath()
                .startsWith(root.getAbsolutePath()));
        System.out.println("uri-scheme=" + root.toURI().getScheme());
        System.out.println("name=" + sub.getName() + " parent-end=" + sub.getParent().endsWith("sub"));
        System.out.println("separator=" + File.separatorChar
                + " path-sep=[" + File.pathSeparatorChar + "]");

        // listFiles 全量（名字排序，不打印父路径）
        String[] all = root.list();
        java.util.Arrays.sort(all);
        System.out.println("list=" + String.join(",", all));

        // listFiles(FileFilter)：只取目录
        File[] dirs = root.listFiles((FileFilter) File::isDirectory);
        System.out.println("dirs=" + dirs.length + " first=" + dirs[0].getName());

        // listFiles(FilenameFilter)：后缀过滤
        File[] logs = root.listFiles((dir1, name) -> name.endsWith(".log"));
        System.out.println("logs=" + logs[0].getName());

        // list(FilenameFilter) 字符串形态
        String[] txts = root.list((d, n) -> n.endsWith(".txt"));
        System.out.println("txt-count=" + txts.length);

        // 长度与删除
        File alpha = new File(root, "alpha.txt");
        System.out.println("len-defined=" + (alpha.length() >= 0));
        System.out.println("delete=" + alpha.delete() + " gone=" + !alpha.exists());

        // 清理
        for (File f : root.listFiles()) {
            if (f.isDirectory()) {
                for (File g : f.listFiles()) {
                    g.delete();
                }
            }
            f.delete();
        }
        root.delete();
        System.out.println("cleaned=" + !root.exists());
    }
}
