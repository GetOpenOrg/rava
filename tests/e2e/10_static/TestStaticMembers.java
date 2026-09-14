public class TestStaticMembers {
    static int counter = 0;
    static final String PREFIX = "Item";

    static int increment() {
        return ++counter;
    }

    static String format(int n) {
        return PREFIX + "-" + n;
    }

    public static void main(String[] args) {
        System.out.println(counter);
        increment();
        increment();
        increment();
        System.out.println(counter);
        System.out.println(format(42));
        System.out.println(format(counter));
    }
}
