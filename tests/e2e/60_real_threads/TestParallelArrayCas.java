// 数组元素 CAS 经协变视图的原子性（#42 mt）：ForkJoinPool 工作队列（ForkJoinTask[]）与
// ConcurrentHashMap 表（Node[]）的槽位 CAS 在并行后端必须原子——否则任务重复执行 / 丢失、
// merge 丢更新。输出只含与调度无关的确定值。
import java.util.*;
import java.util.concurrent.*;
import java.util.stream.*;

public class TestParallelArrayCas {
    static class Sum extends RecursiveTask<Long> {
        final int lo, hi;
        Sum(int lo, int hi) { this.lo = lo; this.hi = hi; }
        protected Long compute() {
            if (hi - lo <= 64) { long s = 0; for (int i = lo; i < hi; i++) s += i; return s; }
            int mid = (lo + hi) >>> 1;
            Sum l = new Sum(lo, mid); l.fork();
            return new Sum(mid, hi).compute() + l.join();
        }
    }

    public static void main(String[] args) {
        for (int round = 0; round < 5; round++) {
            long s = ForkJoinPool.commonPool().invoke(new Sum(0, 200_000));
            ConcurrentHashMap<Integer, Integer> m = new ConcurrentHashMap<>();
            IntStream.range(0, 20_000).parallel().forEach(i -> m.merge(i % 37, 1, Integer::sum));
            int total = m.values().stream().mapToInt(Integer::intValue).sum();
            List<Integer> sq = IntStream.range(0, 500).parallel().boxed().map(i -> i * 2).collect(Collectors.toList());
            boolean ordered = true;
            for (int i = 0; i < sq.size(); i++) ordered &= sq.get(i) == i * 2;
            ConcurrentHashMap<Integer, Integer> big = new ConcurrentHashMap<>();
            IntStream.range(0, 5_000).parallel().forEach(i -> big.put(i, i));
            System.out.println("round " + round + " sum=" + s + " keys=" + m.size() + " total=" + total
                + " collect=" + sq.size() + " ordered=" + ordered + " big=" + big.size());
        }
    }
}
