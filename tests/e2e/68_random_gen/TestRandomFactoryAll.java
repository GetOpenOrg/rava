import java.util.random.RandomGeneratorFactory;

/**
 * jdk.random：RandomGeneratorFactory.all() 算法名录（排序后打印）与工厂元数据
 * （jmod 覆盖计划 A 档；名录来自模块内实现类扫描，与语料 JDK 同源）。
 */
public class TestRandomFactoryAll {

    public static void main(String[] args) {
        RandomGeneratorFactory.all()
                .map(f -> f.name())
                .sorted()
                .forEach(n -> System.out.println("alg=" + n));

        RandomGeneratorFactory.all()
                .filter(f -> f.name().equals("L64X128MixRandom"))
                .findFirst()
                .ifPresent(f -> System.out.println(
                        "lxm-group=" + f.group() + " bits=" + f.stateBits() + " equidist="
                                + f.equidistribution()));
    }
}
