import java.util.Objects;
public class ObjectsDemo {
    public static void main(String[] args) {
        String s = "hello";
        System.out.println(Objects.isNull(null));   // true
        System.out.println(Objects.nonNull(s));     // true
        System.out.println(Objects.toString(s));    // hello
    }
}
