import java.util.Random;

public class RandomDemo {
    public static void main(String[] args) {
        // Seeded random — same seed must produce same sequence
        Random r = new Random(12345L);
        System.out.println(r.nextInt(100));
        System.out.println(r.nextInt(100));
        System.out.println(r.nextInt(100));
        System.out.println(r.nextLong() > Long.MIN_VALUE);   // always true
        System.out.println(r.nextBoolean() || !r.nextBoolean()); // always true

        // nextDouble in range [0, 1)
        double d = r.nextDouble();
        System.out.println(d >= 0.0 && d < 1.0);

        // nextInt() without bound — any int is valid
        int anyInt = r.nextInt();
        System.out.println(anyInt == anyInt); // always true

        // Reset with same seed — should reproduce the same sequence
        Random r2 = new Random(12345L);
        System.out.println(r2.nextInt(100) == new Random(12345L).nextInt(100));

        System.out.println("done");
    }
}
