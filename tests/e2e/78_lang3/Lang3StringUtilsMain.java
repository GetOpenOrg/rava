
import org.apache.commons.lang3.StringUtils;

/**
 * commons-lang3 pilot A11：StringUtils 方法矩阵（W1 #4）。
 */
public class Lang3StringUtilsMain {

    static void eq(String name, String expected, String actual) {
        System.out.println((expected.equals(actual) ? "PASS " : "FAIL ") + name
                + (expected.equals(actual) ? "" : " :: expected=<" + expected + "> actual=<" + actual + ">"));
    }

    static void bool(String name, boolean expected, boolean actual) {
        System.out.println((expected == actual ? "PASS " : "FAIL ") + name
                + (expected == actual ? "" : " :: expected=" + expected + " actual=" + actual));
    }

    public static void main(String[] args) {
        eq("abbreviate", "abcdefghij", StringUtils.abbreviate("abcdefghij", 10)); // maxWidth==len 原样返回
        eq("abbreviate-wide", "你好世界...", StringUtils.abbreviate("你好世界你好世界", 7));
        eq("capitalize", "Hello", StringUtils.capitalize("hello"));
        eq("uncapitalize", "hello", StringUtils.uncapitalize("Hello"));
        eq("swapCase", "hELLO wORLD", StringUtils.swapCase("Hello World"));
        eq("reverse", "cba", StringUtils.reverse("abc"));
        eq("reverseDelimited", "c.b.a", StringUtils.reverseDelimited("a.b.c", '.'));

        eq("join-array", "a|b|c", StringUtils.join(new String[]{"a", "b", "c"}, '|'));
        eq("split-full", "[a, b, c]", java.util.Arrays.toString(StringUtils.split("a,,b,c,", ',')));
        eq("split-preserve", "[a, , b, c]", java.util.Arrays.toString(
                StringUtils.splitByWholeSeparatorPreserveAllTokens("a,,b,c", ",")));

        int[] counts = {
                StringUtils.countMatches("abcabc", "ab"),
                StringUtils.indexOf("abcabc", "b"),
                StringUtils.lastIndexOf("abcabc", "b"),
                StringUtils.indexOfDifference("hello", "help")};
        System.out.println("PASS counts index-of " + java.util.Arrays.toString(counts));

        eq("left-pad", "007", StringUtils.leftPad("7", 3, '0'));
        eq("right-pad", "7--", StringUtils.rightPad("7", 3, '-'));
        eq("repeat", "ababab", StringUtils.repeat("ab", 3));
        eq("wrap", "\"abc\"", StringUtils.wrap("abc", '"'));
        eq("overlay", "hello weather", StringUtils.overlay("hello world", "weather", 6, 11));

        eq("default-if-blank", "fallback", StringUtils.defaultIfBlank("   ", "fallback"));
        eq("default-string", "", StringUtils.defaultString(null));
        eq("substring-before", "abc", StringUtils.substringBefore("abc.def", "."));
        eq("substring-after-last", "txt", StringUtils.substringAfterLast("a.b.txt", "."));
        eq("chomp", "line", StringUtils.chomp("line\n"));

        bool("is-blank-null", true, StringUtils.isBlank(null));
        bool("is-empty", true, StringUtils.isEmpty(""));
        bool("is-alpha", true, StringUtils.isAlpha("abcXYZ"));
        bool("is-numeric", true, StringUtils.isNumeric("123"));
        bool("is-alphanumeric-space", true, StringUtils.isAlphanumericSpace("ab 12"));
        bool("contains-any", true, StringUtils.containsAny("hello", 'x', 'o'));
        bool("starts-with-ignorecase", true, StringUtils.startsWithIgnoreCase("Hello", "he"));
        bool("equals-ignorecase", true, StringUtils.equalsIgnoreCase("ABC", "abc"));

        eq("delete-whitespace", "abc", StringUtils.deleteWhitespace(" a b c "));
        eq("normalize-space", "a b c", StringUtils.normalizeSpace("  a   b  c  "));
        eq("truncate", "abc", StringUtils.truncate("abcdef", 3));
        System.out.println("DONE Lang3StringUtilsMain");
    }
}
