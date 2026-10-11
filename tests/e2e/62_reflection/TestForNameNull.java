// Class.forName(null)：JDK 抛 NullPointerException（不是 ClassNotFoundException）
public class TestForNameNull {
    public static void main(String[] args) throws Exception {
        try {
            Class.forName(null);
            System.out.println("no exception");
        } catch (ClassNotFoundException e) {
            System.out.println("ClassNotFoundException: " + e.getMessage());
        } catch (NullPointerException e) {
            System.out.println("NullPointerException");
        }
        try {
            Class.forName(null, false, TestForNameNull.class.getClassLoader());
            System.out.println("no exception");
        } catch (NullPointerException e) {
            System.out.println("NullPointerException (3-arg)");
        }
        System.out.println(Class.forName("java.lang.String").getName());
    }
}
