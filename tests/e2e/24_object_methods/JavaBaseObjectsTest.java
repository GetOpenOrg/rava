import java.util.Objects;

public class JavaBaseObjectsTest {
    public static void main(String[] args) {
        // Objects.equals
        System.out.println("equals null,null: " + Objects.equals(null, null));
        System.out.println("equals hello,hello: " + Objects.equals("hello", "hello"));
        System.out.println("equals hello,world: " + Objects.equals("hello", "world"));
        System.out.println("equals null,hello: " + Objects.equals(null, "hello"));
        System.out.println("equals hello,null: " + Objects.equals("hello", null));

        // Objects.hashCode
        System.out.println("hashCode null: " + Objects.hashCode(null));

        // Objects.toString
        System.out.println("toString hello: " + Objects.toString("hello"));
        System.out.println("toString null: " + Objects.toString(null));
        System.out.println("toString null,default: " + Objects.toString(null, "default"));

        // Objects.isNull / nonNull
        System.out.println("isNull null: " + Objects.isNull(null));
        System.out.println("isNull hello: " + Objects.isNull("hello"));
        System.out.println("nonNull null: " + Objects.nonNull(null));
        System.out.println("nonNull hello: " + Objects.nonNull("hello"));

        // Objects.requireNonNull
        String s = Objects.requireNonNull("test");
        System.out.println("requireNonNull: " + s);

        // Objects.requireNonNull with message (exception case)
        try {
            Objects.requireNonNull(null, "must not be null");
        } catch (NullPointerException e) {
            System.out.println("caught NPE: " + e.getMessage());
        }
    }
}
