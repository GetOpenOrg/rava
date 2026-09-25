public class StringApiTest {
    public static void main(String[] args) {
        String s = "Hello, World!";

        // charAt
        System.out.println("charAt(0): " + s.charAt(0));

        // indexOf / contains
        System.out.println("indexOf World: " + s.indexOf("World"));
        System.out.println("contains World: " + s.contains("World"));

        // substring
        System.out.println("substring(7): " + s.substring(7));
        System.out.println("substring(0,5): " + s.substring(0, 5));

        // replace
        System.out.println("replace: " + s.replace("World", "Java"));

        // split
        String csv = "a,b,c,d";
        String[] parts = csv.split(",");
        System.out.println("split length: " + parts.length);
        System.out.println("split[0]: " + parts[0]);

        // startsWith / endsWith
        System.out.println("startsWith: " + s.startsWith("Hello"));
        System.out.println("endsWith: " + s.endsWith("!"));

        // compareTo
        System.out.println("compareTo: " + "abc".compareTo("abd"));

        // Integer.parseInt / Long.parseLong / Double.parseDouble
        int i = Integer.parseInt("42");
        long l = Long.parseLong("123456789");
        double d = Double.parseDouble("3.14");
        System.out.println("parseInt: " + i);
        System.out.println("parseLong: " + l);
        System.out.println("parseDouble: " + d);

        // Character
        System.out.println("isDigit: " + Character.isDigit('5'));
        System.out.println("isLetter: " + Character.isLetter('A'));
        System.out.println("toUpperCase: " + (char) Character.toUpperCase('a'));

        // StringBuilder
        StringBuilder sb = new StringBuilder();
        sb.append("Hello");
        sb.append(" ");
        sb.append("World");
        sb.reverse();
        System.out.println("reverse: " + sb.toString());
    }
}
