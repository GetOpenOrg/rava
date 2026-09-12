import java.util.ArrayList;
import java.util.List;

public class HelloWorld {

    private String message;

    public HelloWorld(String message) {
        this.message = message;
    }

    public void greet() {
        System.out.println("Hello, " + message);
    }

    public static String repeat(String s, int times) {
        StringBuilder sb = new StringBuilder();
        for (int i = 0; i < times; i++) {
            sb.append(s);
        }
        return sb.toString();
    }

    public static void main(String[] args) {
        HelloWorld hw = new HelloWorld("World");
        hw.greet();

        String r = repeat("ha", 3);
        System.out.println(r);

        List<String> items = new ArrayList<>();
        items.add("foo");
        items.add("bar");
        System.out.println(items.size());
    }
}
