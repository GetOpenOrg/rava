import java.util.regex.*;

public class RegexMatcherDemo {
    public static void main(String[] args) {
        // Basic find
        Pattern p = Pattern.compile("\\d+");
        Matcher m = p.matcher("abc 123 def 456");
        while (m.find()) {
            System.out.println(m.group());
        }

        // matches() vs find()
        System.out.println("12345".matches("\\d+"));
        System.out.println("abc".matches("\\d+"));
        System.out.println(Pattern.compile("hello").matcher("say hello world").find());

        // replaceAll / replaceFirst
        String result = "baaac".replaceAll("a+", "X");
        System.out.println(result);
        String first = "aababab".replaceFirst("ab", "Z");
        System.out.println(first);

        // groups
        Pattern namePattern = Pattern.compile("(\\w+)@(\\w+)");
        Matcher nm = namePattern.matcher("user@example");
        if (nm.find()) {
            System.out.println(nm.group(1));
            System.out.println(nm.group(2));
        }

        // split
        String[] parts = "one,two,three".split(",");
        for (String s : parts) {
            System.out.println(s);
        }

        // start / end
        Matcher sm = Pattern.compile("\\d+").matcher("abc 99 xyz");
        if (sm.find()) {
            System.out.println(sm.start());
            System.out.println(sm.end());
        }

        System.out.println("done");
    }
}
