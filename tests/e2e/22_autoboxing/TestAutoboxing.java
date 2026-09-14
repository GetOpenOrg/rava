public class TestAutoboxing {

    public static void main(String[] args) {
        // autoboxing: primitive → wrapper
        Integer i = 42;
        Double d = 3.14;
        Boolean b = true;
        Long l = 100L;

        // unboxing: wrapper → primitive
        int pi = i;
        double pd = d;
        boolean pb = b;
        long pl = l;

        System.out.println(pi);   // 42
        System.out.println(pd);   // 3.14
        System.out.println(pb);   // true
        System.out.println(pl);   // 100

        // arithmetic with autoboxing
        Integer x = 10;
        Integer y = 20;
        int sum = x + y;          // unbox both
        System.out.println(sum);  // 30

        Integer product = x * y;  // unbox, multiply, rebox
        System.out.println(product); // 200

        // Integer cache: -128 to 127 are interned
        Integer a = 100;
        Integer a2 = 100;
        System.out.println(a == a2);      // true (cached)
        System.out.println(a.equals(a2)); // true

        // comparison
        Integer big = 200;
        Integer big2 = 200;
        System.out.println(big.equals(big2)); // true

        // null unboxing check (with explicit null guard)
        Integer nullable = null;
        System.out.println(nullable == null); // true

        // toString
        System.out.println(Integer.toString(255));       // 255
        System.out.println(Integer.toBinaryString(10));  // 1010
        System.out.println(Integer.toHexString(255));    // ff

        // parseInt
        int parsed = Integer.parseInt("123");
        System.out.println(parsed); // 123

        // MIN/MAX
        System.out.println(Integer.MIN_VALUE); // -2147483648
        System.out.println(Integer.MAX_VALUE); // 2147483647
    }
}
