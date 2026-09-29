import java.util.Arrays;
import java.util.Random;
import java.util.SplittableRandom;
import java.util.stream.Collectors;

// Random / SplittableRandom 的流式 API（jdk/internal/util/random/RandomSupport 的 Spliterator 族）
public class TestRandomStreams {
    public static void main(String[] args) {
        Random r = new Random(42);
        System.out.println("ints(5,0,100)=" + Arrays.toString(r.ints(5, 0, 100).toArray()));
        System.out.println("ints(3)=" + Arrays.toString(r.ints(3).toArray()));
        System.out.println("ints().limit(4)=" + Arrays.toString(r.ints().limit(4).toArray()));
        System.out.println("ints(10,20).limit(4)=" + Arrays.toString(r.ints(10, 20).limit(4).toArray()));
        System.out.println("longs(3)=" + Arrays.toString(r.longs(3).toArray()));
        System.out.println("longs(3,-5,5)=" + Arrays.toString(r.longs(3, -5L, 5L).toArray()));
        System.out.println("doubles(3)=" + Arrays.toString(r.doubles(3).toArray()));
        System.out.println("doubles(3,1.5,2.5)=" + Arrays.toString(r.doubles(3, 1.5, 2.5).toArray()));
        System.out.println("sum=" + new Random(7).ints(1000, 0, 10).sum());
        System.out.println("distinct=" + new Random(3).ints(0, 6).distinct().limit(6).sorted()
                .mapToObj(Integer::toString).collect(Collectors.joining(",")));
        System.out.println("nextInt(5,10)=" + new Random(11).nextInt(5, 10));
        System.out.println("nextLong(100)=" + new Random(11).nextLong(100));
        System.out.println("nextDouble(2.0)=" + new Random(11).nextDouble(2.0));

        SplittableRandom s = new SplittableRandom(99);
        System.out.println("split ints=" + Arrays.toString(s.ints(4, 0, 50).toArray()));
        SplittableRandom child = s.split();
        System.out.println("child longs=" + Arrays.toString(child.longs(2, 0, 1000).toArray()));
        System.out.println("split doubles=" + Arrays.toString(s.doubles(2).toArray()));

        try {
            new Random(1).ints(-1);
        } catch (IllegalArgumentException e) {
            System.out.println("IAE: " + e.getMessage());
        }
        try {
            new Random(1).ints(5, 10);
        } catch (IllegalArgumentException e) {
            System.out.println("IAE: " + e.getMessage());
        }
    }
}
