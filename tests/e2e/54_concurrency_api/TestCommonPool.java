// FS-T3：availableProcessors 返回真实核数后，ForkJoinPool 公共池 / 并行流 / CompletableFuture 走公共池。
// 输出不含核数本身（跨机器确定），只断言与核数无关的性质。
import java.util.concurrent.*;
import java.util.stream.*;

public class TestCommonPool {
    static class Fib extends RecursiveTask<Long> {
        final int n;
        Fib(int n) { this.n = n; }
        protected Long compute() {
            if (n < 12) return slow(n);
            Fib f1 = new Fib(n - 1);
            f1.fork();
            return new Fib(n - 2).compute() + f1.join();
        }
        static long slow(int n) { return n < 2 ? n : slow(n - 1) + slow(n - 2); }
    }

    public static void main(String[] args) throws Exception {
        int cpus = Runtime.getRuntime().availableProcessors();
        System.out.println("cpus>0 " + (cpus > 0) + " parallelism>0 " + (ForkJoinPool.commonPool().getParallelism() > 0)
            + " consistent " + (ForkJoinPool.getCommonPoolParallelism() == Math.max(1, cpus - 1)));
        System.out.println("fib(24)=" + ForkJoinPool.commonPool().invoke(new Fib(24)));
        long sum = IntStream.rangeClosed(1, 100_000).parallel().asLongStream().sum();
        System.out.println("parallel sum=" + sum);
        System.out.println("parallel collect=" + IntStream.range(0, 20).parallel().boxed()
            .map(i -> i * i).collect(Collectors.toList()));
        CompletableFuture<Integer> a = CompletableFuture.supplyAsync(() -> 20);
        CompletableFuture<Integer> b = CompletableFuture.supplyAsync(() -> 22);
        System.out.println("combine=" + a.thenCombine(b, Integer::sum).get(10, TimeUnit.SECONDS));
        Boolean worker = CompletableFuture.supplyAsync(() -> {
            Thread t = Thread.currentThread();
            // 公共池并行度 > 1（cpus > 2）时异步任务在公共池守护工作线程上执行
            return cpus > 2 ? (t.getName().startsWith("ForkJoinPool.commonPool-worker-") && t.isDaemon()) : true;
        }).get(10, TimeUnit.SECONDS);
        System.out.println("worker " + worker);
        ConcurrentHashMap<Integer, Integer> m = new ConcurrentHashMap<>();
        IntStream.range(0, 1000).parallel().forEach(i -> m.merge(i % 10, 1, Integer::sum));
        System.out.println("merge " + new java.util.TreeMap<>(m));
    }
}
