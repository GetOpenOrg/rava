public class java_base_e2e_reflect_invoke {

    static class Greeter {
        public String hello() {
            return "Hello from Greeter";
        }

        public String getValue() {
            return "42";
        }
    }

    public static void main(String[] args) throws Exception {
        Greeter g = new Greeter();

        // getDeclaredMethod + invoke
        Class<?> cls = g.getClass();
        java.lang.reflect.Method m1 = cls.getDeclaredMethod("hello");
        Object result1 = m1.invoke(g, null);
        System.out.println(result1);

        java.lang.reflect.Method m2 = cls.getDeclaredMethod("getValue");
        Object result2 = m2.invoke(g, null);
        System.out.println(result2);
    }
}
