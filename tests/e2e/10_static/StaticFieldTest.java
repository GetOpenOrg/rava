public class StaticFieldTest {
    static int counter = 0;

    public static void increment() {
        counter = counter + 1;
    }

    public static void main(String[] args) {
        System.out.println(counter);
        increment();
        increment();
        increment();
        System.out.println(counter);
        counter = 100;
        System.out.println(counter);
    }
}
