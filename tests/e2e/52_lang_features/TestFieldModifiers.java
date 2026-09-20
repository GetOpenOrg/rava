public class TestFieldModifiers {
    private volatile int counter = 0;
    private transient String secret = "x";
    public static final long MASK = 0xFFFFL;

    public static void main(String[] args) {
        TestFieldModifiers t = new TestFieldModifiers();
        t.counter = 3;
        System.out.println("counter=" + t.counter);
        System.out.println("secret=" + t.secret);
        System.out.println("mask=" + Long.toHexString(MASK));
        System.out.println("ok");
    }
}
