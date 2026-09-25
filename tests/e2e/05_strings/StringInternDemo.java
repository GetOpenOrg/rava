public class StringInternDemo {
    public static void main(String[] args) {
        // intern() returns a canonical string
        String s1 = new String("hello").intern();
        String s2 = new String("hello").intern();
        // Both interned strings are == (same pool reference)
        System.out.println(s1 == s2);
        System.out.println(s1.equals(s2));

        // String literals are already interned
        String lit = "hello";
        System.out.println(lit == s1);

        // Different strings have different pool entries
        String a = "foo".intern();
        String b = "bar".intern();
        System.out.println(a.equals(b));
        System.out.println(a.equals("foo"));

        // concat then intern
        String c = ("he" + "llo").intern();
        System.out.println(c.equals("hello"));

        // intern of empty string
        String e1 = "".intern();
        String e2 = "".intern();
        System.out.println(e1 == e2);
        System.out.println(e1.isEmpty());

        System.out.println("done");
    }
}
