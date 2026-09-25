import java.util.Objects;
import java.util.IntSummaryStatistics;
import java.util.LongSummaryStatistics;
import java.util.DoubleSummaryStatistics;

public class JavaBaseComboTest {
    public static void main(String[] args) {
        // Objects + Statistics 综合测试

        // 1. 用 Objects.requireNonNull 保护 Statistics 创建
        IntSummaryStatistics intStats = Objects.requireNonNull(new IntSummaryStatistics());
        intStats.accept(5);
        intStats.accept(10);
        intStats.accept(15);
        System.out.println("int count: " + intStats.getCount());
        System.out.println("int avg: " + intStats.getAverage());

        // 2. LongSummaryStatistics 独立使用
        LongSummaryStatistics longStats = new LongSummaryStatistics();
        longStats.accept(1000000000L);
        longStats.accept(2000000000L);
        System.out.println("long sum: " + longStats.getSum());
        System.out.println("long avg: " + longStats.getAverage());

        // 3. DoubleSummaryStatistics
        DoubleSummaryStatistics dblStats = new DoubleSummaryStatistics();
        dblStats.accept(1.1);
        dblStats.accept(2.2);
        dblStats.accept(3.3);
        System.out.println("dbl min: " + dblStats.getMin());
        System.out.println("dbl max: " + dblStats.getMax());

        // 4. Objects.equals 对 Statistics 结果进行比较
        System.out.println("equals sums: " + Objects.equals(
            intStats.getSum(),
            longStats.getSum()));
        System.out.println("equals same: " + Objects.equals(
            intStats.getCount(),
            longStats.getCount()));

        // 5. Objects.toString with null fallback
        String label = null;
        System.out.println("label: " + Objects.toString(label, "N/A"));

        // 6. Objects.isNull / nonNull
        System.out.println("intStats nonNull: " + Objects.nonNull(intStats));
        System.out.println("null isNull: " + Objects.isNull(null));
    }
}
