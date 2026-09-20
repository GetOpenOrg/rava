public class TestStringNewMethods {
    public static void main(String[] args) {
        String s = "  hello  \n world \t ";
        System.out.println("stripLeading=[" + s.stripLeading() + "]");
        System.out.println("stripTrailing=[" + s.stripTrailing() + "]");
        String tb = "line1\nline2\nline3";
        System.out.println("lines=" + tb.lines().count());
        System.out.println("linesContent=" + tb.lines().reduce("", (a, b) -> a + ">" + b));
        System.out.println("transform=" + "42".transform(Integer::parseInt).intValue() * 2);
        System.out.println("chars=" + "abc".chars().count());
        System.out.println("codePoints=" + "abc".codePoints().count());
        System.out.println("isBlank=" + "   ".isBlank());
        System.out.println("trimIndent=[" + """
            hello
            world
            """.stripIndent() + "]");
        System.out.println("stripIndent=[" + """
            hello
              world
            """.stripIndent() + "]");
    }
}
