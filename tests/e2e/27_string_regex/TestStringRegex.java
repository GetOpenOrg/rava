import java.util.regex.Pattern;
import java.util.regex.Matcher;

public class TestStringRegex {

    public static void main(String[] args) {
        // String.matches
        System.out.println("hello123".matches("[a-z]+\\d+"));  // true
        System.out.println("hello".matches("[a-z]+\\d+"));     // false
        System.out.println("123".matches("\\d+"));             // true

        // String.replaceAll
        String s = "foo bar  baz   qux";
        System.out.println(s.replaceAll("\\s+", " ").trim()); // foo bar baz qux

        String digits = "a1b2c3d4";
        System.out.println(digits.replaceAll("[a-z]", ""));   // 1234
        System.out.println(digits.replaceAll("\\d", ""));     // abcd

        // String.split
        String csv = "one,two,three,four";
        String[] parts = csv.split(",");
        for (String p : parts) System.out.println(p);
        // one, two, three, four

        // split with limit
        String[] limited = csv.split(",", 2);
        System.out.println(limited.length); // 2
        System.out.println(limited[0]);     // one
        System.out.println(limited[1]);     // two,three,four

        // Pattern + Matcher
        Pattern email = Pattern.compile("[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\\.[a-zA-Z]{2,}");
        Matcher m = email.matcher("contact: user@example.com and info@test.org");
        int count = 0;
        while (m.find()) {
            System.out.println(m.group());
            count++;
        }
        System.out.println("found: " + count);  // found: 2

        // groups
        Pattern datePattern = Pattern.compile("(\\d{4})-(\\d{2})-(\\d{2})");
        Matcher dm = datePattern.matcher("Today is 2026-09-15.");
        if (dm.find()) {
            System.out.println(dm.group(1)); // 2026
            System.out.println(dm.group(2)); // 09
            System.out.println(dm.group(3)); // 15
        }

        // replaceFirst
        String str = "apple banana apple cherry";
        System.out.println(str.replaceFirst("apple", "mango")); // mango banana apple cherry
    }
}
