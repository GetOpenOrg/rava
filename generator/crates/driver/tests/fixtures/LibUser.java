import greet.Greeter;

/** `--lib` 夹具的消费方 */
public class LibUser {
    public static void main(String[] args) {
        System.out.println(new Greeter().greet("rava"));
    }
}
