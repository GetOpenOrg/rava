import java.lang.reflect.Field;
import sun.misc.Unsafe;

/**
 * 堆上基本类型数组跨元素原子读-改-写的并发回归（array/store.rs 分条互斥）：
 * Unsafe.getAndAddInt 在 byte[] 的 int 路径跨 4 个元素，与其他跨元素读-改-写互斥——
 * 同偏移两线程计数不丢；重叠区间跨 16 字节分条边界（各加只动同一字节的掩码）互不丢更新；
 * long 路径（跨 8 个元素）往返一致。区间在单个元素内的原子性由 TestUnsafePrimitiveArray 覆盖。
 */
public class TestHeapArrayAtomicView {
    static final int ITERS = 200_000;

    public static void main(String[] args) throws Exception {
        Field tf = Unsafe.class.getDeclaredField("theUnsafe");
        tf.setAccessible(true);
        Unsafe u = (Unsafe) tf.get(null);
        final long B = Unsafe.ARRAY_BYTE_BASE_OFFSET;

        // 1) 同偏移 [16,20)：两线程各 ITERS 次加 1，跨元素读-改-写彼此互斥，总数不丢
        byte[] same = new byte[64];
        Thread t1 = adder(u, same, 16, 1, ITERS);
        Thread t2 = adder(u, same, 16, 1, ITERS);
        t1.join(); t2.join();
        System.out.println("same-offset counter=" + u.getInt(same, B + 16) + " expected=" + (2 * ITERS));

        // 2) 重叠区间跨 16 字节边界：A 的 int 在 [14,18)（加 1<<16 → 只进全局字节 16），
        //    B 的 int 在 [16,20)（加 1 → 只进全局字节 16）；两路径只共同改动字节 16，各 100 次
        byte[] ov = new byte[64];
        Thread a = adder(u, ov, 14, 0x00010000, 100);
        Thread b = adder(u, ov, 16, 0x00000001, 100);
        a.join(); b.join();
        int stray = 0;
        for (int i = 8; i < 24; i++) if (i != 16 && ov[i] != 0) stray++;
        System.out.println("overlap byte16=" + (ov[16] & 0xFF) + " expected=200 stray=" + stray);

        // 3) long 路径（跨 8 个元素）：写位形 → 读-改-写 → 读回
        byte[] lg = new byte[64];
        u.putLong(lg, B + 24, 0x0102030405060708L);
        long got = u.getAndAddLong(lg, B + 24, 1L);
        System.out.println("long rmw old=" + Long.toHexString(got) + " now=" + Long.toHexString(u.getLong(lg, B + 24)));
    }

    static Thread adder(Unsafe u, byte[] arr, int off, int mask, int iters) {
        long base = Unsafe.ARRAY_BYTE_BASE_OFFSET + off;
        Thread t = new Thread(() -> {
            for (int i = 0; i < iters; i++) u.getAndAddInt(arr, base, mask);
        });
        t.start();
        return t;
    }
}
