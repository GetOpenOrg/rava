import java.util.PrimitiveIterator;
import java.util.stream.DoubleStream;
import java.util.stream.IntStream;
import java.util.stream.LongStream;

/**
 * N5 槽位形态 1 + 3：原始类型流的 SpinedBuffer 路径。
 *
 * SpinedBuffer.OfInt / OfLong / OfDouble 覆盖 OfPrimitive 的抽象槽
 * arrayForEach / arrayLength（经 javac 桥填擦除槽位），其 Spliterator 覆盖
 * arrayForOne；Node.copyInto（接口 default）走 toArray，Spliterator.tryAdvance
 * 走 iterator 逐个推进。三种元素类型 × 四个入口（sorted 缓冲 / builder /
 * toArray / iterator）逐一覆盖。
 */
public class TestPrimitiveSpinedBuffer {
    public static void main(String[] args) {
        // ── int ────────────────────────────────────────────────────────
        // sorted：有状态阶段，元素经 SpinedBuffer.OfInt 缓冲后 forEach（arrayForEach）
        StringBuilder sb = new StringBuilder();
        IntStream.of(5, 3, 9, 1, 7).sorted().forEach(v -> sb.append(v).append(' '));
        System.out.println("int.sorted=" + sb.toString().trim());
        // builder：直接写入 SpinedBuffer.OfInt，build 后遍历
        IntStream.Builder ib = IntStream.builder();
        for (int i = 0; i < 20; i++) ib.add(i * i);   // 跨多个 spine 块（初始块 16）
        System.out.println("int.builder.sum=" + ib.build().sum());
        // toArray：arrayLength + Node.copyInto
        int[] ia = IntStream.range(0, 40).filter(v -> v % 3 == 0).sorted().toArray();
        System.out.println("int.toArray.len=" + ia.length + ",last=" + ia[ia.length - 1]);
        // iterator：Spliterator.tryAdvance → arrayForOne
        PrimitiveIterator.OfInt it = IntStream.of(4, 2, 8).sorted().iterator();
        StringBuilder si = new StringBuilder();
        while (it.hasNext()) si.append(it.nextInt()).append(',');
        System.out.println("int.iter=" + si);
        // 空缓冲（count=0 分支）
        System.out.println("int.empty=" + IntStream.empty().sorted().toArray().length);

        // ── long ───────────────────────────────────────────────────────
        StringBuilder sl = new StringBuilder();
        LongStream.of(30L, 10L, 20L).sorted().forEach(v -> sl.append(v).append(' '));
        System.out.println("long.sorted=" + sl.toString().trim());
        LongStream.Builder lb = LongStream.builder();
        for (long i = 1; i <= 18; i++) lb.add(i * 1_000_000_000L);
        System.out.println("long.builder.sum=" + lb.build().sum());
        long[] la = LongStream.rangeClosed(1, 25).map(v -> v * v).sorted().toArray();
        System.out.println("long.toArray.len=" + la.length + ",last=" + la[la.length - 1]);
        PrimitiveIterator.OfLong lit = LongStream.of(3L, 1L, 2L).sorted().iterator();
        StringBuilder sli = new StringBuilder();
        while (lit.hasNext()) sli.append(lit.nextLong()).append(',');
        System.out.println("long.iter=" + sli);

        // ── double ─────────────────────────────────────────────────────
        StringBuilder sd = new StringBuilder();
        DoubleStream.of(2.5, 0.5, 1.5).sorted().forEach(v -> sd.append(v).append(' '));
        System.out.println("double.sorted=" + sd.toString().trim());
        DoubleStream.Builder db = DoubleStream.builder();
        for (int i = 0; i < 17; i++) db.add(i + 0.25);
        System.out.println("double.builder.sum=" + db.build().sum());
        double[] da = DoubleStream.of(9.0, 4.0, 1.0).sorted().toArray();
        System.out.println("double.toArray=" + da[0] + "," + da[1] + "," + da[2]);
        PrimitiveIterator.OfDouble dit = DoubleStream.of(0.3, 0.1, 0.2).sorted().iterator();
        StringBuilder sdi = new StringBuilder();
        while (dit.hasNext()) sdi.append(dit.nextDouble()).append(',');
        System.out.println("double.iter=" + sdi);

        // ── 装箱回对象流（OfPrimitive → 引用流边界）────────────────────
        System.out.println("boxed=" + IntStream.of(3, 1, 2).sorted().boxed()
                .map(String::valueOf).reduce("", (a, b) -> a + b));
    }
}
