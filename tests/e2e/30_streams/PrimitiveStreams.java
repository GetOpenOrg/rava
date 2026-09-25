import java.util.stream.IntStream;
import java.util.stream.LongStream;
import java.util.stream.DoubleStream;

public class PrimitiveStreams {
    public static void main(String[] args) {
        // IntStream.range sum
        int isum = IntStream.range(1, 6).sum();
        System.out.println(isum); // 15

        // IntStream.range filter count
        long icount = IntStream.range(1, 11).filter(n -> n % 2 == 0).count();
        System.out.println(icount); // 5

        // IntStream.rangeClosed
        int rcsum = IntStream.rangeClosed(1, 5).sum();
        System.out.println(rcsum); // 15

        // LongStream.range sum
        long lsum = LongStream.range(1, 6).sum();
        System.out.println(lsum); // 15

        // LongStream.rangeClosed count
        long lcount = LongStream.rangeClosed(1, 100).filter(n -> n % 10 == 0).count();
        System.out.println(lcount); // 10

        // DoubleStream.of sum
        double dsum = DoubleStream.of(1.5, 2.5, 3.0).sum();
        System.out.println((int) dsum); // 7

        // DoubleStream.of filter count
        long dcount = DoubleStream.of(1.0, 2.0, 3.0, 4.0).filter(d -> d > 2.0).count();
        System.out.println(dcount); // 2
    }
}
