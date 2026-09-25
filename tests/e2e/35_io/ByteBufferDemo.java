import java.nio.ByteBuffer;
import java.nio.ByteOrder;

public class ByteBufferDemo {
    public static void main(String[] args) {
        // allocate and put/get bytes
        ByteBuffer buf = ByteBuffer.allocate(16);
        buf.put((byte) 1);
        buf.put((byte) 2);
        buf.put((byte) 3);
        System.out.println(buf.position());
        buf.flip();
        System.out.println(buf.limit());
        System.out.println(buf.get());
        System.out.println(buf.get());
        System.out.println(buf.get());

        // putInt / getInt
        ByteBuffer buf2 = ByteBuffer.allocate(8);
        buf2.putInt(0x01020304);
        buf2.putInt(-1);
        buf2.flip();
        System.out.println(buf2.getInt());
        System.out.println(buf2.getInt());

        // wrap
        byte[] arr = {10, 20, 30, 40};
        ByteBuffer wrapped = ByteBuffer.wrap(arr);
        System.out.println(wrapped.capacity());
        System.out.println(wrapped.get());
        System.out.println(wrapped.get());

        // remaining / hasRemaining
        ByteBuffer buf3 = ByteBuffer.allocate(4);
        buf3.put((byte) 5);
        buf3.put((byte) 6);
        buf3.flip();
        System.out.println(buf3.remaining());
        System.out.println(buf3.hasRemaining());
        buf3.get();
        buf3.get();
        System.out.println(buf3.hasRemaining());

        // rewind
        ByteBuffer buf4 = ByteBuffer.allocate(4);
        buf4.put((byte) 9);
        buf4.flip();
        System.out.println(buf4.get());
        buf4.rewind();
        System.out.println(buf4.get());

        System.out.println("done");
    }
}
