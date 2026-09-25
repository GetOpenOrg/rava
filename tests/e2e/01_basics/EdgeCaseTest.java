public class EdgeCaseTest {
    public static void main(String[] args) {
        // 1. Integer overflow (32-bit)
        int maxInt = Integer.MAX_VALUE;
        int overflow = maxInt + 1;
        System.out.println("int overflow: " + overflow);
        System.out.println("int min: " + Integer.MIN_VALUE);

        // 2. Long overflow (64-bit)
        long maxLong = Long.MAX_VALUE;
        long longOverflow = maxLong + 1;
        System.out.println("long overflow: " + longOverflow);

        // 3. Integer arithmetic edge cases
        System.out.println("int min / -1: " + (Integer.MIN_VALUE / -1));
        System.out.println("int min % -1: " + (Integer.MIN_VALUE % -1));

        // 4. Floating point special values
        double posInf = 1.0 / 0.0;
        double negInf = -1.0 / 0.0;
        double nan = 0.0 / 0.0;
        System.out.println("posInf: " + (posInf == Double.POSITIVE_INFINITY));
        System.out.println("negInf: " + (negInf == Double.NEGATIVE_INFINITY));
        System.out.println("isNaN: " + Double.isNaN(nan));

        // 5. Float to int conversion edge cases
        System.out.println("(int)NaN: " + (int) Double.NaN);
        System.out.println("(int)+Inf: " + (int) Double.POSITIVE_INFINITY);
        System.out.println("(int)-Inf: " + (int) Double.NEGATIVE_INFINITY);
        System.out.println("(int)3.99: " + (int) 3.99);
        System.out.println("(int)-3.99: " + (int) -3.99);

        // 6. ASCII strings
        String text = "Hello AB";
        System.out.println("str len: " + text.length());
        System.out.println("charAt: " + (int) text.charAt(6));

        // 7. Bit operations
        System.out.println("bitCount(255): " + Integer.bitCount(255));
        System.out.println("leadingZeros(1): " + Integer.numberOfLeadingZeros(1));
        System.out.println("trailingZeros(8): " + Integer.numberOfTrailingZeros(8));

        // 8. NaN comparison semantics
        double nan2 = Double.NaN;
        System.out.println("NaN==NaN: " + (nan2 == nan2));
        System.out.println("NaN!=NaN: " + (nan2 != nan2));
        System.out.println("NaN<0: " + (nan2 < 0));
        System.out.println("NaN>0: " + (nan2 > 0));
        System.out.println("NaN>=0: " + (nan2 >= 0));
        System.out.println("NaN<=0: " + (nan2 <= 0));

        // 9. String edge cases
        System.out.println("empty equals: " + "".equals(""));
        System.out.println("null str: " + String.valueOf((Object) null));
        System.out.println("compareTo: " + "abc".compareTo("abd"));

        System.out.println("Done.");
    }
}
