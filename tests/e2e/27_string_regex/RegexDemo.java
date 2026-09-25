import java.util.regex.Pattern;
import java.util.regex.Matcher;

public class RegexDemo {
    public static void main(String[] args) {
        // basic matches
        System.out.println(Pattern.matches("\\d+", "12345"));
        System.out.println(Pattern.matches("\\d+", "abc"));

        // find all numbers in string
        Pattern p = Pattern.compile("\\d+");
        Matcher m = p.matcher("abc 123 def 456 ghi 789");
        while (m.find()) {
            System.out.println("found: " + m.group());
        }

        // groups
        Pattern date = Pattern.compile("(\\d{4})-(\\d{2})-(\\d{2})");
        Matcher dm = date.matcher("Today is 2024-03-15 and tomorrow is 2024-03-16");
        while (dm.find()) {
            System.out.println("date=" + dm.group(0) + " year=" + dm.group(1) + " month=" + dm.group(2) + " day=" + dm.group(3));
        }

        // replaceAll
        String result = p.matcher("hello 123 world 456").replaceAll("NUM");
        System.out.println(result);

        // split
        String[] parts = Pattern.compile(",\\s*").split("apple, banana, cherry, date");
        for (String part : parts) {
            System.out.println(part);
        }

        // named groups
        Pattern named = Pattern.compile("(?<year>\\d{4})-(?<month>\\d{2})");
        Matcher nm = named.matcher("2024-03");
        if (nm.matches()) {
            System.out.println("year=" + nm.group("year") + " month=" + nm.group("month"));
        }
    }
}
