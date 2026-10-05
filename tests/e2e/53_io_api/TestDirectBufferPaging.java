import java.nio.*;

/** 直接内存边界（a3-U1）：跨页 putLong / getLong、两种字节序的批量交换拷贝（copySwapMemory）、
 *  分配清零（setMemory）、直接缓冲区之间的批量拷贝（copyMemory）。 */
public class TestDirectBufferPaging {
    public static void main(String[] args) {
        int page = 4096;
        ByteBuffer d = ByteBuffer.allocateDirect(3 * page);
        long zeros = 0;
        for (int i = 0; i < d.capacity(); i++) zeros += d.get(i);
        System.out.println("zeroed sum=" + zeros);
        for (int k = 1; k <= 2; k++) {
            int at = k * page - 4;
            d.putLong(at, 0x1122334455667788L * k);
            System.out.println("cross page " + k + ": " + Long.toHexString(d.getLong(at))
                + " lo=" + d.get(at) + " hi=" + d.get(at + 7));
        }
        long[] src = new long[600];
        for (int i = 0; i < src.length; i++) src[i] = ((long) i << 40) ^ (i * 0x9E3779B97F4A7C15L);
        for (ByteOrder order : new ByteOrder[] { ByteOrder.BIG_ENDIAN, ByteOrder.LITTLE_ENDIAN }) {
            d.clear();
            d.order(order);
            LongBuffer lb = d.asLongBuffer();
            lb.put(src);
            long[] back = new long[src.length];
            lb.flip();
            lb.get(back);
            System.out.println(order + " roundtrip=" + java.util.Arrays.equals(src, back)
                + " first byte=" + d.get(8) + " last long=" + Long.toHexString(d.getLong((src.length - 1) * 8)));
            int[] ints = new int[700];
            for (int i = 0; i < ints.length; i++) ints[i] = i * 31 - 9000;
            d.clear();
            d.asIntBuffer().put(ints);
            int[] ib = new int[ints.length];
            d.asIntBuffer().get(ib);
            char[] cs = "页边界-direct-swap".toCharArray();
            d.asCharBuffer().put(cs);
            char[] cb = new char[cs.length];
            d.asCharBuffer().get(cb);
            System.out.println(order + " ints=" + java.util.Arrays.equals(ints, ib) + " b0=" + d.get(0)
                + " chars=" + new String(cb));
        }
        ByteBuffer other = ByteBuffer.allocateDirect(2 * page);
        d.clear();
        d.position(page - 100).limit(page - 100 + other.capacity());
        other.put(d);
        other.flip();
        System.out.println("bulk direct copy: remaining=" + other.remaining() + " at100="
            + Long.toHexString(other.order(d.order()).getLong(96)));
    }
}
