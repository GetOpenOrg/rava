public class IntOverflow {
    public static void main(String[] args) {
        int max = Integer.MAX_VALUE;
        int wrapped = max + 1;
        System.out.println(wrapped); // JVM: -2147483648

        int min = Integer.MIN_VALUE;
        int wrapped2 = min - 1;
        System.out.println(wrapped2); // JVM: 2147483647

        long lmax = Long.MAX_VALUE;
        long lwrapped = lmax + 1L;
        System.out.println(lwrapped); // JVM: -9223372036854775808
    }
}
