import java.util.concurrent.atomic.DoubleAdder;
import java.util.concurrent.atomic.LongAdder;

/**
 * LongAdder / DoubleAdder 计数器族（Caffeine/HikariCP 的指标计数地基，
 * e2e 此前零覆盖）：add/increment/sum/sumThenReset/reset 语义。
 */
public class TestLongAdderCounters {

    public static void main(String[] args) {
        LongAdder la = new LongAdder();
        System.out.println("initial=" + la.sum());
        la.increment();
        la.increment();
        la.add(40);
        System.out.println("sum=" + la.sum());
        System.out.println("intValue=" + la.intValue() + " longValue=" + la.longValue());

        la.decrement();
        System.out.println("after-dec=" + la.sum());

        // sumThenReset：取值后清零
        System.out.println("then-reset=" + la.sumThenReset());
        System.out.println("after-reset=" + la.sum());

        la.reset();
        System.out.println("reset=" + la.sum());

        // 负值与累计
        la.add(-5);
        la.add(5);
        System.out.println("neg-acc=" + la.sum());

        DoubleAdder da = new DoubleAdder();
        da.add(0.1);
        da.add(0.2);
        System.out.println("double-sum=" + da.sum());
        da.add(-0.3);
        System.out.println("double-cancel=" + (da.sum() == 0.0));
    }
}
