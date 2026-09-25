public class NumberDemo {
    public static void main(String[] args) {
        // Number hierarchy: Integer / Long / Double / Float as Number
        Number n1 = Integer.valueOf(42);
        Number n2 = Long.valueOf(1000000000000L);
        Number n3 = Double.valueOf(3.14);
        Number n4 = Float.valueOf(2.71f);

        System.out.println(n1.intValue());
        System.out.println(n1.longValue());
        System.out.println(n1.doubleValue());

        System.out.println(n2.longValue());
        System.out.println(n2.intValue());   // truncation

        System.out.println(n3.doubleValue());
        System.out.println(n3.intValue());   // truncation

        System.out.println(n4.floatValue());
        System.out.println(n4.intValue());   // truncation

        // Integer static methods
        System.out.println(Integer.MAX_VALUE);
        System.out.println(Integer.MIN_VALUE);
        System.out.println(Integer.parseInt("123"));
        System.out.println(Integer.toBinaryString(10));
        System.out.println(Integer.toHexString(255));
        System.out.println(Integer.toOctalString(8));
        System.out.println(Integer.bitCount(7));
        System.out.println(Integer.reverse(1));

        // Long static methods
        System.out.println(Long.MAX_VALUE);
        System.out.println(Long.parseLong("9876543210"));
        System.out.println(Long.toBinaryString(15L));

        // Double static methods
        System.out.println(Double.MAX_VALUE > 0);
        System.out.println(Double.isNaN(Double.NaN));
        System.out.println(Double.isInfinite(1.0 / 0.0));
        System.out.println(Double.parseDouble("3.14"));

        // Float
        System.out.println(Float.isNaN(Float.NaN));
        System.out.println(Float.parseFloat("2.5"));

        System.out.println("done");
    }
}
