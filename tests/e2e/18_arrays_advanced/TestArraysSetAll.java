import java.util.Arrays;

/**
 * Arrays.setAll 族（方法级实测：7 jar，此前零覆盖）：
 * 对象/原始数组按下标生成、setAll 与 parallelSetAll 结果一致（值仅依赖下标）。
 */
public class TestArraysSetAll {

    public static void main(String[] args) {
        String[] names = new String[4];
        Arrays.setAll(names, i -> "n" + i);
        System.out.println("obj=" + String.join(",", names));

        int[] ints = new int[5];
        Arrays.setAll(ints, i -> i * i);
        System.out.println("ints=" + Arrays.toString(ints));

        long[] longs = new long[3];
        Arrays.setAll(longs, i -> 10L << i);
        System.out.println("longs=" + Arrays.toString(longs));

        double[] ds = new double[3];
        Arrays.setAll(ds, i -> i / 2.0);
        System.out.println("doubles=" + Arrays.toString(ds));

        // parallelSetAll：值仅依赖下标 → 与串行结果逐元素一致
        int[] par = new int[5];
        Arrays.parallelSetAll(par, i -> i * i);
        System.out.println("parallel-eq=" + Arrays.equals(par, ints));

        // 空数组形态
        Arrays.setAll(new int[0], i -> i);
        System.out.println("empty-ok=true");

        // 值仅依赖下标的重算幂等性
        Arrays.setAll(ints, i -> ints[i] + 1);
        System.out.println("rebased=" + Arrays.toString(ints));
    }
}
