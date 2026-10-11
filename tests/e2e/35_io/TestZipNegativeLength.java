// Deflater / Inflater 的 off / len 越界与负值：JDK 在 Java 层或 native 前抛 ArrayIndexOutOfBoundsException
import java.util.zip.*;

public class TestZipNegativeLength {
    interface Op { void run() throws Exception; }

    static void check(String label, Op op) {
        try {
            op.run();
            System.out.println(label + ": ok");
        } catch (Exception e) {
            System.out.println(label + ": " + e.getClass().getName() + ": " + e.getMessage());
        }
    }

    public static void main(String[] args) throws Exception {
        byte[] data = "hello hello hello hello".getBytes();
        Deflater d = new Deflater();
        check("setInput neg len", () -> d.setInput(data, 0, -1));
        check("setInput neg off", () -> d.setInput(data, -1, 2));
        check("setInput over", () -> d.setInput(data, 20, 10));
        check("deflate neg len", () -> d.deflate(new byte[16], 0, -5));
        check("deflate over", () -> d.deflate(new byte[16], 10, 10));
        check("setDictionary neg", () -> d.setDictionary(data, 0, -3));
        d.setInput(data);
        d.finish();
        byte[] buf = new byte[128];
        int n = d.deflate(buf);
        d.end();
        System.out.println("deflated ok: " + (n > 0));

        Inflater inf = new Inflater();
        check("inflater setInput neg", () -> inf.setInput(buf, 0, -1));
        check("inflate neg len", () -> inf.inflate(new byte[16], 0, -1));
        check("inflate neg off", () -> inf.inflate(new byte[16], -2, 4));
        inf.setInput(buf, 0, n);
        byte[] out = new byte[64];
        int m = inf.inflate(out);
        System.out.println("inflated: " + new String(out, 0, m));
        inf.end();
    }
}
