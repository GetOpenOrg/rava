public class TryFinallyReturn {
    static int depth;
    static String[] args0 = new String[0];
    static Object a() { return args0.length > 5 ? null : "a"; }
    static String b() { return args0.length > 5 ? null : "b"; }
    static Object pick(int k) {
        depth++;
        try {
            switch (k) {
                case 1: return a();
                case 2: return b();
                case 3: return String.valueOf(a());
                default: throw new IllegalStateException("bad");
            }
        } finally {
            depth--;
        }
    }
    public static void main(String[] args) {
        System.out.println(pick(1));
        System.out.println(pick(2));
        System.out.println(pick(3));
    }
}
