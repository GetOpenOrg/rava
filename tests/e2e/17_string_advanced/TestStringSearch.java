import java.util.Arrays;

public class TestStringSearch {

    public static void main(String[] args) {
        String s = "Hello, World! Hello Again";

        // indexOf / lastIndexOf
        System.out.println("indexOf Hello=" + s.indexOf("Hello"));
        System.out.println("lastIndexOf Hello=" + s.lastIndexOf("Hello"));
        System.out.println("indexOf Hello from 2=" + s.indexOf("Hello", 2));
        System.out.println("indexOf missing=" + s.indexOf("xyz"));
        System.out.println("indexOf('H')=" + s.indexOf('H'));
        System.out.println("indexOf('H',1)=" + s.indexOf('H', 1));
        System.out.println("lastIndexOf('o')=" + s.lastIndexOf('o'));

        // charAt / length
        System.out.println("charAt(0)=" + s.charAt(0));
        System.out.println("charAt(7)=" + s.charAt(7));
        System.out.println("length=" + s.length());

        // substring
        System.out.println("substring(7,12)=" + s.substring(7, 12));
        System.out.println("substring(7)=" + s.substring(7));
        System.out.println("subSequence=" + s.subSequence(0, 5));

        // split（含 limit 重载）
        String csv = "a,b,c,d";
        String[] parts = csv.split(",");
        System.out.println("split len=" + parts.length + " " + Arrays.toString(parts));
        String[] limited = csv.split(",", 2);
        System.out.println("split limit2 len=" + limited.length + " " + Arrays.toString(limited));
        String[] tricky = "a..b".split("\\.");
        System.out.println("regex split len=" + tricky.length + " " + Arrays.toString(tricky));

        // replace / replaceAll / replaceFirst
        System.out.println("replace Hello->Bye=" + s.replace("Hello", "Bye"));
        System.out.println("replaceChar l->L=" + "hello".replace('l', 'L'));
        System.out.println("replaceAll digits=" + "a1b2c3".replaceAll("\\d", "#"));
        System.out.println("replaceFirst digit=" + "a1b2c3".replaceFirst("\\d", "#"));

        // toCharArray / valueOf
        char[] chars = "abc".toCharArray();
        System.out.println("chars len=" + chars.length + " first=" + chars[0]);
        System.out.println("new String(char[])=" + new String(chars));
        System.out.println("valueOf(int)=" + String.valueOf(42));
        System.out.println("valueOf(char[])=" + String.valueOf(new char[]{'x', 'y'}));
        System.out.println("valueOf(bool)=" + String.valueOf(true));
        System.out.println("valueOf(double)=" + String.valueOf(1.5));
        System.out.println("valueOf(obj)=" + String.valueOf(new StringBuilder("sbVal")));
        System.out.println("valueOf(null)=" + String.valueOf((Object) null));

        // concat / join
        System.out.println("concat=" + "ab".concat("cd"));
        System.out.println("join=" + String.join("-", "a", "b", "c"));

        // matches / contains 与正则
        System.out.println("matches \\d+=" + "12345".matches("\\d+"));
        System.out.println("matches [A-Z]+=" + "ABC".matches("[A-Z]+"));

        // indexOf 越界行为
        try {
            s.charAt(1000);
        } catch (StringIndexOutOfBoundsException e) {
            System.out.println("charAt oob -> " + e.getClass().getSimpleName());
        }
        try {
            s.substring(20, 5);
        } catch (StringIndexOutOfBoundsException e) {
            System.out.println("substring reversed -> " + e.getClass().getSimpleName());
        }

        // 逐字符遍历
        StringBuilder codes = new StringBuilder();
        for (int i = 0; i < "ABC".length(); i++) {
            codes.append((int) "ABC".charAt(i)).append(" ");
        }
        System.out.println("codes=" + codes.toString().trim());

        // String.codePointAt / chars simplicity: 用 charAt 表達
        System.out.println("done");
    }
}
