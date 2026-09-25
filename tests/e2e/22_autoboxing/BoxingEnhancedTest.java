public class BoxingEnhancedTest {
    public static void main(String[] args) {
        // Integer constants
        System.out.println("Integer.MAX_VALUE: " + Integer.MAX_VALUE);
        System.out.println("Integer.MIN_VALUE: " + Integer.MIN_VALUE);
        System.out.println("Integer.SIZE: " + Integer.SIZE);
        System.out.println("Integer.BYTES: " + Integer.BYTES);

        // Integer static methods
        System.out.println("compare(3,5): " + Integer.compare(3, 5));
        System.out.println("compare(5,3): " + Integer.compare(5, 3));
        System.out.println("compare(3,3): " + Integer.compare(3, 3));
        System.out.println("max(3,7): " + Integer.max(3, 7));
        System.out.println("min(3,7): " + Integer.min(3, 7));
        System.out.println("sum(3,7): " + Integer.sum(3, 7));
        System.out.println("signum(-5): " + Integer.signum(-5));
        System.out.println("signum(0): " + Integer.signum(0));
        System.out.println("signum(5): " + Integer.signum(5));

        // Integer conversion
        System.out.println("toBinaryString(255): " + Integer.toBinaryString(255));
        System.out.println("toHexString(255): " + Integer.toHexString(255));
        System.out.println("toOctalString(255): " + Integer.toOctalString(255));

        // Integer bit operations
        System.out.println("leadingZeros(1): " + Integer.numberOfLeadingZeros(1));
        System.out.println("trailingZeros(16): " + Integer.numberOfTrailingZeros(16));
        System.out.println("bitCount(255): " + Integer.bitCount(255));
        System.out.println("reverseBytes: " + Integer.reverseBytes(0x12345678));

        // Long constants
        System.out.println("Long.MAX_VALUE: " + Long.MAX_VALUE);
        System.out.println("Long.MIN_VALUE: " + Long.MIN_VALUE);
        System.out.println("Long.SIZE: " + Long.SIZE);
        System.out.println("Long.BYTES: " + Long.BYTES);

        // Long static methods
        System.out.println("Long.compare: " + Long.compare(100L, 200L));
        System.out.println("Long.max: " + Long.max(100L, 200L));
        System.out.println("Long.min: " + Long.min(100L, 200L));
        System.out.println("Long.sum: " + Long.sum(100L, 200L));
        System.out.println("Long.toBinaryString: " + Long.toBinaryString(255L));
        System.out.println("Long.toHexString: " + Long.toHexString(255L));
        System.out.println("Long.bitCount: " + Long.bitCount(255L));

        // Double methods
        System.out.println("Double.isNaN(NaN): " + Double.isNaN(Double.NaN));
        System.out.println("Double.isNaN(1.0): " + Double.isNaN(1.0));
        System.out.println("Double.isInfinite: " + Double.isInfinite(Double.POSITIVE_INFINITY));
        System.out.println("Double.compare: " + Double.compare(1.0, 2.0));
        System.out.println("Double.max: " + Double.max(1.5, 2.5));
        System.out.println("Double.min: " + Double.min(1.5, 2.5));
        System.out.println("Double.sum: " + Double.sum(1.5, 2.5));

        // Float methods
        System.out.println("Float.isNaN: " + Float.isNaN(Float.NaN));
        float f = Float.parseFloat("3.14");
        System.out.println("parseFloat: " + f);
        System.out.println("Float.compare: " + Float.compare(1.0f, 2.0f));
        System.out.println("Float.max: " + Float.max(1.5f, 2.5f));

        // Boolean methods
        System.out.println("Boolean.compare: " + Boolean.compare(true, false));
        boolean bp = Boolean.parseBoolean("true");
        System.out.println("parseBoolean: " + bp);
        boolean bp2 = Boolean.parseBoolean("false");
        System.out.println("parseBoolean2: " + bp2);
        System.out.println("Boolean.hashCode true: " + Boolean.hashCode(true));
        System.out.println("Boolean.hashCode false: " + Boolean.hashCode(false));
        System.out.println("Boolean.toString: " + Boolean.toString(true));
    }
}
