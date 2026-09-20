import java.util.Arrays;
import java.util.IntSummaryStatistics;
import java.util.List;
import java.util.stream.Collectors;
import java.util.stream.DoubleStream;
import java.util.stream.IntStream;
import java.util.stream.LongStream;

public class TestStreamNumeric {

    public static void main(String[] args) {
        int[] raw = {4, 1, 3, 1, 5, 2};

        // IntStream 基础
        System.out.println("sum=" + IntStream.of(raw).sum());
        System.out.println("count=" + IntStream.of(raw).count());
        System.out.println("min=" + IntStream.of(raw).min().orElse(-1));
        System.out.println("max=" + IntStream.of(raw).max().orElse(-1));
        System.out.println("avg=" + IntStream.of(raw).average().orElse(0.0));
        System.out.println("distinct sorted=" + IntStream.of(raw).distinct().sorted().boxed().collect(Collectors.toList()));

        // range / rangeClosed
        System.out.println("range(1,5)=" + IntStream.range(1, 5).boxed().collect(Collectors.toList()));
        System.out.println("rangeClosed(1,5)=" + IntStream.rangeClosed(1, 5).boxed().collect(Collectors.toList()));
        System.out.println("range sum even=" + IntStream.rangeClosed(1, 10).filter(v -> v % 2 == 0).sum());

        // iterate / limit
        System.out.println("iterate=" + IntStream.iterate(1, v -> v * 2).limit(6).boxed().collect(Collectors.toList()));

        // summaryStatistics
        IntSummaryStatistics stats = IntStream.of(raw).summaryStatistics();
        System.out.println("stats count=" + stats.getCount() + " min=" + stats.getMin()
                + " max=" + stats.getMax() + " sum=" + stats.getSum() + " avg=" + stats.getAverage());

        // map / mapToObj
        System.out.println("mapped=" + IntStream.of(1, 2, 3).map(v -> v * v).boxed().collect(Collectors.toList()));
        System.out.println("mapToObj=" + IntStream.of(1, 2).mapToObj(v -> "n" + v).collect(Collectors.toList()));

        // reduce
        System.out.println("reduce sum=" + IntStream.of(1, 2, 3, 4).reduce(0, (a, b) -> a + b));
        System.out.println("reduce product=" + IntStream.rangeClosed(1, 4).reduce(1, (a, b) -> a * b));

        // LongStream
        System.out.println("long sum=" + LongStream.rangeClosed(1, 5).sum());
        System.out.println("long mapped=" + LongStream.of(100000000000L).map(v -> v / 2).sum());

        // DoubleStream
        System.out.println("double avg=" + DoubleStream.of(1.5, 2.5, 3.5).average().orElse(0.0));
        System.out.println("double sum=" + DoubleStream.of(0.1, 0.2).sum());

        // boxed 回到对象流
        List<Integer> boxed = IntStream.of(3, 1, 2).boxed().sorted().collect(Collectors.toList());
        System.out.println("boxed sorted=" + boxed);

        // flatMap 展开
        List<String> lines = Arrays.asList("a b", "c d");
        List<String> words = lines.stream()
                .flatMap(line -> Arrays.stream(line.split(" ")))
                .collect(Collectors.toList());
        System.out.println("flatMap words=" + words);

        // ints → String 拼接
        String joined = IntStream.rangeClosed(1, 4).mapToObj(String::valueOf).collect(Collectors.joining("-"));
        System.out.println("joined=" + joined);

        // 集合 Stream 与原始流互转
        List<Integer> nums = Arrays.asList(9, 8, 7);
        System.out.println("to int sum=" + nums.stream().mapToInt(Integer::intValue).sum());
        System.out.println("summarizing=" + nums.stream().mapToInt(Integer::intValue).summaryStatistics().getSum());

        // takeWhile / dropWhile（Java 9+）
        System.out.println("takeWhile(<3)=" + IntStream.of(1, 2, 3, 1).takeWhile(v -> v < 3).boxed().collect(Collectors.toList()));
        System.out.println("dropWhile(<3)=" + IntStream.of(1, 2, 3, 1).dropWhile(v -> v < 3).boxed().collect(Collectors.toList()));

        System.out.println("done");
    }
}
