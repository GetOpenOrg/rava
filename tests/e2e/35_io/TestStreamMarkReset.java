import java.io.BufferedInputStream;
import java.io.ByteArrayInputStream;
import java.io.BufferedReader;
import java.io.IOException;
import java.io.InputStream;
import java.io.StringReader;

/**
 * 解析器级 IO 原语：mark/reset 语义与越界边界（jackson/commons-io/jsoup 的看家动作，
 * e2e 此前零覆盖）。含 BufferedInputStream 越过 mark limit 后 reset 的异常形态。
 */
public class TestStreamMarkReset {

    public static void main(String[] args) throws Exception {
        ByteArrayInputStream bais = new ByteArrayInputStream(new byte[] { 1, 2, 3, 4, 5 });
        System.out.println("bais-supported=" + bais.markSupported());
        bais.mark(2);
        System.out.println("read1=" + bais.read() + "," + bais.read());
        bais.reset();
        System.out.println("after-reset=" + bais.read());

        // BufferedInputStream：mark limit 内 reset 有效
        BufferedInputStream bis = new BufferedInputStream(
                new ByteArrayInputStream(new byte[] { 10, 20, 30, 40 }), 8);
        System.out.println("bis-supported=" + bis.markSupported());
        bis.mark(4);
        System.out.println("bis-a=" + bis.read() + "," + bis.read());
        bis.reset();
        System.out.println("bis-replay=" + bis.read());

        // 边界：读越过 mark(limit) 后 reset → IOException
        BufferedInputStream bis2 = new BufferedInputStream(
                new ByteArrayInputStream(new byte[] { 1, 2, 3, 4, 5, 6, 7, 8, 9, 10 }), 2);
        bis2.mark(1);
        bis2.read();
        bis2.read();
        bis2.read();
        try {
            bis2.reset();
        } catch (IOException e) {
            System.out.println("reset-ex=" + e.getClass().getSimpleName());
        }

        // Reader 族
        StringReader sr = new StringReader("abc");
        System.out.println("sr-supported=" + sr.markSupported());
        sr.mark(1);
        sr.read();
        sr.reset();
        System.out.println("sr-replay=" + (char) sr.read());

        BufferedReader br = new BufferedReader(new StringReader("line1\nline2"));
        br.mark(100);
        System.out.println("br-line1=" + br.readLine());
        br.reset();
        System.out.println("br-replay=" + br.readLine());

        // mark 不支持时 reset 的边界（基类 InputStream：markSupported=false，reset 默认抛 IOException）
        InputStream noMark = new InputStream() {
            @Override
            public int read() {
                return -1;
            }
        };
        System.out.println("nomark-supported=" + noMark.markSupported());
        noMark.mark(1);
        try {
            noMark.reset();
        } catch (IOException e) {
            System.out.println("nomark-ex=" + e.getClass().getSimpleName());
        }
    }
}
