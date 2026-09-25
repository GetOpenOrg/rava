import java.io.*;
import java.nio.file.*;
import java.util.*;

public class FileIODemo {
    public static void main(String[] args) throws Exception {
        String tmpPath = "/tmp/ruva_fileio_test.txt";

        // --- BufferedWriter + FileWriter write ---
        BufferedWriter bw = new BufferedWriter(new FileWriter(tmpPath));
        bw.write("hello");
        bw.newLine();
        bw.write("world");
        bw.newLine();
        bw.close();

        // --- BufferedReader + FileReader read ---
        BufferedReader br = new BufferedReader(new FileReader(tmpPath));
        String line;
        while ((line = br.readLine()) != null) {
            System.out.println(line);
        }
        br.close();

        // --- Files.readAllLines ---
        List<String> lines = Files.readAllLines(Paths.get(tmpPath));
        System.out.println(lines.size());

        // --- Files.exists ---
        System.out.println(Files.exists(Paths.get(tmpPath)));

        // cleanup
        Files.delete(Paths.get(tmpPath));
        System.out.println(Files.exists(Paths.get(tmpPath)));
    }
}
