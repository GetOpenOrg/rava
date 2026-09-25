import java.io.File;
import java.io.FileWriter;
import java.io.FileReader;
import java.io.BufferedReader;

public class FileIOTest {
    public static void main(String[] args) throws Exception {
        String testFile = "/tmp/jnc-fileio-test.txt";

        // Write to file
        FileWriter writer = new FileWriter(testFile);
        writer.write("Hello from JNC!\n");
        writer.write("Line two\n");
        writer.write("Line three\n");
        writer.close();
        System.out.println("Write OK");

        // Check file exists
        File f = new File(testFile);
        System.out.println("Exists: " + f.exists());
        System.out.println("Is file: " + f.isFile());

        // Read back
        BufferedReader reader = new BufferedReader(new FileReader(testFile));
        String line1 = reader.readLine();
        String line2 = reader.readLine();
        String line3 = reader.readLine();
        reader.close();
        System.out.println("Read: " + line1);
        System.out.println("Read: " + line2);
        System.out.println("Read: " + line3);

        // Clean up
        boolean deleted = f.delete();
        System.out.println("Deleted: " + deleted);
    }
}
