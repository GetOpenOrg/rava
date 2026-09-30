import java.nio.*;
import java.util.zip.*;

/** 直接内存：ByteBuffer.allocateDirect（Unsafe.allocateMemory0 族）+ zip 直接缓冲区 native。 */
public class TestDirectBuffer {
    public static void main(String[] args) throws Exception {
        ByteBuffer d = ByteBuffer.allocateDirect(64);
        System.out.println("isDirect: " + d.isDirect() + " cap=" + d.capacity() + " zero=" + d.get(10));
        d.putInt(0x01020304).putLong(-2L).putDouble(3.5).putShort((short) -7).putChar('Z');
        d.flip();
        System.out.println("int=" + Integer.toHexString(d.getInt()) + " long=" + d.getLong()
            + " double=" + d.getDouble() + " short=" + d.getShort() + " char=" + d.getChar());
        d.clear();
        d.order(ByteOrder.LITTLE_ENDIAN).putInt(0, 0x0A0B0C0D);
        System.out.println("le bytes: " + d.get(0) + "," + d.get(1) + "," + d.get(2) + "," + d.get(3));
        byte[] src = "direct-memory".getBytes();
        d.clear();
        d.put(src);
        d.flip();
        byte[] dst = new byte[d.remaining()];
        d.get(dst);
        System.out.println("bulk: " + new String(dst));
        ByteBuffer slice = d.position(7).slice();
        System.out.println("slice: " + (char) slice.get(0) + " remaining=" + slice.remaining());
        IntBuffer ib = ByteBuffer.allocateDirect(16).asIntBuffer();
        for (int i = 0; i < 4; i++) ib.put(i * i);
        ib.flip();
        StringBuilder sb = new StringBuilder();
        while (ib.hasRemaining()) sb.append(ib.get()).append(' ');
        System.out.println("intview: " + sb.toString().trim());

        byte[] data = "The quick brown fox jumps over the lazy dog. ".repeat(20).getBytes();
        ByteBuffer in = ByteBuffer.allocateDirect(data.length);
        in.put(data).flip();
        CRC32 crc = new CRC32();
        crc.update(in);
        CRC32 crcHeap = new CRC32();
        crcHeap.update(data);
        System.out.println("crc32 direct==heap: " + (crc.getValue() == crcHeap.getValue()) + " " + Long.toHexString(crc.getValue()));
        in.rewind();
        Adler32 ad = new Adler32();
        ad.update(in);
        System.out.println("adler32: " + Long.toHexString(ad.getValue()));

        in.rewind();
        Deflater def = new Deflater();
        def.setInput(in);
        def.finish();
        ByteBuffer comp = ByteBuffer.allocateDirect(1024);
        while (!def.finished()) def.deflate(comp);
        def.end();
        comp.flip();
        System.out.println("compressed smaller: " + (comp.remaining() < data.length));
        Inflater inf = new Inflater();
        inf.setInput(comp);
        ByteBuffer out = ByteBuffer.allocateDirect(data.length);
        while (!inf.finished()) inf.inflate(out);
        inf.end();
        out.flip();
        byte[] round = new byte[out.remaining()];
        out.get(round);
        System.out.println("roundtrip equal: " + java.util.Arrays.equals(round, data));
    }
}
