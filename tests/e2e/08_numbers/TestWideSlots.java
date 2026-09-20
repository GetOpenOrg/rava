public class TestWideSlots {

    // long 形参后紧跟 int 形参：后者 slot 索引 +2
    static long sumLong(long a, int b, long c, int d) {
        return a + b + c + d;
    }

    // double 同理
    static double sumDouble(double a, int b, double c, short d) {
        return a + b + c + d;
    }

    // long / double 交错
    static double mixed(long a, double b, int c, long d, double e) {
        return a + b + c + d + e;
    }

    static int count;

    // 局部变量侧的 long/double 双槽占用
    static void locals() {
        int i = 1;
        long l = 2L;
        int j = 3;
        double d = 4.5;
        int k = 5;
        System.out.println("locals: " + i + " " + l + " " + j + " " + d + " " + k);
        long l2 = l * 10;
        double d2 = d * 2;
        System.out.println("locals2: " + l2 + " " + d2);
    }

    // 多参数且含 long/double 的重载
    static int pick(int a, int b) { return 1; }
    static int pick(long a, int b) { return 2; }
    static int pick(double a, int b) { return 3; }
    static int pick(int a, long b) { return 4; }

    public static void main(String[] args) {
        System.out.println(sumLong(1L, 2, 3L, 4));
        System.out.println(sumLong(Long.MAX_VALUE / 2, 7, 0L, 9));
        System.out.println(sumDouble(1.5, 2, 3.25, (short) 4));
        System.out.println(mixed(1L, 2.5, 3, 4L, 5.5));

        locals();

        System.out.println("pick(int,int)=" + pick(1, 2));
        System.out.println("pick(long,int)=" + pick(1L, 2));
        System.out.println("pick(double,int)=" + pick(1.5, 2));
        System.out.println("pick(int,long)=" + pick(1, 2L));

        // long/double 数组元素与循环变量混排
        long[] ls = {10L, 20L, 30L};
        double[] ds = {1.5, 2.5, 3.5};
        int[] is = {1, 2, 3};
        for (int t = 0; t < 3; t++) {
            System.out.println(ls[t] + " " + ds[t] + " " + is[t]);
        }

        // 三元返回 long / double 混合
        boolean flag = true;
        long r = flag ? 100000000000L : 5L;
        System.out.println("r=" + r);
        System.out.println("r+1=" + (r + 1));

        System.out.println("done");
    }
}
