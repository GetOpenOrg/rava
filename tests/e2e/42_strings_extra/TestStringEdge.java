public class TestStringEdge {
    public static void main(String[] args) {
        System.out.println("nullConcat=" + "x" + null + "|" + null + "y");
        String s = new String("hello");
        System.out.println("internSame=" + (s.intern() == "hello"));
        System.out.println("region=" + "helloWORLD".regionMatches(true, 0, "HELLO", 0, 5));
        System.out.println("contentEq=" + "abc".contentEquals(new StringBuilder("abc")));
        System.out.println("sub=" + "hello".substring(1, 4));
        System.out.println("valueOfInt=" + String.valueOf(123));
        System.out.println("valueOfBool=" + String.valueOf(true));
        System.out.println("valueOfCharArr=" + String.valueOf(new char[]{'a', 'b'}));
        System.out.println("getBytesLen=" + "abc".getBytes(java.nio.charset.StandardCharsets.UTF_8).length);
        System.out.println("repeat=" + "ab".repeat(3));
        System.out.println("compareTo=" + "apple".compareTo("apply"));
    }
}
