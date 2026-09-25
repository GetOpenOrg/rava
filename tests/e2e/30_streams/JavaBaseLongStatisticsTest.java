import java.util.LongSummaryStatistics;

public class JavaBaseLongStatisticsTest {
    public static void main(String[] args) {
        // LongSummaryStatistics
        LongSummaryStatistics stats = new LongSummaryStatistics();
        stats.accept(100L);
        stats.accept(200L);
        stats.accept(300L);
        System.out.println("count: " + stats.getCount());
        System.out.println("sum: " + stats.getSum());
        System.out.println("min: " + stats.getMin());
        System.out.println("max: " + stats.getMax());
        System.out.println("avg: " + stats.getAverage());

        // Empty
        LongSummaryStatistics empty = new LongSummaryStatistics();
        System.out.println("empty count: " + empty.getCount());
        System.out.println("empty sum: " + empty.getSum());
        System.out.println("empty min: " + empty.getMin());
        System.out.println("empty max: " + empty.getMax());

        // Combine
        LongSummaryStatistics a = new LongSummaryStatistics();
        a.accept(10L);
        a.accept(20L);
        LongSummaryStatistics b = new LongSummaryStatistics();
        b.accept(30L);
        b.accept(40L);
        a.combine(b);
        System.out.println("combined count: " + a.getCount());
        System.out.println("combined sum: " + a.getSum());
        System.out.println("combined min: " + a.getMin());
        System.out.println("combined max: " + a.getMax());
    }
}
