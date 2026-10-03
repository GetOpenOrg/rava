import java.io.ByteArrayInputStream;
import java.io.IOException;
import java.io.PushbackInputStream;
import java.io.PushbackReader;
import java.io.StringReader;

/**
 * 解析器级 IO 原语：PushbackInputStream / PushbackReader 的回看语义与容量边界
 * （流式解析器回退一个字符的标准件，e2e 此前零覆盖）。
 */
public class TestPushbackStreams {

    public static void main(String[] args) throws Exception {
        PushbackInputStream pi = new PushbackInputStream(
                new ByteArrayInputStream(new byte[] { 1, 2, 3 }), 4);
        System.out.println("read=" + pi.read());
        pi.unread(1);
        System.out.println("unread-single=" + pi.read());

        byte[] buf = new byte[3];
        int n = pi.read(buf);
        System.out.println("n=" + n + " b0=" + buf[0]);
        pi.unread(buf, 0, n);
        System.out.println("unread-block-first=" + pi.read());

        // 容量边界：超容量 unread → IOException
        PushbackInputStream small = new PushbackInputStream(
                new ByteArrayInputStream(new byte[] { 5 }), 1);
        small.unread(9);
        try {
            small.unread(8);
        } catch (IOException e) {
            System.out.println("overflow-ex=" + e.getClass().getSimpleName());
        }

        // 构造边界：容量 0 → IllegalArgumentException
        try {
            new PushbackInputStream(new ByteArrayInputStream(new byte[1]), 0);
        } catch (IllegalArgumentException e) {
            System.out.println("zero-buf-ex=" + e.getClass().getSimpleName());
        }

        PushbackReader pr = new PushbackReader(new StringReader("xyz"), 2);
        System.out.println("pr-read=" + (char) pr.read());
        pr.unread('X');
        pr.unread('W');
        System.out.println("pr-replay=" + (char) pr.read() + (char) pr.read());
        try {
            pr.unread('Z');
        } catch (IOException e) {
            System.out.println("pr-overflow-ex=" + e.getClass().getSimpleName());
        }
    }
}
