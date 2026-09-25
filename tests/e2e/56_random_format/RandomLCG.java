import java.util.Random;
public class RandomLCG {
    public static void main(String[] args) {
        // Fixed seed -- sequence must match JDK LCG exactly
        Random r = new Random(42);
        System.out.println(r.nextInt());
        System.out.println(r.nextInt());
        System.out.println(r.nextInt());
        System.out.println(r.nextInt());
        System.out.println(r.nextInt());

        // nextInt(bound) -- bounded variant
        Random r2 = new Random(42);
        System.out.println(r2.nextInt(100));
        System.out.println(r2.nextInt(100));
        System.out.println(r2.nextInt(100));

        // nextDouble -- must be in [0, 1)
        Random r3 = new Random(100);
        double d = r3.nextDouble();
        System.out.println(d >= 0.0 && d < 1.0);

        // nextBoolean
        Random r4 = new Random(1);
        System.out.println(r4.nextBoolean());

        System.out.println("Done");
    }
}
