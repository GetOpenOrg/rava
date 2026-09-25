public class DoubleInstance {
    public static void main(String[] args) {
        Double nan = Double.NaN;
        Double posInf = Double.POSITIVE_INFINITY;
        Double regular = 3.14;
        System.out.println(nan.isNaN());
        System.out.println(posInf.isNaN());
        System.out.println(regular.isNaN());
        System.out.println(posInf.isInfinite());
        System.out.println(regular.isInfinite());
        Float fnan = Float.NaN;
        System.out.println(fnan.isNaN());
        Float fInf = Float.POSITIVE_INFINITY;
        System.out.println(fInf.isInfinite());
    }
}
