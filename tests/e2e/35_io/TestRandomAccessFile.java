import java.io.File;
import java.io.IOException;
import java.io.RandomAccessFile;

public class TestRandomAccessFile {
    public static void main(String[] args) throws IOException {
        File f = File.createTempFile("raf", ".bin");
        f.deleteOnExit();
        try (RandomAccessFile raf = new RandomAccessFile(f, "rw")) {
            raf.writeInt(0x01020304);
            raf.writeLong(-2L);
            raf.writeUTF("héllo");
            raf.write(new byte[]{9, 8, 7}, 1, 2);
            System.out.println("length=" + raf.length() + " pos=" + raf.getFilePointer());
            raf.seek(0);
            System.out.println("int=" + Integer.toHexString(raf.readInt()));
            System.out.println("long=" + raf.readLong());
            System.out.println("utf=" + raf.readUTF());
            System.out.println("b=" + raf.read() + "," + raf.read() + " eof=" + raf.read());
            byte[] buf = new byte[4];
            raf.seek(2);
            int n = raf.read(buf, 1, 3);
            System.out.println("n=" + n + " buf=" + buf[0] + "," + buf[1] + "," + buf[2] + "," + buf[3]);
            raf.setLength(6);
            System.out.println("after setLength: length=" + raf.length() + " pos=" + raf.getFilePointer());
            raf.seek(10);
            raf.write(0x41);
            System.out.println("sparse length=" + raf.length());
            try {
                raf.read(buf, 3, 5);
            } catch (IndexOutOfBoundsException e) {
                System.out.println("IOOBE");
            }
        }
        try (RandomAccessFile ro = new RandomAccessFile(f, "r")) {
            System.out.println("ro length=" + ro.length() + " first=" + ro.read());
        }
        try {
            new RandomAccessFile(new File(f.getParentFile(), "no/such/dir/x"), "r");
        } catch (java.io.FileNotFoundException e) {
            System.out.println("FNFE");
        }
    }
}
