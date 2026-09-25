import java.lang.reflect.Method;

public class ReflectionAPI {
    public int value;

    public ReflectionAPI(int v) {
        this.value = v;
    }

    public String hello() {
        return "hello";
    }

    public static void main(String[] args) {
        ReflectionAPI obj = new ReflectionAPI(42);

        // Test getClass() basic methods
        Class<?> cls = obj.getClass();
        System.out.println(cls.getName());
        System.out.println(cls.getSimpleName());

        // getDeclaredMethods() returns Method[] — call but don't use result
        // (tests that the call doesn't throw)
        Method[] methods = cls.getDeclaredMethods();

        System.out.println("reflection ok");
    }
}
