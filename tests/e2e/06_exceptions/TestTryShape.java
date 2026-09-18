public class TestTryShape {
    static int g(int x) { if (x > 1) throw new RuntimeException("g"); return x; }
    static int f(int x) {
        try {
            return g(x);
        } catch (RuntimeException e) {
            System.out.println("c " + e.getMessage());
        }
        System.out.println("after");
        return -1;
    }
    static int h(int x) {
        for (int i = 0; i < 3; i++) {
            try {
                if (i == x) throw new IllegalStateException("h" + i);
                g(i + x);
            } catch (IllegalStateException e) {
                System.out.println("ise " + e.getMessage());
                continue;
            } catch (RuntimeException e) {
                System.out.println("rt " + e.getMessage());
            }
            System.out.println("tail " + i);
        }
        return x;
    }
    public static void main(String[] args) {
        System.out.println(f(0));
        System.out.println(f(5));
        System.out.println(h(1));
    }
}
