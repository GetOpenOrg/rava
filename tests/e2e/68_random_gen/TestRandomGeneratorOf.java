import java.util.random.RandomGenerator;
import java.util.random.RandomGeneratorFactory;

/**
 * jdk.random 新一代算法：RandomGeneratorFactory.of(name).create(seed) 固定序列
 * 与 SplittableGenerator.split（jmod 覆盖计划 A 档；@RandomGeneratorProperties 注解
 * 读取 + 工厂反射构造，序列逐值可比）。
 */
public class TestRandomGeneratorOf {

    public static void main(String[] args) {
        for (String name : new String[] { "L64X128MixRandom", "Xoshiro256PlusPlus", "L32X64MixRandom" }) {
            RandomGenerator g = RandomGeneratorFactory.of(name).create(42L);
            System.out.println(name + " " + g.nextLong() + " " + g.nextLong() + " " + g.nextInt());
        }

        RandomGenerator g2 = RandomGeneratorFactory.of("L64X128MixRandom").create(42L);
        if (g2 instanceof RandomGenerator.SplittableGenerator sg) {
            RandomGenerator child = sg.split();
            System.out.println("split-parent=" + g2.nextLong());
            System.out.println("split-child=" + child.nextLong());
        }
        if (g2 instanceof RandomGenerator.JumpableGenerator jg) {
            jg.jump();
            System.out.println("jump=" + g2.nextLong());
        }

        // 同种子重建 → 序列一致
        RandomGenerator a = RandomGeneratorFactory.of("L32X64MixRandom").create(7L);
        RandomGenerator b = RandomGeneratorFactory.of("L32X64MixRandom").create(7L);
        System.out.println("same-seed=" + (a.nextLong() == b.nextLong()));
    }
}
