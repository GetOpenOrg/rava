public class ObjectMethods {
    public static void main(String[] args) {
        Object a = new Object();
        Object b = a;
        // Reference equality: same object
        System.out.println(a.equals(b));
        // Reference equality: different objects
        System.out.println(a.equals(new Object()));
        // hashCode always returns some int (the expression is always true)
        int h = a.hashCode();
        System.out.println(h >= 0 || h < 0);
        // String equals compares by value
        String s = "hello";
        System.out.println(s.equals("hello"));
    }
}
