import java.lang.invoke.MethodHandles;
import java.lang.invoke.VarHandle;
import java.nio.ByteBuffer;
import java.nio.ByteOrder;

/**
 * 直接内存上的原子访问与普通批量写混访：VarHandle（byteBufferView）CAS / getAndAdd 与另一线程对同一
 * 8 字节字相邻字节的 put(byte[]) / putShort 并发，计数不得丢失；未对齐的原子访问被拒（JDK 在 Java 侧
 * 抛 IllegalStateException）；对齐的各宽度 CAS 见证旧值。
 */
public class TestDirectMemoryAtomics {
    static final VarHandle INT = MethodHandles.byteBufferViewVarHandle(int[].class, ByteOrder.nativeOrder());
    static final VarHandle LONG = MethodHandles.byteBufferViewVarHandle(long[].class, ByteOrder.nativeOrder());

    public static void main(String[] args) throws Exception {
        ByteBuffer d = ByteBuffer.allocateDirect(64).order(ByteOrder.nativeOrder());

        System.out.println("cas int ok=" + INT.compareAndSet(d, 8, 0, 41) + " now=" + (int) INT.getVolatile(d, 8));
        System.out.println("cas int stale=" + INT.compareAndSet(d, 8, 0, 99) + " now=" + (int) INT.getVolatile(d, 8));
        System.out.println("getAndAdd old=" + (int) INT.getAndAdd(d, 8, 1) + " now=" + (int) INT.get(d, 8));
        System.out.println("cas long ok=" + LONG.compareAndSet(d, 16, 0L, -5L) + " now=" + (long) LONG.getVolatile(d, 16));
        System.out.println("exchange long old=" + (long) LONG.getAndSet(d, 16, 7L) + " now=" + (long) LONG.get(d, 16));

        try {
            INT.compareAndSet(d, 9, 0, 1);
            System.out.println("misaligned cas: no exception");
        } catch (IllegalStateException e) {
            System.out.println("misaligned cas: " + e.getClass().getSimpleName());
        }

        // 同一 8 字节字：计数在 [36, 40)，另一线程反复写 [32, 36) 与 [40, 44)
        final int iters = 200_000;
        ByteBuffer writerView = d.duplicate().order(ByteOrder.nativeOrder());
        Thread writer = new Thread(() -> {
            byte[] chunk = new byte[4];
            for (int i = 0; i < iters; i++) {
                java.util.Arrays.fill(chunk, (byte) i);
                writerView.put(32, chunk);
                writerView.putShort(40, (short) i);
                writerView.put(42, (byte) i);
            }
        });
        writer.start();
        for (int i = 0; i < iters; i++) {
            INT.getAndAdd(d, 36, 1);
        }
        writer.join();
        System.out.println("counter=" + (int) INT.getVolatile(d, 36) + " expected=" + iters);

        // 普通未对齐读写不受原子对齐约束
        d.putLong(49, 0x0102030405060708L);
        System.out.println("unaligned plain long=" + Long.toHexString(d.getLong(49)));
    }
}
