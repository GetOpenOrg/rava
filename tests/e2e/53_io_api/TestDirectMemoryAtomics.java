import java.lang.reflect.Field;
import sun.misc.Unsafe;

/**
 * 直接内存（Unsafe.allocateMemory 绝对地址）上的原子访问与普通批量访问混用：对齐的 int / long CAS 见证旧值；
 * getAndAddInt 计数与另一线程对同一 8 字节字相邻字节的 putByte / putShort / copyMemory / setMemory 并发，
 * 计数不得丢失；setMemory / copyMemory 负长度抛 IllegalArgumentException；普通读写不要求对齐。
 */
public class TestDirectMemoryAtomics {
    public static void main(String[] args) throws Exception {
        Field tf = Unsafe.class.getDeclaredField("theUnsafe");
        tf.setAccessible(true);
        Unsafe u = (Unsafe) tf.get(null);
        long base = u.allocateMemory(64);
        u.setMemory(base, 64, (byte) 0);

        System.out.println("cas int ok=" + u.compareAndSwapInt(null, base + 8, 0, 41) + " now=" + u.getIntVolatile(null, base + 8));
        System.out.println("cas int stale=" + u.compareAndSwapInt(null, base + 8, 0, 99) + " now=" + u.getInt(base + 8));
        System.out.println("getAndAdd old=" + u.getAndAddInt(null, base + 8, 1) + " now=" + u.getInt(base + 8));
        System.out.println("cas long ok=" + u.compareAndSwapLong(null, base + 16, 0L, -5L) + " now=" + u.getLongVolatile(null, base + 16));
        System.out.println("getAndSet long old=" + u.getAndSetLong(null, base + 16, 7L) + " now=" + u.getLong(base + 16));
        System.out.println("neighbours intact: " + u.getInt(base + 12) + " " + u.getLong(base + 24));

        try {
            u.setMemory(base, -1, (byte) 0);
            System.out.println("setMemory negative: no exception");
        } catch (IllegalArgumentException e) {
            System.out.println("setMemory negative: IllegalArgumentException");
        }
        try {
            u.copyMemory(base, base + 32, -8);
            System.out.println("copyMemory negative: no exception");
        } catch (IllegalArgumentException e) {
            System.out.println("copyMemory negative: IllegalArgumentException");
        }

        // 同一 8 字节字 [32, 40)：计数在 [36, 40)，另一线程反复写 [32, 36)；另一字 [40, 48) 作拷贝源
        final int iters = 200_000;
        final long word = base + 32;
        Thread writer = new Thread(() -> {
            for (int i = 0; i < iters; i++) {
                u.putByte(word, (byte) i);
                u.putShort(word + 1, (short) i);
                u.setMemory(word + 8, 8, (byte) i);
                u.copyMemory(word + 8, word, 4);
            }
        });
        writer.start();
        for (int i = 0; i < iters; i++) {
            u.getAndAddInt(null, word + 4, 1);
        }
        writer.join();
        System.out.println("counter=" + u.getIntVolatile(null, word + 4) + " expected=" + iters);

        // 普通未对齐读写
        u.putLong(base + 49, 0x0102030405060708L);
        System.out.println("unaligned plain long=" + Long.toHexString(u.getLong(base + 49)));
        u.freeMemory(base);
    }
}
