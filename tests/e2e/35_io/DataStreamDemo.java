import java.io.*;

public class DataStreamDemo {
    public static void main(String[] args) throws Exception {
        // Write primitives to byte array, then read back
        ByteArrayOutputStream baos = new ByteArrayOutputStream();
        DataOutputStream dos = new DataOutputStream(baos);

        dos.writeInt(42);
        dos.writeLong(1234567890123L);
        dos.writeDouble(3.14159);
        dos.writeFloat(2.71f);
        dos.writeBoolean(true);
        dos.writeByte(127);
        dos.writeShort(32767);
        dos.writeUTF("Hello DataStream");
        dos.flush();

        byte[] data = baos.toByteArray();
        System.out.println(data.length > 0);

        DataInputStream dis = new DataInputStream(new ByteArrayInputStream(data));
        System.out.println(dis.readInt());
        System.out.println(dis.readLong());
        System.out.printf("%.5f%n", dis.readDouble());
        System.out.printf("%.2f%n", dis.readFloat());
        System.out.println(dis.readBoolean());
        System.out.println(dis.readByte());
        System.out.println(dis.readShort());
        System.out.println(dis.readUTF());

        // available after consuming
        System.out.println(dis.available() == 0);

        System.out.println("done");
    }
}
