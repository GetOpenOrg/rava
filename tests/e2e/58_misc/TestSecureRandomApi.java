import java.security.SecureRandom;
import java.util.HashSet;
import java.util.Set;

/**
 * K-JCA：SecureRandom（原生侧取操作系统熵源，JDK Linux 缺省 NativePRNG 同语义）。输出只含
 * 与随机值无关的不变量：nextBytes 填满且非全零、nextInt(bound) 区间、nextLong / nextDouble /
 * nextBoolean / nextGaussian 可用、两实例序列不同、setSeed 不使序列确定、generateSeed 长度、
 * getAlgorithm / getProvider、带种子构造器。
 */
public class TestSecureRandomApi {
    public static void main(String[] args) {
        SecureRandom sr = new SecureRandom();
        System.out.println(sr.getAlgorithm() + " " + sr.getProvider().getName());

        byte[] b = new byte[64];
        sr.nextBytes(b);
        boolean allZero = true;
        for (byte x : b) if (x != 0) allZero = false;
        System.out.println("nextBytes filled=" + !allZero);

        boolean inRange = true;
        Set<Integer> seen = new HashSet<>();
        for (int i = 0; i < 2000; i++) {
            int v = sr.nextInt(10);
            if (v < 0 || v >= 10) inRange = false;
            seen.add(v);
        }
        System.out.println("nextInt(10) inRange=" + inRange + " allValues=" + (seen.size() == 10));

        double d = sr.nextDouble();
        System.out.println("nextDouble in [0,1)=" + (d >= 0.0 && d < 1.0));
        long l1 = sr.nextLong(), l2 = sr.nextLong();
        System.out.println("nextLong differ=" + (l1 != l2));
        boolean t = false, f = false;
        for (int i = 0; i < 200; i++) { if (sr.nextBoolean()) t = true; else f = true; }
        System.out.println("nextBoolean both=" + (t && f));
        System.out.println("nextGaussian finite=" + Double.isFinite(sr.nextGaussian()));

        SecureRandom a = new SecureRandom(new byte[] {1, 2, 3});
        SecureRandom c = new SecureRandom(new byte[] {1, 2, 3});
        a.setSeed(42L);
        c.setSeed(42L);
        System.out.println("seeded differ=" + (a.nextLong() != c.nextLong()));

        System.out.println("generateSeed len=" + sr.generateSeed(24).length);
    }
}
