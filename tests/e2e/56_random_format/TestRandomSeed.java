import java.util.*;

public class TestRandomSeed {
    public static void main(String[] args) {
        Random r = new Random(42);
        System.out.println("r0=" + r.nextInt(100));
        System.out.println("r1=" + r.nextInt(100));
        System.out.println("r2=" + r.nextInt(100));
        SplittableRandom sr = new SplittableRandom(7);
        System.out.println("sr=" + sr.nextInt(50));
        System.out.println("sr2=" + sr.nextInt(50));
        Random r2 = new Random(42);
        long seq = 0;
        for (int i = 0; i < 3; i++) seq = seq * 31 + r2.nextInt(100);
        System.out.println("seq=" + seq);
    }
}
