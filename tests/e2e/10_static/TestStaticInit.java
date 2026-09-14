public class TestStaticInit {
    static int value;
    static String name;

    static {
        value = 10 * 5;
        name = "Static" + "Init";
        System.out.println("static block ran");
    }

    public static void main(String[] args) {
        System.out.println(value);
        System.out.println(name);
    }
}
