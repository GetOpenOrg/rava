import java.io.File;
import java.io.FileWriter;
import java.io.FileReader;
import java.io.BufferedReader;

public class TryWithResourcesTest {
    public static void main(String[] args) throws Exception {
        String testFile = "/tmp/jnc-twr-test.txt";

        // Test 1: try-with-resources — FileWriter auto-close
        try (FileWriter fw = new FileWriter(testFile)) {
            fw.write("Hello TWR\n");
            fw.write("Line two\n");
        }
        System.out.println("Test 1: Write with TWR OK");

        // Test 2: try-with-resources — BufferedReader auto-close
        try (BufferedReader br = new BufferedReader(new FileReader(testFile))) {
            String line = br.readLine();
            System.out.println("Test 2: Read = " + line);
        }

        // Test 3: exception inside try block — resource still closed
        try (FileWriter fw2 = new FileWriter(testFile)) {
            fw2.write("Before exception\n");
            throw new RuntimeException("test error");
        } catch (RuntimeException e) {
            System.out.println("Test 3: Caught = " + e.getMessage());
        }

        // Verify file was written (resource closed before catch)
        try (BufferedReader br2 = new BufferedReader(new FileReader(testFile))) {
            String line = br2.readLine();
            System.out.println("Test 3: File content = " + line);
        }

        // Test 4: multiple resources
        String testFile2 = "/tmp/jnc-twr-test2.txt";
        try (FileWriter fw3 = new FileWriter(testFile2)) {
            fw3.write("Multi resource\n");
        }
        try (FileReader fr = new FileReader(testFile2);
             BufferedReader br3 = new BufferedReader(fr)) {
            String line = br3.readLine();
            System.out.println("Test 4: Multi = " + line);
        }

        // Cleanup
        new File(testFile).delete();
        new File(testFile2).delete();
        System.out.println("Done");
    }
}
