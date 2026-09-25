import java.util.IntSummaryStatistics;
import java.util.DoubleSummaryStatistics;

public class JavaBaseStatisticsTest {
    public static void main(String[] args) {
        // IntSummaryStatistics
        IntSummaryStatistics intStats = new IntSummaryStatistics();
        intStats.accept(10);
        intStats.accept(20);
        intStats.accept(30);
        System.out.println("int count: " + intStats.getCount());
        System.out.println("int sum: " + intStats.getSum());
        System.out.println("int min: " + intStats.getMin());
        System.out.println("int max: " + intStats.getMax());
        System.out.println("int avg: " + intStats.getAverage());

        // DoubleSummaryStatistics
        DoubleSummaryStatistics dblStats = new DoubleSummaryStatistics();
        dblStats.accept(1.5);
        dblStats.accept(2.5);
        dblStats.accept(3.5);
        System.out.println("dbl count: " + dblStats.getCount());
        System.out.println("dbl sum: " + dblStats.getSum());
        System.out.println("dbl min: " + dblStats.getMin());
        System.out.println("dbl max: " + dblStats.getMax());
        System.out.println("dbl avg: " + dblStats.getAverage());

        // Empty statistics
        IntSummaryStatistics emptyStats = new IntSummaryStatistics();
        System.out.println("empty count: " + emptyStats.getCount());
        System.out.println("empty sum: " + emptyStats.getSum());
        System.out.println("empty min: " + emptyStats.getMin());
        System.out.println("empty max: " + emptyStats.getMax());

        // Combine
        IntSummaryStatistics stats1 = new IntSummaryStatistics();
        stats1.accept(1);
        stats1.accept(2);
        IntSummaryStatistics stats2 = new IntSummaryStatistics();
        stats2.accept(3);
        stats2.accept(4);
        stats1.combine(stats2);
        System.out.println("combined count: " + stats1.getCount());
        System.out.println("combined sum: " + stats1.getSum());
    }
}
