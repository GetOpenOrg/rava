import java.io.ByteArrayInputStream;
import java.io.ByteArrayOutputStream;
import java.io.PrintStream;
import java.util.Scanner;

public class TestSystemSetStreams {
    public static void main(String[] args) throws Exception {
        PrintStream original = System.out;
        ByteArrayOutputStream captured = new ByteArrayOutputStream();
        System.setOut(new PrintStream(captured, true, "UTF-8"));
        System.out.println("captured line 1");
        System.out.printf("n=%d%n", 42);
        System.setOut(original);
        System.out.println("restored; captured=[" + captured.toString("UTF-8").replace("\n", "|") + "]");
        System.out.println("same-original=" + (System.out == original));

        PrintStream originalErr = System.err;
        ByteArrayOutputStream errBuf = new ByteArrayOutputStream();
        System.setErr(new PrintStream(errBuf, true, "UTF-8"));
        System.err.print("to-err");
        System.setErr(originalErr);
        System.out.println("err captured=" + errBuf.toString("UTF-8"));

        System.setIn(new ByteArrayInputStream("7 hello\nsecond line\n".getBytes("UTF-8")));
        Scanner sc = new Scanner(System.in);
        int k = sc.nextInt();
        String w = sc.next();
        sc.nextLine();
        String line = sc.nextLine();
        System.out.println("in: " + k + " " + w + " / " + line);
    }
}
