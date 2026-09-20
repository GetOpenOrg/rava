public class TestStringCodePoints {
    public static void main(String[] args) {
        String s = "A😀B";
        System.out.println("length=" + s.length());
        System.out.println("codePointCount=" + s.codePointCount(0, s.length()));
        System.out.println("charAt0=" + (int) s.charAt(0));
        System.out.println("codePointAt0=" + s.codePointAt(0));
        System.out.println("codePointAt1=" + s.codePointAt(1));
        int hi = s.codePointAt(1);
        System.out.println("isHigh=" + Character.isHighSurrogate((char) hi));
        System.out.println("isLow=" + Character.isLowSurrogate((char) s.charAt(2)));
        System.out.println("offset=" + s.offsetByCodePoints(0, 1));
        System.out.println("asciiLen=" + "hello".codePointCount(0, 5));
        System.out.println("isSurrogatePair=" + Character.isSurrogatePair(s.charAt(1), s.charAt(2)));
    }
}
