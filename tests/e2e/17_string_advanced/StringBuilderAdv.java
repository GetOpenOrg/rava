public class StringBuilderAdv {
    public static void main(String[] args) {
        // insert
        StringBuilder sb = new StringBuilder("Hello World");
        sb.insert(5, ",");
        System.out.println(sb);

        // delete / deleteCharAt
        StringBuilder sb2 = new StringBuilder("abcdefg");
        sb2.delete(2, 5);
        System.out.println(sb2);
        sb2.deleteCharAt(0);
        System.out.println(sb2);

        // replace
        StringBuilder sb3 = new StringBuilder("Hello World");
        sb3.replace(6, 11, "Java");
        System.out.println(sb3);

        // reverse
        StringBuilder sb4 = new StringBuilder("abcde");
        sb4.reverse();
        System.out.println(sb4);

        // indexOf
        StringBuilder sb5 = new StringBuilder("abcabc");
        System.out.println(sb5.indexOf("bc"));
        System.out.println(sb5.indexOf("bc", 2));

        // substring
        StringBuilder sb6 = new StringBuilder("Hello World");
        System.out.println(sb6.substring(6));
        System.out.println(sb6.substring(0, 5));

        // charAt / length
        StringBuilder sb7 = new StringBuilder("test");
        System.out.println(sb7.charAt(0));
        System.out.println(sb7.length());

        // chaining
        StringBuilder chain = new StringBuilder()
            .append("foo")
            .append(42)
            .append(true)
            .append('!');
        System.out.println(chain);

        // capacity / ensureCapacity
        StringBuilder sb8 = new StringBuilder(4);
        System.out.println(sb8.capacity() >= 4);
        sb8.ensureCapacity(100);
        System.out.println(sb8.capacity() >= 100);

        System.out.println("done");
    }
}
