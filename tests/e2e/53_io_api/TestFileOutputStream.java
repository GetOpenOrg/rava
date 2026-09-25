import java.io.BufferedOutputStream;
import java.io.FileInputStream;
import java.io.FileNotFoundException;
import java.io.FileOutputStream;
import java.io.IOException;
import java.io.PrintStream;
import java.nio.charset.StandardCharsets;

/**
 * FileOutputStream 原生层（open0 / write / writeBytes）与 FileInputStream 回读
 * （Rosetta RecordsSerializationTest 揭出：FileOutputStream.open0 未实现）。
 * 覆盖：截断写 / 追加写 / 单字节 / 字节数组 / 偏移切片 / BufferedOutputStream /
 * PrintStream 包装、回读内容与长度、目标目录不存在的 FileNotFoundException 消息
 * （写与读两侧）、关闭后写入的 IOException。文件落在仓库 build/ 下（测试以仓库根为 cwd）。
 */
public class TestFileOutputStream {
    static final String PATH = "build/test_file_output_stream.tmp";

    static String readAll(String path) throws IOException {
        try (FileInputStream in = new FileInputStream(path)) {
            byte[] buf = new byte[256];
            int n = 0, r;
            while ((r = in.read(buf, n, buf.length - n)) > 0) n += r;
            return new String(buf, 0, n, StandardCharsets.UTF_8);
        }
    }

    public static void main(String[] args) throws IOException {
        try (FileOutputStream out = new FileOutputStream(PATH)) {
            out.write('H');
            out.write("ello".getBytes(StandardCharsets.UTF_8));
            byte[] data = "xx, world!yy".getBytes(StandardCharsets.UTF_8);
            out.write(data, 2, 8);
        }
        System.out.println("[" + readAll(PATH) + "]");

        try (FileOutputStream out = new FileOutputStream(PATH, true)) {
            out.write(" +append".getBytes(StandardCharsets.UTF_8));
        }
        System.out.println("[" + readAll(PATH) + "]");

        try (FileOutputStream out = new FileOutputStream(PATH)) {
            out.write("truncated".getBytes(StandardCharsets.UTF_8));
        }
        System.out.println("[" + readAll(PATH) + "]");

        try (BufferedOutputStream bo = new BufferedOutputStream(new FileOutputStream(PATH), 4)) {
            for (int i = 0; i < 10; i++) bo.write('0' + i);
        }
        System.out.println("[" + readAll(PATH) + "]");

        try (PrintStream ps = new PrintStream(new FileOutputStream(PATH, true), true, "UTF-8")) {
            ps.print("|");
            ps.println(42);
            ps.printf("%s-%d", "ab", 7);
        }
        System.out.println("[" + readAll(PATH).replace("\n", "\\n") + "]");

        try {
            new FileOutputStream("build/no_such_dir_xyz/out.txt");
            System.out.println("no exception?");
        } catch (FileNotFoundException e) {
            System.out.println("FNF write: " + e.getMessage());
        }
        try {
            new FileInputStream("build/no_such_dir_xyz/in.txt");
            System.out.println("no exception?");
        } catch (FileNotFoundException e) {
            System.out.println("FNF read: " + e.getMessage());
        }

        FileOutputStream closed = new FileOutputStream(PATH);
        closed.close();
        try {
            closed.write(1);
            System.out.println("no exception?");
        } catch (IOException e) {
            System.out.println("closed: " + e.getClass().getSimpleName() + " " + e.getMessage());
        }
    }
}
