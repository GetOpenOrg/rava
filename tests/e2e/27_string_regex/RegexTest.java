import java.util.regex.Pattern;
import java.util.regex.Matcher;

public class RegexTest {
    public static void main(String[] args) {
        // Pattern.compile + matcher
        Pattern p = Pattern.compile("[0-9]+");
        Matcher m = p.matcher("abc123def456ghi");

        // find() loop
        System.out.println("--- find loop ---");
        while (m.find()) {
            System.out.println("found: " + m.group() + " at " + m.start() + "-" + m.end());
        }

        // matches() — full string match
        Pattern p2 = Pattern.compile("[a-z]+");
        Matcher m2 = p2.matcher("hello");
        System.out.println("matches hello: " + m2.matches());

        Matcher m3 = p2.matcher("hello123");
        System.out.println("matches hello123: " + m3.matches());

        // Pattern.matches static
        System.out.println("static matches: " + Pattern.matches("[0-9]+", "42"));
        System.out.println("static no match: " + Pattern.matches("[0-9]+", "abc"));

        // Capture groups
        Pattern p3 = Pattern.compile("(\\w+)@(\\w+)");
        Matcher m4 = p3.matcher("user@host");
        if (m4.find()) {
            System.out.println("group0: " + m4.group());
            System.out.println("group1: " + m4.group(1));
            System.out.println("group2: " + m4.group(2));
            System.out.println("groupCount: " + m4.groupCount());
        }

        // replaceAll / replaceFirst
        Pattern p4 = Pattern.compile("\\d+");
        Matcher m5 = p4.matcher("a1b2c3");
        System.out.println("replaceAll: " + m5.replaceAll("X"));
        System.out.println("replaceFirst: " + m5.replaceFirst("Y"));

        // split
        Pattern p5 = Pattern.compile(",");
        String[] parts = {"apple", "banana", "cherry"};
        // Use pattern directly
        System.out.println("pattern: " + p5.pattern());

        // toString
        System.out.println("toString: " + p5);
    }
}
