import java.io.ByteArrayInputStream;
import java.io.ByteArrayOutputStream;
import java.io.DataInputStream;
import java.io.DataOutputStream;
import java.io.InputStream;
import java.nio.charset.StandardCharsets;
import java.nio.file.Files;
import java.nio.file.Path;
import java.util.jar.JarEntry;
import java.util.jar.JarInputStream;
import java.util.jar.JarOutputStream;

/**
 * DataInput 无符号读与 JarInputStream 流式条目（方法级实测：readUnsignedByte/
 * readUnsignedShort 5 jar、getNextJarEntry 5 jar，此前零覆盖）：字节布局逐字可比。
 */
public class TestJarDataStreams {

    public static void main(String[] args) throws Exception {
        // DataOutputStream 布局 → DataInputStream 逐方法读
        ByteArrayOutputStream bos = new ByteArrayOutputStream();
        try (DataOutputStream dos = new DataOutputStream(bos)) {
            dos.writeByte(0xFF);            // -1 → 无符号 255
            dos.writeShort(0xABCD);
            dos.writeInt(0x11223344);
            dos.writeUTF("rava");
            dos.writeBoolean(true);
            dos.writeChar('中');
            dos.writeFloat(1.5f);
            dos.writeDouble(2.5);
            dos.writeLong(123456789012345L);
            dos.writeBytes("AB");           // 低字节
        }
        try (DataInputStream dis = new DataInputStream(
                new ByteArrayInputStream(bos.toByteArray()))) {
            System.out.println("ubyte=" + dis.readUnsignedByte());
            System.out.println("ushort=" + Integer.toHexString(dis.readUnsignedShort()));
            System.out.println("uint=" + Long.toHexString(dis.readInt() & 0xFFFFFFFFL));
            System.out.println("utf=" + dis.readUTF());
            System.out.println("bool=" + dis.readBoolean());
            System.out.println("char=" + dis.readChar());
            System.out.println("float=" + dis.readFloat());
            System.out.println("double=" + dis.readDouble());
            System.out.println("long=" + dis.readLong());
            System.out.println("bytes=" + (char) dis.readByte() + (char) dis.readByte());
            System.out.println("at-end=" + (dis.available() == 0));
        }

        // readFully：整段读
        try (DataInputStream dis = new DataInputStream(
                new ByteArrayInputStream(new byte[] { 1, 2, 3 }))) {
            byte[] buf = new byte[3];
            dis.readFully(buf);
            System.out.println("fully=" + buf[0] + buf[1] + buf[2]);
        }

        // JarInputStream：流式遍历条目（非文件系统通道）
        ByteArrayOutputStream jos = new ByteArrayOutputStream();
        try (JarOutputStream jout = new JarOutputStream(jos)) {
            jout.putNextEntry(new JarEntry("one.txt"));
            jout.write("1".getBytes(StandardCharsets.UTF_8));
            jout.closeEntry();
            jout.putNextEntry(new JarEntry("two.txt"));
            jout.write("22".getBytes(StandardCharsets.UTF_8));
            jout.closeEntry();
        }
        try (JarInputStream jin = new JarInputStream(
                new ByteArrayInputStream(jos.toByteArray()))) {
            JarEntry e;
            int entries = 0;
            while ((e = jin.getNextJarEntry()) != null) {
                entries++;
                InputStream src = jin;    // 当前条目体即流本体
                System.out.println("entry=" + e.getName()
                        + " body-len=" + src.readAllBytes().length);
            }
            System.out.println("jar-entries=" + entries);
        }
    }
}
