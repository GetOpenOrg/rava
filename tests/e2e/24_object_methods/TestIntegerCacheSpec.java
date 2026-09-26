// FS-H0：Integer.valueOf / Integer.toString() 改走字节码翻译（IntegerCache.<clinit>）后的规格行为。
public class TestIntegerCacheSpec {
    static Object box(int v) { return v; }

    public static void main(String[] args) {
        int[] probes = {-129, -128, -1, 0, 1, 127, 128, 1000, Integer.MIN_VALUE, Integer.MAX_VALUE};
        for (int v : probes) {
            Integer a = Integer.valueOf(v), b = Integer.valueOf(v);
            Object c = box(v);
            System.out.println(v + " same=" + (a == b) + " autobox=" + (a == c) + " eq=" + a.equals(c)
                + " str=" + a.toString() + " hash=" + a.hashCode());
        }
        Integer parsed = Integer.valueOf("42");
        System.out.println("parsed cached " + (parsed == Integer.valueOf(42)) + " radix " + Integer.valueOf("-ff", 16));
        Integer big = Integer.valueOf("500");
        System.out.println("parsed big " + (big == Integer.valueOf(500)) + " " + big.equals(500));
        StringBuilder sb = new StringBuilder();
        for (int i = -3; i <= 3; i++) sb.append(Integer.valueOf(i * 50)).append(',');
        System.out.println(sb);
        System.out.println("hex " + Integer.toString(255, 16) + " bin " + Integer.toBinaryString(10) + " neg " + Integer.toString(-2147483648));
    }
}
