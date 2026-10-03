import java.io.RandomAccessFile;
import java.nio.ByteBuffer;
import java.nio.channels.FileChannel;
import java.nio.charset.StandardCharsets;
import java.nio.file.Files;
import java.nio.file.Path;

/**
 * RandomAccessFile 与其通道（方法级实测：getChannel 5 jar，此前零覆盖）：
 * seek 读写、通道 size/position、通道 ByteBuffer 读写、mode 边界。
 */
public class TestRandomAccessFileChannel {

    public static void main(String[] args) throws Exception {
        Path tmp = Files.createTempFile("raf-e2e", ".bin");

        try (RandomAccessFile raf = new RandomAccessFile(tmp.toFile(), "rw")) {
            raf.write("hello".getBytes(StandardCharsets.UTF_8));
            raf.writeInt(0x41424344);
            System.out.println("len=" + raf.length());

            // seek 回读
            raf.seek(0);
            byte[] head = new byte[5];
            raf.readFully(head);
            System.out.println("head=" + new String(head, StandardCharsets.UTF_8));
            System.out.println("int-back=" + Integer.toHexString(raf.readInt()));

            // 通道：size/position 与流位置独立
            FileChannel ch = raf.getChannel();
            System.out.println("ch-size=" + ch.size() + " pos=" + ch.position());
            ch.position(0);
            ByteBuffer buf = ByteBuffer.allocate(5);
            ch.read(buf);
            System.out.println("ch-read=" + new String(buf.array(), 0, 5, StandardCharsets.UTF_8));

            // 通道写（追加于指定位置，不扰动流游标）
            ch.write(ByteBuffer.wrap("!".getBytes(StandardCharsets.UTF_8)), 5);
            raf.seek(0);
            byte[] all = new byte[6];
            raf.readFully(all);
            System.out.println("after-ch-write=" + new String(all, StandardCharsets.UTF_8));

            // raf 便捷族
            raf.seek(0);
            System.out.println("read-byte=" + raf.read());
            raf.writeBoolean(true);
            raf.writeDouble(0.5);
            raf.seek(6);
            System.out.println("bool=" + raf.readBoolean() + " dbl=" + raf.readDouble());
        }

        // 只读模式边界
        try (RandomAccessFile ro = new RandomAccessFile(tmp.toFile(), "r")) {
            ro.seek(0);
            System.out.println("ro-read=" + ro.read());
            try {
                ro.write(1);
            } catch (java.io.IOException e) {
                System.out.println("ro-write-ex=" + e.getClass().getSimpleName());
            }
        }

        Files.deleteIfExists(tmp);
        System.out.println("done=true");
    }
}
