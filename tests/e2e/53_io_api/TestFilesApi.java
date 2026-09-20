import java.nio.file.*;
import java.io.IOException;

public class TestFilesApi {
    public static void main(String[] args) throws IOException {
        Path p = Path.of("e2e_files_tmp.txt");
        Path q = Path.of("e2e_files_tmp2.txt");
        try {
            Files.writeString(p, "hello\nworld\n");
            String content = Files.readString(p);
            System.out.println("contentRead=" + content.equals("hello\nworld\n"));
            System.out.println("lines=" + Files.readAllLines(p).size());
            System.out.println("mismatchSame=" + Files.mismatch(p, p));
            Files.writeString(q, "same");
            System.out.println("mismatchDiff=" + Files.mismatch(p, q));
            System.out.println("exists=" + Files.exists(p));
        } finally {
            Files.deleteIfExists(p);
            Files.deleteIfExists(q);
            System.out.println("cleaned=" + (!Files.exists(p) && !Files.exists(q)));
        }
    }
}
