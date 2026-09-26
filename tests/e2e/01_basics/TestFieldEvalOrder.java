// 字段读写求值顺序：JVM 操作数栈先压入旧值，再执行赋值（JLS §15.7.1 从左到右求值）。
// 覆盖 getfield/putfield 与 getstatic/putstatic 在同一实参列表中的交错。
public class TestFieldEvalOrder {
    static int sx = 10;
    int from = 3;
    long lf = 100L;

    static final class Pair {
        final long a, b;
        Pair(long a, long b) { this.a = a; this.b = b; }
        public String toString() { return "(" + a + "," + b + ")"; }
    }

    static String pair(int a, int b) { return "(" + a + "," + b + ")"; }

    Pair split() {
        return new Pair(lf, lf = lf + 50);
    }

    public static void main(String[] args) {
        TestFieldEvalOrder t = new TestFieldEvalOrder();
        System.out.println("field: " + pair(t.from, t.from = t.from + 5) + " now=" + t.from);
        System.out.println("static: " + pair(sx, sx = sx * 2) + " now=" + sx);
        System.out.println("split: " + t.split() + " now=" + t.lf);
        int x = t.from + (t.from = 1) + t.from;
        System.out.println("sum: " + x + " now=" + t.from);
        int y = sx++ + sx + (sx += 3) + sx;
        System.out.println("static sum: " + y + " now=" + sx);
    }
}
